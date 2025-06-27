<template>
  <q-dialog
    :model-value="isOpen"
    maximized
    transition-show="slide-up"
    transition-hide="slide-down"
    class="mod-details-modal"
    @update:model-value="$emit('update:isOpen', $event)"
  >
    <q-card class="mod-details-card">
      <q-linear-progress
        v-if="loading"
        indeterminate
        color="primary"
      ></q-linear-progress>
      <q-card-section class="row items-center q-pb-sm">
        <div
          v-if="!loading"
          class="text-h6 phantom-font-difficulty header-text"
        >
          <img :src="modInfo?._aCategory._sIconUrl" class="mod-details-icon" />
          {{ modInfo?._sName }}
        </div>
        <div
          v-else-if="error"
          class="text-h6 phantom-font-difficulty header-text"
        >
          <q-icon
            name="error"
            size="40px"
            color="negative"
            class="mod-details-icon"
          />
          {{ error }}
        </div>
        <div v-else class="text-h6 phantom-font-difficulty header-text">
          <q-spinner size="40px" color="primary" class="mod-details-icon" />
          {{ $t('ui.loading') }}... {{ $t('ui.loading') }}...
        </div>
        <q-space />
        <q-btn
          flat
          round
          dense
          icon="close"
          class="close-btn"
          @click="closeModal"
        />
      </q-card-section>
      <q-scroll-area class="mod-details-scroll-area">
        <q-card-section v-if="loading" class="flex flex-center">
          <div v-if="isSlowRequest" class="text-center phantom-font">
            <q-icon name="hourglass_empty" size="2em" color="orange" />
            <div class="q-mt-sm" style="color: var(--theme-text-secondary)">
              {{ $t('app.modals.mod_details.slow_request') }}
              <br />
              {{ $t('app.modals.mod_details.slow_request_description') }}
            </div>
          </div>
        </q-card-section>

        <q-card-section v-else-if="error" class="text-center text-negative">
          <q-icon name="error" size="2em" />
          <div class="q-mt-sm phantom-font">{{ error }}</div>
        </q-card-section>

        <template v-else-if="modInfo">
          <q-card-section class="mod-details-content">
            <div class="mod-details-left col">
              <q-carousel
                v-if="
                  modInfo?._aPreviewMedia?._aImages &&
                  modInfo._aPreviewMedia._aImages.length >= 1
                "
                v-model="currentSlide"
                animated
                autoplay
                infinite
                navigation
                swipeable
                class="mod-carousel"
                height="400px"
              >
                <q-carousel-slide
                  v-for="(image, index) in modInfo._aPreviewMedia._aImages"
                  :key="image._sFile"
                  :name="index"
                  class="flex"
                >
                  <img
                    :src="image._sBaseUrl + '/' + image._sFile"
                    class="carousel-image"
                    alt="Mod preview image"
                  />
                </q-carousel-slide>
              </q-carousel>
              <div v-if="modInfo._iCompletionPercentage" class="q-mt-md">
                <q-linear-progress
                  :value="modInfo._iCompletionPercentage / 100"
                  color="primary"
                  class="q-mb-sm"
                  rounded
                />
                <div class="text-subtitle1 phantom-font">
                  <span style="color: var(--theme-text-secondary)">{{
                    modInfo._sDevelopmentState
                  }}</span>
                  <span style="color: var(--theme-text-secondary)"> - </span>
                  {{ modInfo._iCompletionPercentage }}% complete<span
                    v-if="modInfo._iCompletionPercentage < 75"
                    >.</span
                  >
                  <span
                    v-else-if="
                      modInfo._iCompletionPercentage > 75 &&
                      modInfo._iCompletionPercentage < 90
                    "
                    >!</span
                  ><span v-else-if="modInfo._iCompletionPercentage >= 90"
                    >!!</span
                  ><span v-else-if="modInfo._iCompletionPercentage === 100"
                    >!!!</span
                  >
                </div>
              </div>
              <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                <div class="flex">
                  {{ $t('app.modals.mod_details.description') }}
                  <q-space />
                  <div
                    class="text-subtitle1 phantom-font text-right"
                    style="color: var(--theme-text-secondary)"
                  >
                    {{ modInfo._sDescription }}
                  </div>
                </div>
                <hr />
              </h6>
              <div>
                <div
                  class="phantom-font mod-info-text"
                  v-html="processHtmlAnchors(modInfo._sText)"
                ></div>
              </div>
              <div v-if="modInfo._nUpdatesCount && modInfo._nUpdatesCount > 0">
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  <div class="flex">
                    {{ $t('app.modals.mod_details.updates') }}
                    {{ $t('app.modals.mod_details.updates') }}
                    <q-space />
                    <div
                      class="text-subtitle1 phantom-font text-right"
                      style="color: var(--theme-text-secondary)"
                    >
                      {{
                        $t('app.modals.mod_details.updates_count', {
                          count: modInfo._nUpdatesCount,
                        })
                      }}
                    </div>
                  </div>
                  <hr />
                </h6>
                <div v-if="modUpdates">
                  <div
                    v-for="update in modUpdates._aRecords"
                    :key="update._idRow || update._sName"
                  >
                    <q-expansion-item
                      dense
                      class="phantom-font"
                      group="updates"
                      expand-icon-class="dropdown-icon"
                    >
                      <template #header>
                        <div class="flex column">
                          <div class="flex row">
                            <div class="text-subtitle1">
                              {{ update._sName }}
                            </div>
                            <div
                              class="q-ml-xs flex row items-cente custom-badge-small"
                            >
                              <q-icon name="add" size="xs" />
                              {{ formatDate(update._tsDateAdded) }}
                            </div>
                          </div>
                          <div class="">
                            <div
                              v-if="update._sVersion"
                              class="phantom-font"
                              style="color: var(--theme-text-secondary)"
                            >
                              {{ update._sVersion }}
                            </div>
                            <div
                              v-if="
                                update._aChangeLog &&
                                update._aChangeLog.length > 0
                              "
                              class="flex row badge-container"
                            >
                              <div
                                v-for="(count, category) in groupChanges(
                                  update._aChangeLog
                                )"
                                :key="category"
                                class="phantom-font"
                                style="color: var(--theme-text-secondary)"
                              >
                                <q-badge
                                  :style="{
                                    backgroundColor: getCategoryColor(category),
                                  }"
                                  >{{ category }}
                                  <span
                                    v-if="count > 1"
                                    style="
                                      color: lightgray;
                                      margin-left: 0.25rem;
                                    "
                                    >{{ count }}</span
                                  ></q-badge
                                >
                              </div>
                            </div>
                          </div>
                        </div>
                        <q-space />
                      </template>
                      <div class="q-mt-sm">
                        <div class="changelog q-ml-sm q-mb-sm">
                          <div
                            v-for="(change, changeIndex) in update._aChangeLog"
                            :key="`${change.cat}-${changeIndex}`"
                            class="phantom-font flex row items-center q-mb-xs"
                          >
                            <q-chip
                              :clickable="false"
                              :ripple="false"
                              :style="{
                                backgroundColor: getCategoryColor(change.cat),
                                color: 'white',
                              }"
                              >{{ change.cat }}</q-chip
                            >
                            <div>{{ change.text }}</div>
                          </div>
                        </div>
                        <div
                          class="phantom-font update-text"
                          v-html="processHtmlAnchors(update._sText)"
                        ></div>
                      </div>
                    </q-expansion-item>
                  </div>
                </div>
              </div>
              <div v-if="modComments && modComments._aRecords.length > 0">
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  <div class="flex">
                    {{ $t('app.modals.mod_details.comments') }}
                    {{ $t('app.modals.mod_details.comments') }}
                    <q-space />
                    <div
                      class="text-subtitle1 phantom-font text-right"
                      style="color: var(--theme-text-secondary)"
                    >
                      {{
                        $t('app.modals.mod_details.comments_count', {
                          count: modComments._aMetadata._nRecordCount,
                        })
                      }}
                      {{
                        $t('app.modals.mod_details.comments_count', {
                          count: modComments._aMetadata._nRecordCount,
                        })
                      }}
                    </div>
                  </div>
                  <hr />
                </h6>
                <!-- Valid comments with posters -->
                <div v-for="comment in validComments" :key="comment._idRow">
                  <div class="flex column">
                    <div class="flex row items-center phantom-font">
                      <img
                        :src="
                          comment._aPoster._sAvatarUrl
                            ? comment._aPoster._sAvatarUrl
                            : 'https://images.gamebanana.com/static/img/defaults/avatar.gif'
                        "
                        class="mod-details-icon"
                      />
                      <div
                        class="text-subtitle1"
                        style="color: var(--theme-text-secondary)"
                        :style="
                          comment._aPoster._sProfileUrl
                            ? { cursor: 'pointer' }
                            : {}
                        "
                        @click="
                          openUrl(
                            comment._aPoster._sProfileUrl
                              ? comment._aPoster._sProfileUrl
                              : ''
                          )
                        "
                      >
                        <img
                          v-if="comment._aPoster._sUpicUrl"
                          :src="comment._aPoster._sUpicUrl"
                        />
                        <span v-else>{{ comment._aPoster._sName }}</span>
                      </div>
                      <div class="flex row items-center q-ml-xs">
                        <span v-if="comment._aLabels">
                          <q-badge
                            v-for="(label, index) in comment._aLabels"
                            :key="index"
                            class="q-ml-xs"
                            color="primary"
                            outline
                            >{{ label }}</q-badge
                          ></span
                        >
                        <div
                          class="flex row items-center custom-badge-small q-ml-xs"
                        >
                          <q-icon
                            v-if="comment._iPinLevel > 0"
                            name="push_pin"
                            size="xs"
                            class="q-ml-xs BlueColor"
                          />
                          <q-icon name="add" size="xs" />
                          <span>
                            {{ formatDate(comment._tsDateAdded) }}
                          </span>
                        </div>
                      </div>
                    </div>
                    <div class="q-mt-sm">
                      <div
                        class="phantom-font comment-text"
                        v-html="processHtmlAnchors(comment._sText)"
                      ></div>
                      <div v-if="comment._aPoster._sSigUrl">
                        <img
                          :src="comment._aPoster._sSigUrl"
                          :style="
                            comment._aPoster._sProfileUrl
                              ? { cursor: 'pointer' }
                              : {}
                          "
                          @click="
                            openUrl(
                              comment._aPoster._sProfileUrl
                                ? comment._aPoster._sProfileUrl
                                : ''
                            )
                          "
                        />
                      </div>
                      <i
                        v-else
                        class="phantom-font"
                        style="color: var(--theme-text-secondary)"
                      >
                        {{ comment._aPoster._sUserTitle }}
                      </i>
                    </div>
                    <div v-if="comment._nReplyCount > 0">
                      <div
                        class="phantom-font text-right"
                        style="color: var(--theme-text-secondary)"
                      >
                        {{
                          $t('app.modals.mod_details.replies_count', {
                            count: comment._nReplyCount,
                          })
                        }}
                        {{
                          $t('app.modals.mod_details.replies_count', {
                            count: comment._nReplyCount,
                          })
                        }}
                      </div>
                    </div>
                  </div>
                  <hr style="border-top-style: dashed" />
                </div>
                <!-- Trashed comments (without posters) -->
                <div v-for="comment in trashedComments" :key="comment._idRow">
                  <div>
                    <span
                      class="phantom-font"
                      style="color: var(--theme-text-secondary)"
                      >{{ $t('app.modals.mod_details.comment_trashed') }}</span
                    >
                  </div>
                  <hr style="border-top-style: dashed" />
                </div>
              </div>
            </div>
            <div class="mod-details-right phantom-font col">
              <div
                v-if="modInfo._nThanksCount"
                class="q-mb-md flex justify-center items-center"
                style="color: var(--theme-text-secondary)"
              >
                <q-icon
                  name="favorite"
                  size="2em"
                  class="q-mr-xs"
                  color="pink"
                />
                <div>
                  <i18n-t keypath="app.modals.mod_details.thanked_by">
                    <template #count>
                      <b style="color: var(--pink)">{{
                        modInfo._nThanksCount
                      }}</b>
                    </template>
                  </i18n-t>
                </div>
              </div>
              <div class="mod-badges">
                <div
                  v-if="modInfo._nLikeCount && modInfo._nLikeCount > 0"
                  class="custom-badge"
                >
                  <q-icon name="thumb_up" class="q-mr-xs" />
                  {{ formatNumber(modInfo._nLikeCount) }}
                </div>
                <div
                  v-if="modInfo._nDownloadCount && modInfo._nDownloadCount > 0"
                  class="custom-badge"
                >
                  <q-icon name="download" class="q-mr-xs" />
                  {{ formatNumber(modInfo._nDownloadCount) }}
                </div>
                <div
                  v-if="modInfo._nViewCount && modInfo._nViewCount > 0"
                  class="custom-badge"
                >
                  <q-icon name="visibility" class="q-mr-xs" />
                  {{ formatNumber(modInfo._nViewCount) }}
                </div>
                <div
                  v-if="modInfo._nPostCount && modInfo._nPostCount > 0"
                  class="custom-badge"
                >
                  <q-icon name="comment" class="q-mr-xs" />
                  {{ formatNumber(modInfo._nPostCount) }}
                </div>
                <div v-if="modInfo._tsDateAdded > 0" class="custom-badge">
                  <q-icon name="add" class="q-mr-xs" />
                  {{ formatDate(modInfo._tsDateAdded) }}
                </div>
                <div v-if="modInfo._tsDateModified > 0" class="custom-badge">
                  <q-icon name="edit" class="q-mr-xs" />
                  {{ formatDate(modInfo._tsDateModified) }}
                </div>
                <div
                  v-if="modInfo._tsDateUpdated && modInfo._tsDateUpdated > 0"
                  class="custom-badge"
                >
                  <q-icon name="update" class="q-mr-xs" />
                  {{ formatDate(modInfo._tsDateUpdated) }}
                </div>
              </div>
              <div class="q-mt-md">
                <q-btn
                  color="primary"
                  :label="$t('ui.actions.download')"
                  size="lg"
                  class="action-button"
                  :disabled="!modInfo._aFiles || !modInfo._aFiles.length"
                  @click="downloadMod"
                />
                <div
                  v-if="modInfo._aFiles && modInfo._aFiles.length > 0"
                  class="q-mt-sm"
                >
                  <q-badge
                    v-if="modInfo._aFiles[0]._sAvastAvResult === 'clean'"
                    label="AVAST"
                  >
                    <q-icon name="check" size="xs" class="q-ml-xs" />
                  </q-badge>
                  <q-badge v-else label="AVAST" color="negative">
                    <q-icon name="warning" size="xs" class="q-ml-xs" />
                    <q-tooltip class="phantom-font text-center">
                      {{ $t('app.modals.mod_details.might_have_malware') }}
                      {{ $t('app.modals.mod_details.might_have_malware') }}
                    </q-tooltip>
                  </q-badge>
                </div>

                <q-btn
                  icon="launch"
                  :label="$t('ui.actions.open_in_browser')"
                  class="full-width q-mt-sm"
                  outline
                  @click="openUrl(modInfo._sProfileUrl)"
                />
              </div>
              <div v-if="modInfo._aStudio" class="q-mt-md">
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  Studio
                  <hr />
                </h6>
                <div class="flex justify-center">
                  <div>Created by</div>
                  <div @onclick="openUrl(modInfo._aStudio._sProfileUrl)">
                    <img :src="modInfo._aStudio._sBannerUrl" />
                  </div>
                </div>
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  {{ $t('app.modals.mod_details.submitter') }}
                  <hr />
                </h6>
                <div class="flex">
                  <img
                    :src="modInfo._aSubmitter._sAvatarUrl"
                    class="mod-details-icon"
                  />
                  <div
                    class="text-subtitle1 phantom-font text-right"
                    style="color: var(--theme-text-secondary)"
                    :style="
                      modInfo._aSubmitter._sProfileUrl
                        ? { cursor: 'pointer' }
                        : {}
                    "
                    @click="openUrl(modInfo._aSubmitter._sProfileUrl)"
                  >
                    {{ modInfo._aSubmitter._sName }}
                  </div>
                </div>
              </div>
              <div v-if="hasCredits" class="q-mt-md">
                <div class="credits-section">
                  <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                    {{ $t('misc.credits') }}
                    <hr />
                  </h6>
                  <div class="credits-groups">
                    <div
                      v-for="(group, index) in modInfo._aCredits"
                      :key="index"
                      class="credits-group"
                    >
                      <h6 class="credits-title">{{ group._sGroupName }}</h6>
                      <div class="credits-list">
                        <div
                          v-for="(author, authorIndex) in group._aAuthors"
                          :key="authorIndex"
                          class="credits-item"
                        >
                          <div class="credits-info">
                            <div
                              v-if="author._sUpicUrl"
                              :style="
                                author._sProfileUrl || author._sUrl
                                  ? { cursor: 'pointer' }
                                  : {}
                              "
                              @click="
                                openUrl(
                                  author._sProfileUrl
                                    ? author._sProfileUrl
                                    : author._sUrl
                                      ? author._sUrl
                                      : ''
                                )
                              "
                            >
                              <img :src="author._sUpicUrl" />
                            </div>
                            <div
                              v-else
                              class="credits-name"
                              :style="
                                author._sProfileUrl || author._sUrl
                                  ? { cursor: 'pointer' }
                                  : {}
                              "
                              @click="
                                openUrl(
                                  author._sProfileUrl
                                    ? author._sProfileUrl
                                    : author._sUrl
                                      ? author._sUrl
                                      : ''
                                )
                              "
                            >
                              {{ author._sName }}
                              <q-icon
                                v-if="author._sProfileUrl"
                                name="arrow_outward"
                              />
                            </div>
                            <div v-if="author._sRole" class="credits-role">
                              {{ author._sRole }}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div v-if="modInfo._sDevNotes" class="q-mt-md">
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  {{ $t('app.modals.mod_details.developer_notes') }}
                  <hr />
                </h6>
                <div
                  class="phantom-font mod-info-text"
                  v-html="processHtmlAnchors(modInfo._sDevNotes)"
                ></div>
              </div>
              <div v-if="modInfo._sLicense" class="q-mt-md">
                <h6 class="text-h6 phantom-font-difficulty q-mb-md q-mt-md">
                  {{ $t('app.modals.mod_details.license') }}
                  <hr />
                </h6>
                <div
                  class="phantom-font mod-info-text"
                  v-html="processHtmlAnchors(modInfo._sLicense)"
                ></div>
              </div>
            </div>
          </q-card-section>
        </template>
      </q-scroll-area>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import {
  ref,
  watch,
  onMounted,
  onUnmounted,
  defineEmits,
  defineProps,
  computed,
} from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'
import { processHtmlAnchors } from '@utils/index'
import {
  GBChangeLogEntry,
  GBModPosts,
  GBModPost,
  GBModUpdates,
  GBProfilePage,
} from '@custom-types/gamebanana'

