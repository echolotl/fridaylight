// Download state store for tracking mod downloads
import { reactive } from 'vue';

export interface DownloadProgress {
  modId: number;
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
export const downloadingMods = reactive<Record<number, DownloadProgress>>({});

// Add or update a mod in the downloading list
export function updateDownloadProgress(progress: DownloadProgress) {
  downloadingMods[progress.modId] = {
    ...downloadingMods[progress.modId],
    ...progress
  };
}

// Mark a download as complete and remove it after a delay
export function completeDownload(modId: number) {
  if (downloadingMods[modId]) {
    downloadingMods[modId].isComplete = true;
    downloadingMods[modId].percentage = 100;
    
    // Remove from list after a delay to allow the user to see the completion
    setTimeout(() => {
      delete downloadingMods[modId];
    }, 3000);
  }
}

// Mark a download as failed
export function errorDownload(modId: number, error: string) {
  if (downloadingMods[modId]) {
    downloadingMods[modId].isError = true;
    downloadingMods[modId].error = error;
    
    // Keep error state visible for a while
    setTimeout(() => {
      delete downloadingMods[modId];
    }, 5000);
  }
}