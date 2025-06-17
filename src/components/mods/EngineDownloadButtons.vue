<template>
  <div class="engine-download">
    <h6 class="phantom-font-difficulty q-mb-md">
      Engine Downloads
      <hr />
    </h6>
    <div class="engine-download-buttons text-center">
      <q-btn
        v-for="engine in primaryEngines"
        :key="engine.engineType"
        flat
        class="engine-btn"
        style="padding-right: 8px"
        no-caps
        @click="$emit('download-engine', engine.engineType)"
      >
        <template #default>
          <div class="engine-text">
            <span>{{ engine.engineName }}</span>
          </div>
          <span v-if="engine.data.engine_version" class="engine-version">
            (v{{ engine.data.engine_version }})
          </span>
          <img
            v-if="
              engine.data.engine_icon && engineIcons[engine.data.engine_icon]
            "
            :src="engineIcons[engine.data.engine_icon]"
            :alt="engine.engineName"
            class="engine-icon"
          />
          <q-btn
            flat
            icon="info"
            round
            dense
            style="color: var(--theme-text-secondary)"
            class="q-ml-sm"
            @click.stop="openDetails(engine.engineType)"
          />
        </template>
      </q-btn>
    </div>
    <q-expansion-item
      v-if="secondaryEngines.length > 0"
      class="more-engines full-width"
      dense
      expand-icon-class="dropdown-icon"
    >
      <template #header>
        <div class="flex items-center justify-between">
          <span>More Engines</span>
        </div>
      </template>
      <div class="more-engines-content">
        <q-btn
          v-for="engine in secondaryEngines"
          :key="engine.engineType"
          flat
          style="padding-right: 8px"
          no-caps
          class="engine-btn"
          @click="$emit('download-engine', engine.engineType)"
        >
          <template #default>
            <div class="engine-text">
              <span>{{ engine.engineName }}</span>
            </div>
            <span v-if="engine.data.engine_version" class="engine-version">
              (v{{ engine.data.engine_version }})
            </span>
            <img
              v-if="
                engine.data.engine_icon && engineIcons[engine.data.engine_icon]
              "
              :src="engineIcons[engine.data.engine_icon]"
              :alt="engine.engineName"
              class="engine-icon"
            />
            <q-btn
              flat
              icon="info"
              style="color: var(--theme-text-secondary)"
              round
              dense
              class="q-ml-sm"
              @click.stop="openDetails(engine.engineType)"
            />
          </template>
        </q-btn>
      </div>
      <div>
        Have a suggestion for an engine? Ask on the
        <a
          class="cursor-pointer link"
          @click="
            handleUrlClick('https://www.github.com/echolotl/fridaylight/issues')
          "
          >GitHub</a
        >!
      </div>
    </q-expansion-item>
  </div>

  <!-- View Details Modal -->
  <MessageDialog
    :model-value="detailsModal"
    :confirm-label="'Close'"
    single-option
    disable-icon
    :persistent="false"
    title="Engine Details"
    @update:model-value="detailsModal = $event"
  >
    <template #default>
      <q-scroll-area style="max-height: 80vh; height: 400px; width: 100%">
        <div class="">
          <div class="banner-container">
            <div
              class="details-banner"
              :style="{
                backgroundImage: currentModalEngine?.engine_banner
                  ? bannerImage
                  : 'url(images/placeholder.png)',
              }"
            ></div>
            <div class="banner-overlay">
              <div class="engine-logo">
                <img
                  v-if="engineLogo"
                  :src="engineLogo"
                  :alt="currentModalEngine?.engine_name"
                  class="engine-logo-img"
                />
              </div>
              <div class="banner-version phantom-font-difficulty">
                <span v-if="currentModalEngine?.engine_version">
                  v{{ currentModalEngine.engine_version }}
                </span>
                <img
                  v-if="currentModalEngine?.engine_icon"
                  :src="engineIcons[currentModalEngine.engine_icon]"
                  alt="Engine Icon"
                  class="engine-icon"
                />
              </div>
            </div>
          </div>
          <div class="modal-main-content">
            <p v-html="processedEngineDescription"></p>
            <div
              v-if="currentModalEngine?.credits?.length"
              class="credits-content"
            >
              <h6 class="phantom-font-difficulty q-mt-md q-mb-md">
                Credits
                <hr />
              </h6>

              <q-list dense class="q-px-none">
                <q-item
                  v-for="credit in currentModalEngine?.credits"
                  :key="credit.name"
                  class="q-px-none"
                >
                  <q-item-section v-if="!credit.url">
                    <span>{{ credit.name }}</span>
                  </q-item-section>
                  <q-item-section v-else>
                    <a
                      class="cursor-pointer link"
                      @click="handleUrlClick(credit.url)"
                      >{{ credit.name }}</a
                    >
                  </q-item-section>
                  <q-item-section side>
                    <span
                      class="phantom-font"
                      style="color: var(--theme-text-secondary)"
                      >{{ credit.role }}</span
                    >
                  </q-item-section>
                </q-item>
              </q-list>
              <q-btn
                v-if="currentModalEngine?.credits_url"
                class="q-mt-md full-width"
                icon="open_in_new"
                label="View Full Credits"
                flat
                @click="handleUrlClick(currentModalEngine.credits_url)"
              >
              </q-btn>
            </div>
            <span
              v-if="currentModalEngine?.suggestors?.length"
              style="color: var(--theme-text-secondary)"
              >Suggested by:
              {{ currentModalEngine.suggestors.join(', ') }}</span
            >
          </div>
        </div>
      </q-scroll-area>
    </template>
  </MessageDialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  getAllEngineTypes,
  getEngineTypeData,
  processHtmlAnchors,
  type EngineTypeInfo,
  type EngineData,
} from '../../utils'
import { resolveResource, sep } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import MessageDialog from '@modals/MessageDialog.vue'