const props = defineProps({
  modId: {
    type: Number,
    required: true,
  },
  modelType: {
    type: String,
    default: 'Mod',
  },
  isOpen: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:isOpen', 'download'])

const modInfo = ref<GBProfilePage | null>(null)
const modUpdates = ref<GBModUpdates | null>(null)
const modComments = ref<GBModPosts | null>(null)
const loading = ref(true)
const error = ref('')
const currentSlide = ref(0)
const isSlowRequest = ref(false)

// Determine if there are credits to display
const hasCredits = computed(() => {
  return (
    modInfo.value?._aCredits &&
    Array.isArray(modInfo.value._aCredits) &&
    modInfo.value._aCredits.length > 0
  )
})

// Type guard to check if a comment has a poster
function hasValidPoster(
  comment: GBModPosts['_aRecords'][number]
): comment is GBModPost {
  return (
    comment &&
    '_aPoster' in comment &&
    typeof comment._aPoster === 'object' &&
    comment._aPoster !== null
  )
}

// Computed property to get valid comments with posters
const validComments = computed(() => {
  if (!modComments.value?._aRecords) return []
  return modComments.value._aRecords.filter(hasValidPoster)
})

// Computed property to get trashed comments (without posters)
const trashedComments = computed(() => {
  if (!modComments.value?._aRecords) return []
  return modComments.value._aRecords.filter(comment => !hasValidPoster(comment))
})

// Function to clear all data from the component
function clearData() {
  modInfo.value = null
  modUpdates.value = null
  modComments.value = null
  error.value = ''
  currentSlide.value = 0
  loading.value = false // Clear loading state when closing modal
  isSlowRequest.value = false
}

// Fetch mod details when component is mounted and isOpen changes to true
watch(
  () => props.isOpen,
  async newVal => {
    if (newVal) {
      await fetchModInfo()
    } else {
      // Clear data when modal is closed
      clearData()
    }
  }
)

onMounted(async () => {
  if (props.isOpen) {
    await fetchModInfo()
  }
})

// Clear data when component is unmounted
onUnmounted(() => {
  clearData()
})

async function fetchModInfo() {
  loading.value = true
  error.value = ''
  isSlowRequest.value = false

  // Set up a timeout to show slow request message after 15 seconds
  const slowRequestTimeout = setTimeout(() => {
    if (loading.value) {
      isSlowRequest.value = true
    }
  }, 15000)

  try {
    const infoResult = await invoke<GBProfilePage>('get_mod_info_command', {
      modId: props.modId,
      modelType: props.modelType || 'Mod',
    })

    const updatesResult = await invoke<GBModUpdates>(
      'get_mod_updates_command',
      {
        modId: props.modId,
        page: 1,
        modelType: props.modelType || 'Mod',
      }
    )

    const commentsResult = await invoke<GBModPosts>('get_mod_posts_command', {
      modId: props.modId,
      page: 1,
      modelType: props.modelType || 'Mod',
    })

    if (!infoResult) {
      throw new Error('Failed to fetch mod information')
    }
    if (!updatesResult) {
      throw new Error('Failed to fetch mod updates')
    }
    if (!commentsResult) {
      throw new Error('Failed to fetch mod comments')
    }
    modInfo.value = infoResult
    modUpdates.value = updatesResult
    modComments.value = commentsResult
  } catch (err: unknown) {
    error.value = (err as Error).message
  } finally {
    clearTimeout(slowRequestTimeout)
    loading.value = false
    isSlowRequest.value = false
  }
}

// Close the modal
function closeModal() {
  emit('update:isOpen', false)
}

// Helper function to format numbers (e.g., 1000 -> 1K)
function formatNumber(num: number): string {
  if (!num) return '0'
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

// Helper function to format date as relative time
function formatDate(timestamp: number): string {
  if (!timestamp) return 'N/A'

  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000)

  // Convert to appropriate time units
  if (diffInSeconds < 60) {
    return `${diffInSeconds}s`
  } else if (diffInSeconds < 3600) {
    return `${Math.floor(diffInSeconds / 60)}m`
  } else if (diffInSeconds < 86400) {
    return `${Math.floor(diffInSeconds / 3600)}h`
  } else if (diffInSeconds < 2592000) {
    // ~30 days
    return `${Math.floor(diffInSeconds / 86400)}d`
  } else if (diffInSeconds < 31536000) {
    // ~365 days
    return `${Math.floor(diffInSeconds / 2592000)}mo`
  } else {
    return `${Math.floor(diffInSeconds / 31536000)}y`
  }
}

// Function to handle the download button click
// Function to group changelog categories and count occurrences
function groupChanges(changes: GBChangeLogEntry[]): Record<string, number> {
  const grouped: Record<string, number> = {}

  if (Array.isArray(changes)) {
    changes.forEach(change => {
      if (change.cat) {
        if (grouped[change.cat]) {
          grouped[change.cat]++
        } else {
          grouped[change.cat] = 1
        }
      }
    })
  }

  return grouped
}

// Function to determine the color for a category
function getCategoryColor(category: string): string {
  switch (category) {
    case 'Bugfix':
    case 'Overhaul':
      return 'var(--red)'
    case 'Adjustment':
    case 'Amendment':
      return 'var(--green)'
    case 'Improvement':
    case 'Optimization':
    case 'Tweak':
      return 'var(--pink)'
    case 'Addition':
    case 'Suggestion':
    case 'Feature':
      return 'var(--blue)'
    case 'Removal':
    case 'Refactor':
      return 'var(--theme-border)'
    default:
      return '' // Use default background
  }
}

function downloadMod() {
  const formattedMod = modInfo.value
  if (formattedMod) {
    emit('download', formattedMod)
  }
}
</script>

<style scoped>
.mod-details-modal {
  background: rgba(0, 0, 0, 0.7);
}

.mod-details-card {
  background: var(--theme-solid);
  width: 80vw;
  margin-top: 5vh;
  border-radius: 1rem !important;
  height: 100%;
  overflow-y: auto;
  border: 2px solid var(--theme-border);
}
.header-text {
  display: flex;
  align-items: center;
  flex-direction: row;
}
.mod-details-icon {
  width: 40px;
  height: 40px;
  margin-right: 10px;
  object-fit: contain;
}
.mod-details-content {
  display: flex;
  flex-direction: row;
  padding: 16px;
  max-width: 100%;
}

.mod-details-left {
  flex: 1;
  padding-right: 16px;
}

.mod-details-right {
  flex: 1;
  padding-left: 16px;
  min-width: 200px;
  max-width: 20%;
}

.mod-badges {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}
.custom-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  border-radius: 0.5rem;
  background-color: var(--theme-surface);
  color: var(--theme-text-secondary);
  border: 2px solid var(--theme-border);
  font-size: 1.25rem;
}

.custom-badge-small {
  padding: 4px 8px;
  border-radius: 0.5rem;
  color: var(--theme-text-secondary);
  border: 2px solid var(--theme-border);
  font-size: 0.75rem;
}

.mod-carousel {
  max-width: 800px;
  max-height: 100%;
  height: 20vh;
  width: fit-content;
  margin: 0 auto;
  border-radius: 8px;
  overflow: hidden;
  border: 2px solid var(--theme-border);
  background: transparent;
}

.action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  border: 2px solid var(--theme-border);
  width: 100%;
}
.action-button-secondary {
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  width: 100%;
  margin-top: 8px;
}

