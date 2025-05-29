import { readTextFile, exists } from "@tauri-apps/plugin-fs";
import { sep } from "@tauri-apps/api/path";
import { resolveResource } from '@tauri-apps/api/path';

/**
 * Function to format the engine name based on the engine type 
 * @param engineType The engine type string to format
 * @returns The formatted engine name
 */
export async function formatEngineName(engineType: string): Promise<string> {
  // Get the JSON for the engine type
  const engineTypeData = await resolveResource(`resources/${engineType}.json`);
  if (engineTypeData) {
    try {
      const engineTypeJson = JSON.parse(await readTextFile(engineTypeData));
      // Return the formatted name
      return engineTypeJson.name || engineType;
    } catch (error) {
      console.error("Error parsing engine type JSON:", error);
      return engineType; // Fallback to original if parsing fails
    }
  }
  return engineType; // Default return if no engineTypeData 
}

/**
 * Function to get and parse the metadata.json file from a mod folder
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


/**
 * Function to get the mods folder path for a given engine executable
 * @param executablePath The path to the engine executable
 * @param customModsFolder Optional custom mods folder name
 * @returns The path to the mods folder
 */
export function getEngineModsFolderPath(executablePath: string, customModsFolder?: string): string {

  // Check if the path is valid
  const lastSlashIndex = Math.max(
    executablePath.lastIndexOf("/"),
    executablePath.lastIndexOf("\\")
  );
  
  const baseDir = lastSlashIndex > 0 
    ? executablePath.substring(0, lastSlashIndex) 
    : executablePath;
    
  // Most engines use "mods" folder in the same directory as the executable
  return `${baseDir}${sep()}${customModsFolder || "mods"}${sep()}`;
}