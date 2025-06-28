<template>
  <div
    v-if="hasGamebanana && gameBananaInfo && gameBananaInfo.url"
    class="flex justify-center"
  >
    <q-btn
      flat
      label="Open in Gamebanana"
      class="q-mb-sm"
      icon="M24,6v10.5h-1.5v3h-1.5v1.5h-1.5v1.5h-3v1.5H6v-1.5h-3v-1.5h-1.5v-1.5H0v-4.5h9v-1.5h4.5v-1.5h1.5v-4.5h1.5V3h-1.5V0h4.5v3h1.5v1.5h1.5v1.5h1.5Z"
      @click="
        emit('open-gamebanana', gameBananaInfo.id, gameBananaInfo.model_type)
      "
    ></q-btn>
    <div
      v-if="
        actualVersion &&
        versionStatus &&
        versionStatus.is_update_available &&
        versionStatus.completely_sure
      "
      class="text-caption update-available-text q-mt-md"
    >
      {{ $t('ui.update_available') }} <br />
      <div class="text-primary text-center">
        {{ versionStatus.latest_version }}
      </div>
    </div>
    <div
      v-else-if="
        actualVersion &&
        versionStatus &&
        versionStatus.is_update_available &&
        !versionStatus.completely_sure
      "
    >
      <div class="text-caption">
        {{ $t('ui.update_might_be_available') }} <br />
        <div class="text-primary text-center">
          {{ versionStatus.latest_version }}
        </div>
      </div>
    </div>
    <!-- Show loading while checking version -->
    <div v-else-if="!versionStatus" class="text-caption flex items-center">
      <q-spinner size="sm" class="q-mr-sm" />
      {{ $t('ui.checking_for_updates') }}
    </div>
  </div>
  <div v-if="hasContributors" class="contributor-infobox">
    <h6 class="phantom-font-difficulty">{{ $t('misc.credits') }}</h6>
    <hr />
    <div class="contributor-groups">
      <div
        v-for="(group, index) in contributorsData"
        :key="index"
        class="contributor-group"
      >
        <h6 class="contributor-title">{{ group.group }}</h6>
        <div class="contributor-list">
          <div
            v-for="(contributor, memberIndex) in group.members"
            :key="memberIndex"
            class="contributor-item"
          >
            <div v-if="contributor.icon" class="contributor-icon">
              <img
                :src="getIconSrc(contributor)"
                :alt="`${contributor.name}'s icon`"
              />
            </div>
            <div class="contributor-info">
              <div class="contributor-name">
                {{ contributor.name }}
              </div>
              <div v-if="contributor.role" class="contributor-role">
                {{ contributor.role }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch, reactive } from 'vue'
import { Contributor, ContributorGroup, ModInfoGBData } from '@main-types'
import { sep } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  contributors: {
    type: Array as () => ContributorGroup[],
    default: () => [],
  },
  folderPath: {
    type: String,
    default: '',
  },
  version: {
    type: String,
    default: '',
  },
})

const emit = defineEmits(['open-gamebanana'])

// Local state to store contributor data from metadata.json
const contributorsData = ref<ContributorGroup[]>([])
const gameBananaInfo = ref<ModInfoGBData | null>(null)
const actualVersion = ref<string>('')
const metadataLoaded = ref(false)
const metadataData = ref<any>(null)
const versionStatus = ref<VersionResponse | null>(null)

// Map to store processed icon data
const iconCache = reactive(new Map<string, string>())

// Determine if there are contributors to display
const hasContributors = computed(() => {
  return contributorsData.value.length > 0
})
// Determine if there is GameBanana info to display
const hasGamebanana = computed(() => {
  return gameBananaInfo.value !== null
})

// Function to load metadata.json and extract contributors
const loadContributorsFromMetadata = async () => {
  try {
    // Check if metadata has contributors in the supported format
    if (metadataData.value && typeof metadataData.value === 'object') {
      const meta = metadataData.value

      if (Array.isArray(meta.contributors)) {
        // Process only if it has the right structure
        console.log('Found contributors in metadata:', meta.contributors)
        contributorsData.value = meta.contributors

        // Preload all icon data
        preloadIconData()
      } else {
        console.log(
          'No valid contributors format found in metadata, falling back to props:',
          props.contributors
        )
        contributorsData.value = props.contributors
        // Preload icon data for props contributors
        preloadIconData()
      }
    } else {
      console.log(
        'Invalid metadata format, falling back to props:',
        props.contributors
      )
      contributorsData.value = props.contributors
      preloadIconData()
    }
  } catch (error) {
    console.warn('Failed to load metadata.json for contributors:', error)
    // Fallback to props if there's an error
    contributorsData.value = props.contributors
    preloadIconData()
  }
}

const loadGameBananaInfo = async () => {
  try {
    // Check if metadata has GameBanana info
    if (metadataData.value && typeof metadataData.value === 'object') {
      const meta = metadataData.value

      if (meta.gamebanana) {
        console.log('Found GameBanana info in metadata:', meta.gamebanana)
        gameBananaInfo.value = meta.gamebanana
      } else {
        console.log('No GameBanana info found in metadata, using default.')
        gameBananaInfo.value = null
      }
    } else {
      console.log('Invalid metadata format, using default GameBanana info.')
      gameBananaInfo.value = null
    }
    checkGamebananaVersion()
  } catch (error) {
    console.warn('Failed to load GameBanana info:', error)
    gameBananaInfo.value = null
  }
}