.q-carousel__slide {
  padding: 0;
}

.carousel-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: cover;
}

.mod-details {
  max-width: 800px;
  margin: 0 auto;
  padding: 0 16px;
}

.mod-details-scroll-area {
  height: calc(95vh - 50px);
}

/* Credits section styles */
.credits-section {
  padding: 10px 0;
}

.credits-groups {
  margin-top: 10px;
}

.credits-group {
  margin-bottom: 15px;
}

.credits-title {
  color: var(--theme-primary);
  font-weight: bold;
  margin: 5px 0;
  font-size: 1rem;
}

.credits-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 8px 0;
}

.credits-item {
  display: flex;
  align-items: center;
  padding: 0 0.9rem;
}

.credits-icon {
  width: 32px;
  height: 32px;
  overflow: hidden;
  flex-shrink: 0;
  margin-right: 0.5rem;
  border-radius: 50%;
}

.credits-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.credits-info {
  display: flex;
  flex-direction: column;
}

.credits-name {
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
}

.credits-role {
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
  opacity: 0.85;
  line-height: 1.2;
}

.changelog {
  border-left: 2px solid var(--theme-border);
  padding-left: 1rem;
}

.badge-container {
  gap: 0.5rem;
}
</style>

<!-- Styles for v-html content (not scoped) -->
<style>
.mod-info-text,
.update-textm .comment-text {
  line-height: 1.2;
  text-wrap: break-chars;
  max-width: fit-content;
}