defineEmits(['download-engine'])

const engineTypes = ref<EngineTypeInfo[]>([])
const engineIcons = ref<Record<string, string>>({})

const detailsModal = ref(false)
const currentModalEngine = ref<EngineData | null>(null)
const bannerImage = ref('')
const engineLogo = ref('')
const primaryEngines = computed(() =>
  engineTypes.value.filter((engine: EngineTypeInfo) => engine.isPrimary)
)

const secondaryEngines = computed(() =>
  engineTypes.value.filter((engine: EngineTypeInfo) => !engine.isPrimary)
)

const processedEngineDescription = computed(() => {
  if (!currentModalEngine.value?.engine_description) return ''

  return processHtmlAnchors(currentModalEngine.value.engine_description)
})

onMounted(async () => {
  try {
    engineTypes.value = await getAllEngineTypes()
    await loadEngineIcons()
  } catch (error) {
    console.error('Failed to load engine types:', error)
  }
})

const handleUrlClick = (url: string) => {
  ;(
    window as typeof window & { handleUrlClick: (url: string) => void }
  ).handleUrlClick(url)
}

async function getIconAsBase64(iconPath: string): Promise<string> {
  try {
    const resourcePath = await resolveResource(iconPath)
    const iconData = await invoke<string>('get_file_as_base64', {
      filePath: resourcePath,
    })
    return iconData
  } catch (error) {
    console.error(`Error loading icon from ${iconPath}:`, error)
    return ''
  }
}

async function loadEngineIcons() {
  for (const engine of engineTypes.value) {
    if (engine.data.engine_icon) {
      try {
        engineIcons.value[engine.data.engine_icon] = await getIconAsBase64(
          `resources${sep()}${engine.data.engine_icon}`
        )
      } catch (error) {
        console.error(`Failed to load icon for ${engine.engineName}:`, error)
      }
    }
  }
}

async function openDetails(engine_type: string) {
  const engineData = await getEngineTypeData(engine_type)
  if (engineData) {
    currentModalEngine.value = engineData
    if (engineData.engine_banner) {
      const imageData = await getIconAsBase64(
        'resources' + sep() + engineData.engine_banner || ''
      )
      bannerImage.value = `url(${imageData})`
    } else {
      bannerImage.value = ''
    }
    if (engineData.engine_logo) {
      engineLogo.value = await getIconAsBase64(
        'resources' + sep() + engineData.engine_logo || ''
      )
    } else {
      engineLogo.value = ''
    }
    detailsModal.value = true
  } else {
    console.error(`No data found for engine type: ${engine_type}`)
  }
}
</script>

<style scoped>
.engine-download {
  margin-top: 1rem;
}
.engine-download-buttons {
  display: flex;
  margin-bottom: 16px;
  gap: 8px;
  flex-wrap: wrap;
}
.engine-btn {
  border-radius: 8px;
  padding: 8px 16px;
  max-width: fit-content;
  width: 100%;
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;
  background-color: var(--theme-card);
  border-bottom: 2px solid var(--theme-border);
  display: flex;
  flex-direction: row;
  flex-shrink: 0;
}
.engine-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}
.details-banner {
  width: 100%;
  height: 200px;
  background-size: cover;
  background-position: center;
  border-radius: 0.5rem 0.5rem 0 0;
  mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1), rgba(0, 0, 0, 0));
  margin-bottom: 16px;
}
.banner-overlay {
  position: absolute;
  top: 0;
  left: 1rem;
  right: 0;
  bottom: 0;
  height: 200px;
  width: calc(100% - 2rem);
}
.engine-icon {
  width: 32px;
  height: 32px;
  margin-left: 8px;
}

.engine-logo-img {
  position: absolute;
  width: auto;
  height: 100px;
  object-fit: contain;
  overflow: hidden;
  bottom: 1rem;
  left: 1rem;
}
.banner-version {
  position: absolute;
  bottom: 1rem;
  right: 1rem;
  font-size: 0.75rem;
  color: var(--theme-text);
  margin-left: 4px;
  text-shadow: 1px 1px 4px rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-direction: row;
}

.engine-text {
  margin-bottom: none;
}
.engine-version {
  font-size: 0.75rem;
  color: var(--theme-text-secondary);
  margin-left: 4px;
}
.more-engines {
  border-radius: 1rem;
}
.modal-main-content {
  padding: 1rem;
  color: var(--theme-text);
  width: 100%;
  text-wrap-mode: wrap;
}
.more-engines-content {
  flex-wrap: wrap;
  display: flex;
  padding: 1rem 0;
  gap: 8px;
  text-align: center;
}
:deep(.dropdown-icon) {
  color: var(--theme-text-secondary);
}

.link {
  color: var(--theme-primary);
  text-decoration: underline;
}

.link:hover {
  color: var(--theme-primary-light);
}
</style>
