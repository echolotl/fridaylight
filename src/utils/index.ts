import { readTextFile, exists } from '@tauri-apps/plugin-fs'
import { sep } from '@tauri-apps/api/path'

/**
 * Function to format the engine name based on the engine type
 * @param engineType The engine type string to format
 * @returns The formatted engine name
 */
export function formatEngineName(engineType: string): string {
  switch (engineType) {
    case 'psych':
      return 'Psych Engine'
    case 'kade':
      return 'Kade Engine'
    case 'vanilla':
      return 'Vanilla'
    case 'fps-plus':
      return 'FPS Plus'
    case 'prevslice':
      return 'Ludum Dare'
    case 'codename':
      return 'Codename Engine'
    default:
      return engineType // Return original if unknown
  }
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