.mod-info-text img,
.update-text img,
.comment-text img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.mod-info-text a,
.update-text a,
.comment-text a {
  color: var(--q-primary);
  text-decoration: none;
}

.mod-info-text a:hover,
.update-text a:hover,
.comment-text a:hover {
  text-decoration: underline;
}

.mod-info-text p,
.update-text p,
.comment-text p {
  margin-bottom: 1rem;
}

.mod-info-text h1,
.mod-info-text h2,
.mod-info-text h3,
.mod-info-text h4,
.mod-info-text h5,
.mod-info-text h6,
.update-text h1,
.update-text h2,
.update-text h3,
.update-text h4,
.update-text h5,
.update-text h6,
.comment-text h1,
.comment-text h2,
.comment-text h3,
.comment-text h4,
.comment-text h5,
.comment-text h6 {
  margin-top: 1.5rem;
  margin-bottom: 1rem;
  font-weight: bold;
  line-height: 1.2;
}

.mod-info-text h1,
.update-text h1,
.comment-text h1 {
  font-size: 1.5rem;
}

.mod-info-text h2,
.update-text h2,
.comment-text h2 {
  font-size: 1.75rem;
}

.mod-info-text h3,
.update-text h3,
.comment-text h3 {
  font-size: 1.5rem;
}

