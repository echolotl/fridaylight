import { readTextFile, exists } from '@tauri-apps/plugin-fs'
import { sep } from '@tauri-apps/api/path'
import { resolveResource } from '@tauri-apps/api/path'

/**
 * Function to format the engine name based on the engine type
 * @param engineType The engine type string to format
 * @returns The formatted engine name
 */
export async function formatEngineName(engineType: string): Promise<string> {
  // Get the JSON for the engine type
  const engineTypeData = await resolveResource(`resources/${engineType}.json`)
  if (engineTypeData) {
    try {
      const engineTypeJson = JSON.parse(await readTextFile(engineTypeData))
      // Return the formatted name
      return engineTypeJson.engine_name || engineType
    } catch (error) {
      console.error('Error parsing engine type JSON:', error)
      return engineType // Fallback to original if parsing fails
    }
  }
  return engineType // Default return if no engineTypeData
}

/**
 * Function to get and parse the metadata.json file from a mod folder
 * @param modFolderPath The path to the mod folder
 * @returns The parsed metadata from the metadata.json file
 */
export function getMetadataJSON(modFolderPath: string): Promise<any> {
  const metadataPath = `${modFolderPath}${sep()}.flight${sep()}metadata.json`

  return exists(metadataPath)
    .then(fileExists => {
      if (fileExists) {
        return readTextFile(metadataPath)
      } else {
        throw new Error('Metadata file not found.')
      }
    })
    .then(metadataContent => {
      return JSON.parse(metadataContent)
    })
}

/**
 * Function to get the mods folder path for a given engine executable
 * @param executablePath The path to the engine executable
 * @param customModsFolder Optional custom mods folder name
 * @returns The path to the mods folder
 */
export function getEngineModsFolderPath(
  executablePath: string,
  customModsFolder?: string
): string {
  // Check if the path is valid
  const lastSlashIndex = Math.max(
    executablePath.lastIndexOf('/'),
    executablePath.lastIndexOf('\\')
  )

  const baseDir =
    lastSlashIndex > 0
      ? executablePath.substring(0, lastSlashIndex)
      : executablePath

  // Most engines use "mods" folder in the same directory as the executable
  return `${baseDir}${sep()}${customModsFolder || 'mods'}${sep()}`
}

/**
 * Interface for engine data structure
 */
export interface EngineData {
  engine_type: string
  engine_name: string
  engine_url?: string
  engine_banner?: string
  engine_logo?: string
  engine_icon?: string
  engine_description?: string
  engine_version?: string
  primary?: boolean
  [key: string]: unknown // Allow for additional properties
}

/**
 * Interface for engine type information
 */
export interface EngineTypeInfo {
  engineType: string
  engineName: string
  isPrimary: boolean
  data: EngineData
}

/**
 * Function to get all available engine types from the resources directory
 * @returns Promise<EngineTypeInfo[]> Array of engine type information including primary status
 */
export async function getAllEngineTypes(): Promise<EngineTypeInfo[]> {
  const engineTypes: EngineTypeInfo[] = []

  try {
    // Get the resources/data directory path
    const resourcesDataPath = await resolveResource('resources')
    if (!resourcesDataPath) {
      throw new Error('Resources data directory not found')
    }

    // Read all JSON files in the resources directory
    const { readDir } = await import('@tauri-apps/plugin-fs')
    const entries = await readDir(resourcesDataPath)

    for (const entry of entries) {
      if (entry.isFile && entry.name.endsWith('.json')) {
        try {
          const filePath = `${resourcesDataPath}${sep()}${entry.name}`
          const fileContent = await readTextFile(filePath)
          const engineData = JSON.parse(fileContent)

          // Extract engine type from filename (remove .json extension)
          const engineType = entry.name.replace('.json', '')

          engineTypes.push({
            engineType,
            engineName: engineData.engine_name || engineType,
            isPrimary: engineData.primary || false, // Default to false if primary field doesn't exist yet
            data: engineData,
          })
        } catch (error) {
          console.error(`Error parsing engine file ${entry.name}:`, error)
        }
      }
    }

    // Sort by primary status first (primary engines first), then by name
    engineTypes.sort((a, b) => {
      if (a.isPrimary !== b.isPrimary) {
        return a.isPrimary ? -1 : 1
      }
      return a.engineName.localeCompare(b.engineName)
    })

    return engineTypes
  } catch (error) {
    console.error('Error getting engine types:', error)
    return []
  }
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  else if (bytes < 1048576) return `${(bytes / 1024).toFixed(2)} KB`
  else if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(2)} MB`
  else return `${(bytes / 1073741824).toFixed(2)} GB`
}
