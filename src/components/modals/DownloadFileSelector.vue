<template>
  <q-dialog
    v-model="isOpen"
    persistent
    transition-show="fade"
    transition-hide="fade"
  >
    <q-card class="download-selector-modal phantom-font">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6 phantom-font-difficulty">
          {{ $t('app.modals.download_file_select.title') }}
        </div>
        <q-space />
        <q-btn v-close-popup icon="close" flat round dense @click="cancel" />
      </q-card-section>

      <q-card-section>
        <p>
          {{
            $t('app.modals.download_file_select.description', {
              modName: props.modName,
            })
          }}
        </p>

        <q-list separator>
          <q-item
            v-for="(file, index) in files"
            :key="index"
            v-ripple
            clickable
            :active="selectedFile && selectedFile._idRow === file._idRow"
            active-class="selected-file"
            class="selected-file-inactive"
            @click="selectFile(file)"
          >
            <q-item-section>
              <q-item-label
                >{{ file._sFile }}
                <q-icon
                  v-if="file._aAnalysisWarnings?.contains_exe"
                  name="terminal"
                  class="q-ml-xs"
                  color="red"
                  size="xs"
                >
                  <q-tooltip>
                    <span class="phantom-font">{{
                      $t(
                        'app.modals.download_file_select.contains_executable_files'
                      )
                    }}</span>
                  </q-tooltip>
                </q-icon>
                <q-icon
                  v-if="file._aAnalysisWarnings?.nested_archive"
                  name="archive"
                  class="q-ml-xs"
                  color="orange"
                  size="xs"
                >
                  <q-tooltip>
                    <span class="phantom-font">{{
                      $t(
                        'app.modals.download_file_select.contains_nested_archives'
                      )
                    }}</span>
                  </q-tooltip>
                </q-icon>
                <q-icon
                  v-if="file._sAnalysisState === 'pending'"
                  name="hourglass_empty"
                  class="q-ml-xs"
                  color="orange"
                  size="xs"
                >
                  <q-tooltip>
                    <span class="phantom-font">{{
                      $t('app.modals.download_file_select.analysis_pending')
                    }}</span>
                  </q-tooltip>
                </q-icon>
              </q-item-label>
              <q-item-label
                v-if="file._sDescription"
                caption
                class="file-description"
              >
                {{ file._sDescription }}
              </q-item-label>
              <q-item-label caption>
                <span class="file-size">{{
                  formatFileSize(file._nFilesize)
                }}</span>
                <span class="file-date">{{
                  formatDate(file._tsDateAdded)
                }}</span>
                <span class="file-downloads q-ml-xs"
                  ><q-icon name="download"></q-icon
                  >{{ file._nDownloadCount }}</span
                >
              </q-item-label>
            </q-item-section>
          </q-item>
        </q-list>

        <div
          v-if="alternateFileSources && alternateFileSources.length > 0"
          class="alternate-sources q-mt-md"
        >
          <div class="text-subtitle1">
            {{
              $t('app.modals.download_file_select.alternative_download_sources')
            }}
          </div>
          <q-list separator>
            <q-item
              v-for="(source, index) in alternateFileSources"
              :key="`alt-${index}`"
              v-ripple
              clickable
              tag="a"
              :href="source.url"
              target="_blank"
              rel="noopener"
            >
              <q-item-section>
                <q-item-label>{{ source.description }}</q-item-label>
                <q-item-label caption class="altsource-description">{{
                  source.url
                }}</q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-icon name="open_in_new" />
              </q-item-section>
            </q-item>
          </q-list>
        </div>
      </q-card-section>
      <q-card-actions align="right">
        <q-btn
          v-close-popup
          flat
          label="Cancel"
          color="primary"
          @click="cancel"
        />
        <q-btn
          flat
          label="Download"
          color="primary"
          :disable="!selectedFile"
          @click="confirm"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { GBAltFile, GBFile } from '@custom-types/gamebanana'
import { ref, watch } from 'vue'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  },
  files: {
    type: Array as () => GBFile[],
    default: () => [],
  },
  modName: {
    type: String,
    default: '',
  },
  alternateFileSources: {
    type: Array as () => GBAltFile[],
    default: () => [],
  },
})

const emit = defineEmits(['update:modelValue', 'select', 'cancel'])

const isOpen = ref(props.modelValue)
const selectedFile = ref<GBFile | null>(null)

// Set the default selected file to the first file with the most downloads
watch(
  () => props.files,
  newFiles => {
    if (newFiles && newFiles.length > 0) {
      // Sort by download count and set the most downloaded as default
      const sortedFiles = [...newFiles].sort(
        (a, b) => b._nDownloadCount - a._nDownloadCount
      )
      selectedFile.value = sortedFiles[0]
    }
  },
  { immediate: true }
)

watch(
  () => props.modelValue,
  val => {
    isOpen.value = val
  }
)

watch(isOpen, val => {
  emit('update:modelValue', val)
})

const selectFile = (file: GBFile) => {
  selectedFile.value = file
}

const confirm = () => {
  if (selectedFile.value) {
    emit('select', selectedFile.value)
    isOpen.value = false
  }
}

const cancel = () => {
  emit('cancel')
  isOpen.value = false
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'

  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatDate = (timestamp: number): string => {
  const date = new Date(timestamp * 1000)
  return date.toLocaleDateString()
}
</script>

<style scoped>
.download-selector-modal {
  width: 700px;
  max-width: 90vw;
  max-height: 90vh;
  background-color: var(--theme-solid);
  color: var(--theme-text);
  border: var(--theme-border) 2px solid;
  backdrop-filter: blur(30px);
}

.selected-file-inactive {
  color: var(--theme-text) !important;
  border-radius: 0.5rem;
}
.selected-file {
  background-color: var(--q-primary) !important;
  color: white !important;
  border-radius: 0.5rem;
}

.file-size {
  font-weight: 500;
  color: var(--theme-text-secondary);
}

.file-date {
  margin-left: 10px;
  color: var(--theme-text-secondary);
}

.file-downloads {
  color: var(--theme-text-secondary);
}

.file-description {
  margin-top: 5px;
  white-space: pre-line;
  color: var(--theme-text-secondary);
}

.alternate-sources {
  border-top: 1px solid var(--theme-border);
  padding-top: 16px;
}

.alternate-sources .altsource-description {
  color: var(--theme-text-secondary);
}

.q-field :deep(.q-field__label) {
  color: var(--theme-text) !important;
}

.q-field.q-field--outlined :deep(.q-field__control) {
  color: var(--theme-text);
}
</style>
