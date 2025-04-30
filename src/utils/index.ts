import { readTextFile, exists } from "@tauri-apps/plugin-fs";
import { sep } from "@tauri-apps/api/path";
// Function to format engine names
/**
 * @param engineType The engine type string to format
 * @returns The formatted engine name
 */
export function formatEngineName(engineType: string): string {
  switch (engineType) {
    case 'psych':
      return 'Psych Engine';
    case 'kade':
      return 'Kade Engine';
    case 'vanilla':
      return 'Vanilla';
    case 'fps-plus':
      return 'FNF-Plus';
    case 'prevslice':
      return 'Ludum Dare';
    case 'codename':
      return 'Codename Engine';
    default:
      return engineType; // Return original if unknown
  }
}

// Function to get a mod's metadata.json content
/**
 * @param modFolderPath The path to the mod folder
 * @returns The parsed metadata from the metadata.json file
 */
export function getMetadataJSON(modFolderPath: string): Promise<any> {
  return new Promise(async (resolve, reject) => {
    try {
      const metadataPath = `${modFolderPath}${sep()}.flight${sep()}metadata.json`;
      if (await exists(metadataPath)) {
        const metadataContent = await readTextFile(metadataPath);
        const metadata = JSON.parse(metadataContent);
        resolve(metadata);
      } else {
        reject(new Error("Metadata file not found."));
      }
    } catch (error) {
      reject(error);
    }
  });
}