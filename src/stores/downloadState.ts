// Download state store for tracking mod downloads
import { reactive } from 'vue';
import { v4 as uuidv4 } from "uuid";

export interface DownloadProgress {
  id: string;        // Unique identifier for this download
  modId: number;     // GameBanana mod ID or other source ID
  name: string;
  bytesDownloaded: number;
  totalBytes: number;
  percentage: number;
  step: string;
  thumbnailUrl: string;
  isComplete: boolean;
  isError: boolean;
  error?: string;
}

// Store to track all currently downloading mods
export const downloadingMods = reactive<Record<string, DownloadProgress>>({});

// Create a new download entry and return its ID
export function createDownload(modId: number, name: string, thumbnailUrl: string = ""): string {
  const id = uuidv4();
  downloadingMods[id] = {
    id,
    modId,
    name,
    bytesDownloaded: 0,
    totalBytes: 0,
    percentage: 0,
    step: "Preparing...",
    thumbnailUrl,
    isComplete: false,
    isError: false
  };
  return id;
}

// Add or update a mod in the downloading list
export function updateDownloadProgress(progress: Partial<DownloadProgress> & { id: string }) {
  // Debug logging to identify issues
  console.log(`Updating download progress for ID ${progress.id}:`, progress);
  
  if (downloadingMods[progress.id]) {
    // Ensure percentage is a number
    if (progress.percentage !== undefined) {
      progress.percentage = Number(progress.percentage);
    }
    
    // Update the download state
    downloadingMods[progress.id] = {
      ...downloadingMods[progress.id],
      ...progress
    };
    
    console.log(`Updated download state:`, downloadingMods[progress.id]);
  } else {
    console.warn(`Could not update download progress: ID ${progress.id} not found in downloadingMods`);
  }
}

// Mark a download as complete and remove it after a delay
export function completeDownload(id: string) {
  if (downloadingMods[id]) {
    downloadingMods[id].isComplete = true;
    downloadingMods[id].percentage = 100;
    
    // Remove from list after a delay to allow the user to see the completion
    setTimeout(() => {
      delete downloadingMods[id];
    }, 3000);
  }
}

// Mark a download as failed
export function errorDownload(id: string, error: string) {
  if (downloadingMods[id]) {
    downloadingMods[id].isError = true;
    downloadingMods[id].error = error;
    
    // Keep error state visible for a while
    setTimeout(() => {
      delete downloadingMods[id];
    }, 5000);
  }
}