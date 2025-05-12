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

/**
 * DownloadStateManager - Singleton class for managing download state
 */
class DownloadStateManager {
  private static instance: DownloadStateManager;
  
  // Store to track all currently downloading mods
  public downloadingMods = reactive<Record<string, DownloadProgress>>({});

  // Private constructor to prevent direct instantiation
  private constructor() {}

  // Get the singleton instance
  public static getInstance(): DownloadStateManager {
    if (!DownloadStateManager.instance) {
      DownloadStateManager.instance = new DownloadStateManager();
    }
    return DownloadStateManager.instance;
  }

  // Create a new download entry and return its ID
  public createDownload(modId: number, name: string, thumbnailUrl: string = ""): string {
    const id = uuidv4();
    this.downloadingMods[id] = {
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
  public updateDownloadProgress(progress: Partial<DownloadProgress> & { id: string }) {
    // Debug logging to identify issues
    console.log(`Updating download progress for ID ${progress.id}:`, progress);
    
    if (this.downloadingMods[progress.id]) {
      // Ensure percentage is a number
      if (progress.percentage !== undefined) {
        progress.percentage = Number(progress.percentage);
      }
      
      // Update the download state
      this.downloadingMods[progress.id] = {
        ...this.downloadingMods[progress.id],
        ...progress
      };
      
      console.log(`Updated download state:`, this.downloadingMods[progress.id]);
    } else {
      console.warn(`Could not update download progress: ID ${progress.id} not found in downloadingMods`);
    }
  }

  // Mark a download as complete and remove it after a delay
  public completeDownload(id: string) {
    if (this.downloadingMods[id]) {
      this.downloadingMods[id].isComplete = true;
      this.downloadingMods[id].percentage = 100;
      
      // Remove from list after a delay to allow the user to see the completion
      setTimeout(() => {
        delete this.downloadingMods[id];
      }, 3000);
    }
  }

  // Mark a download as failed
  public errorDownload(id: string, error: string) {
    if (this.downloadingMods[id]) {
      this.downloadingMods[id].isError = true;
      this.downloadingMods[id].error = error;
      
      // Keep error state visible for a while
      setTimeout(() => {
        delete this.downloadingMods[id];
      }, 5000);
    }
  }
}

// Export the singleton instance
export const downloadState = DownloadStateManager.getInstance();

// Export convenience references to the methods for backwards compatibility
export const downloadingMods = downloadState.downloadingMods;
export const createDownload = downloadState.createDownload.bind(downloadState);
export const updateDownloadProgress = downloadState.updateDownloadProgress.bind(downloadState);
export const completeDownload = downloadState.completeDownload.bind(downloadState);
export const errorDownload = downloadState.errorDownload.bind(downloadState);