const loadMetadataData = async () => {
  if (!props.folderPath) return

  try {
    const metadata = await invoke('get_mod_metadata', {
      modPath: props.folderPath,
    })
    metadataData.value = metadata
    console.log('Metadata data loaded:', metadataData.value)
  } catch (error) {
    console.warn('Failed to load metadata data:', error)
  }
  await loadContributorsFromMetadata()
  await loadGameBananaInfo()
  metadataLoaded.value = true
}

const checkGamebananaVersion = async () => {
  try {
    const version = await invoke('check_gamebanana_mod_version', {
      id: gameBananaInfo.value?.id,
      modelType: gameBananaInfo.value?.model_type,
    })

    actualVersion.value = version as string

    // Check for updates after getting the latest version
    if (props.version && actualVersion.value) {
      versionStatus.value = await checkVersions()
    }
  } catch (error) {
    console.warn('Failed to check GameBanana version:', error)
  }
}

// Function to preload all icon data
const preloadIconData = async () => {
  if (!contributorsData.value || !props.folderPath) return
  const fileSeperator = sep()

  // Extract all contributors needing icon processing
  const allContributors: Contributor[] = []

  // Gather all contributors from all groups
  contributorsData.value.forEach(group => {
    if (Array.isArray(group.members)) {
      allContributors.push(...group.members)
    }
  })

  // Process all contributors to load their icons
  for (const contributor of allContributors) {
    if (contributor.icon && !contributor.icon.startsWith('data:')) {
      const originalPath = contributor.icon
      const iconPath = `${props.folderPath}${fileSeperator}.flight${fileSeperator}${originalPath}`
      console.log(`Preloading icon from: ${iconPath}`)

      try {
        const base64Data = (await invoke('get_file_as_base64', {
          filePath: iconPath,
        })) as string
        console.log(`Successfully loaded icon for ${contributor.name}`)
        iconCache.set(originalPath, base64Data)
      } catch (error) {
        console.warn(
          `Failed to load icon for contributor ${contributor.name}:`,
          error
        )
        // Keep the original path in the cache as a fallback
        iconCache.set(originalPath, originalPath)
      }
    }
  }
}

// Helper function to get the correct icon source
const getIconSrc = (contributor: Contributor): string => {
  if (!contributor.icon) return ''

  // If it's already a data URL, return it
  if (contributor.icon.startsWith('data:')) {
    return contributor.icon
  }

  // Check if we have a cached version
  if (iconCache.has(contributor.icon)) {
    return iconCache.get(contributor.icon) || contributor.icon
  }

  // Return the original path as fallback
  return contributor.icon
}

interface VersionResponse {
  is_update_available: boolean
  latest_version: string
  current_version: string
  completely_sure: boolean
}

// Helper function to check current version vs GameBanana version
const checkVersions = async (): Promise<VersionResponse | null> => {
  if (!props.version || !actualVersion.value) return null

  // If this is formatted as a semver version, compare it
  if (isSemver(props.version) && isSemver(actualVersion.value)) {
    const result = (await invoke('compare_update_semver', {
      currentVersion: props.version,
      latestVersion: actualVersion.value,
    })) as boolean
    console.log(
      `Comparing versions: current=${props.version}, latest=${actualVersion.value}, result=${result}`
    )
    return {
      is_update_available: !result,
      latest_version: actualVersion.value,
      current_version: props.version,
      completely_sure: true,
    }
  } else {
    // If not semver, just compare as strings
    const result = props.version !== actualVersion.value
    return {
      is_update_available: result,
      latest_version: actualVersion.value,
      current_version: props.version,
      completely_sure: false,
    }
  }
}

const isSemver = (version: string): boolean => {
  const semverRegex =
    /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$/
  return semverRegex.test(version)
}

// Load contributors data when component mounts or folderPath changes
onMounted(loadMetadataData)
watch(() => props.folderPath, loadContributorsFromMetadata)

// Watch for version prop changes to re-validate
watch(
  () => props.version,
  async newVersion => {
    if (newVersion && actualVersion.value) {
      versionStatus.value = await checkVersions()
    }
  }
)
</script>

<style scoped>
.contributor-infobox {
  padding: 15px;
  height: fit-content;
  position: sticky;
  top: 20px;
}

.contributor-groups {
  margin-top: 15px;
}

.contributor-group {
  margin-bottom: 15px;
}

.contributor-title {
  color: var(--theme-primary);
  font-weight: bold;
  margin: 5px 0;
  font-size: 1rem;
}

.contributor-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 8px 0;
}

.contributor-item {
  display: flex;
  align-items: center;
  padding: 0 0.9rem;
}

.contributor-item:last-child {
  border-bottom: none;
}

.contributor-icon {
  width: 32px;
  height: 32px;
  overflow: hidden;
  flex-shrink: 0;
  margin-right: 0.5rem;
}

.contributor-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.contributor-info {
  display: flex;
  flex-direction: column;
}

.contributor-name {
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
}

.contributor-role {
  font-size: 0.8rem;
  color: var(--theme-text-secondary);
  opacity: 0.85;
  line-height: 1.2;
}

hr {
  border-color: var(--theme-border);
  margin-top: 5px;
  margin-bottom: 10px;
}

h6 {
  margin: 0;
  color: var(--theme-text);
  margin-right: 6rem;
}

.update-available-text {
  animation: colorFade 1s ease-in-out infinite alternate;
}

@keyframes colorFade {
  0% {
    color: var(--theme-text);
  }
  100% {
    color: var(--q-primary);
  }
}
</style>