.mod-info-text h4,
.update-text h4,
.comment-text h4 {
  font-size: 1.25rem;
}

.mod-info-text h5,
.update-text h5,
.comment-text h5 {
  font-size: 1.125rem;
}

.mod-info-text h6,
.update-text h6,
.comment-text h6 {
  font-size: 1rem;
}

.mod-info-text ul,
.mod-info-text ol,
.update-text ul,
.update-text ol,
.comment-text ul,
.comment-text ol {
  margin-left: 2rem;
  margin-bottom: 1rem;
}

.mod-info-text li,
.update-text li,
.comment-text li {
  margin-bottom: 0.5rem;
}

.mod-info-text iframe,
.update-text iframe,
.comment-text iframe {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border: 2px solid var(--theme-border);
}

.mod-info-text pre,
.update-text pre,
.comment-text pre {
  background-color: var(--theme-card);
  border: 1px solid var(--theme-border);
  padding: 0.5rem;
  border-radius: 0.2rem;
  font-family: monospace;
  width: fit-content;
}

.BlueColor {
  color: var(--blue);
}
.RedColor {
  color: var(--red);
}
.GreenColor {
  color: var(--green);
}
.PurpleColor {
  color: var(--purple);
}
.OrangeColor {
  color: var(--orange);
}
.YellowColor {
  color: var(--yellow);
}
.PinkColor {
  color: var(--pink);
}
:deep(.dropdown-icon) {
  color: var(--theme-text-secondary);
}
</style>
