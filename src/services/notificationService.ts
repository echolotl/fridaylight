import { Notify } from 'quasar'

export interface NotificationOptions {
  message: string
  caption?: string
  timeout?: number
  position?:
    | 'top'
    | 'top-left'
    | 'top-right'
    | 'bottom'
    | 'bottom-left'
    | 'bottom-right'
    | 'left'
    | 'right'
    | 'center'
  actions?: Array<{
    label: string
    color?: string
    handler: () => void
  }>
  onDismiss?: () => void
}

export interface OngoingNotificationResult {
  dismiss: () => void
  update: (options: Partial<NotificationOptions>) => void
}

export type NotificationType =
  | 'positive'
  | 'negative'
  | 'warning'
  | 'info'
  | 'ongoing'

/**
 * Centralized notification service for managing all application notifications
 * This service provides a consistent interface for displaying notifications
 * throughout the application while abstracting away the underlying Quasar implementation
 */
export class NotificationService {
  private static instance: NotificationService
  private activeOngoingNotifications = new Map<string, () => void>()

  private constructor() {}

  /**
   * Get the singleton instance of the notification service
   */
  public static getInstance(): NotificationService {
    if (!NotificationService.instance) {
      NotificationService.instance = new NotificationService()
    }
    return NotificationService.instance
  }

  /**
   * Show a success notification
   */
  public success(options: NotificationOptions): void {
    this.showNotification('positive', options)
  }

  /**
   * Show an error notification
   */
  public error(options: NotificationOptions): void {
    this.showNotification('negative', options)
  }

  /**
   * Show a warning notification
   */
  public warning(options: NotificationOptions): void {
    this.showNotification('warning', options)
  }

  /**
   * Show an info notification
   */
  public info(options: NotificationOptions): void {
    this.showNotification('info', options)
  }

  /**
   * Show an ongoing notification (with no timeout)
   * Returns an object with methods to dismiss or update the notification
   */
  public ongoing(options: NotificationOptions): OngoingNotificationResult {
    const notificationId = this.generateNotificationId()

    const dismissFn = Notify.create({
      type: 'ongoing',
      message: options.message,
      caption: options.caption,
      position: options.position || 'bottom-right',
      timeout: 0, // No timeout for ongoing notifications
      actions: options.actions?.map(action => ({
        label: action.label,
        color: action.color || 'white',
        handler: action.handler,
      })),
      onDismiss: options.onDismiss,
    })

    // Store the dismiss function
    this.activeOngoingNotifications.set(notificationId, dismissFn)

    return {
      dismiss: () => {
        const dismissFunction =
          this.activeOngoingNotifications.get(notificationId)
        if (dismissFunction) {
          try {
            dismissFunction()
          } catch (error) {
            console.warn('Error dismissing notification:', error)
          }
          this.activeOngoingNotifications.delete(notificationId)
        }
      },
      update: (updateOptions: Partial<NotificationOptions>) => {
        // For updates, we need to dismiss the old one and create a new one
        const dismissFunction =
          this.activeOngoingNotifications.get(notificationId)
        if (dismissFunction) {
          try {
            dismissFunction()
          } catch (error) {
            console.warn('Error dismissing notification for update:', error)
          }
        }

        // Create new notification with updated options
        const mergedOptions = { ...options, ...updateOptions }
        const newDismissFn = Notify.create({
          type: 'ongoing',
          message: mergedOptions.message,
          caption: mergedOptions.caption,
          position: mergedOptions.position || 'bottom-right',
          timeout: 0,
          actions: mergedOptions.actions?.map(action => ({
            label: action.label,
            color: action.color || 'white',
            handler: action.handler,
          })),
          onDismiss: mergedOptions.onDismiss,
        })

        // Update the stored dismiss function
        this.activeOngoingNotifications.set(notificationId, newDismissFn)
      },
    }
  }

  /**
   * Show a download progress notification
   * This is a convenience method for download-related notifications
   */
  public downloadProgress(
    modName: string,
    progress?: string
  ): OngoingNotificationResult {
    const message = progress
      ? `Downloading "${modName}"... ${progress}`
      : `Downloading "${modName}"...`

    return this.ongoing({
      message,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update progress notification
   * This is a convenience method for update-related notifications
   */
  public updateProgress(
    modName: string,
    progress?: string
  ): OngoingNotificationResult {
    const message = progress
      ? `Updating "${modName}"... ${progress}`
      : `Updating "${modName}"...`

    return this.ongoing({
      message,
      position: 'bottom-right',
    })
  }

  /**
   * Show a download preparation notification
   */
  public downloadPreparing(modName: string): OngoingNotificationResult {
    return this.ongoing({
      message: `Preparing to download "${modName}"...`,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update preparation notification
   */

  public updatePreparing(modName: string): OngoingNotificationResult {
    return this.ongoing({
      message: `Preparing to update "${modName}"...`,
      position: 'bottom-right',
    })
  }

  /**
   * Show a download success notification
   */
  public downloadSuccess(modName: string, caption?: string): void {
    this.success({
      message: `"${modName}" downloaded and installed successfully!`,
      caption: caption || 'Ready to play from the mods list',
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update success notification
   */

  public updateSuccess(modName: string, caption?: string): void {
    this.success({
      message: `"${modName}" updated successfully!`,
      caption: caption || 'Ready to play from the mods list',
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a download error notification
   */
  public downloadError(modName: string, error: string): void {
    this.error({
      message: `Failed to download "${modName}"`,
      caption: error,
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update error notification
   */

  public updateError(modName: string, error: string): void {
    this.error({
      message: `Failed to update "${modName}"`,
      caption: error,
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a download cancelled notification
   */
  public downloadCancelled(modName: string): void {
    this.info({
      message: `Download cancelled for "${modName}"`,
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update cancelled notification
   */

  public updateCancelled(modName: string): void {
    this.info({
      message: `Update cancelled for "${modName}"`,
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show an update not available notification
   */
  public updateNotAvailable(modName: string): void {
    this.info({
      message: 'Update not available',
      caption: `${modName} cannot be updated`,
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a non-specific download cancelled notification
   */
  public downloadCancelledGeneric(): void {
    this.info({
      message: 'Download cancelled',
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a installation failure notification
   */
  public installationFailed(modName: string, error: string): void {
    this.error({
      message: `Failed to install "${modName}"`,
      caption: error,
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a mod operation notification (start/stop)
   */
  public modStopped(modName: string): void {
    this.info({
      message: `${modName} has stopped`,
      timeout: 2000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a mod error notification
   */
  public modError(modName: string, operation: string, error: string): void {
    this.error({
      message: `Failed to ${operation} ${modName}`,
      caption: error,
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a modpack download error notification
   */
  public modpackNoEngineError(modpackName: string, engine_name: string): void {
    this.error({
      message: `Unable to download a ${engine_name} modpack`,
      caption: `Modpack "${modpackName}" requires ${engine_name}, which is not installed.`,
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a general operation success notification
   */
  public operationSuccess(message: string, caption?: string): void {
    this.success({
      message,
      caption,
      timeout: 3000,
      position: 'bottom-right',
    })
  }

  /**
   * Show a general operation error notification
   */
  public operationError(message: string, error: string): void {
    this.error({
      message,
      caption: error,
      timeout: 5000,
      position: 'bottom-right',
    })
  }

  /**
   * Dismiss all ongoing notifications
   */
  public dismissAllOngoing(): void {
    for (const [id, dismissFn] of this.activeOngoingNotifications.entries()) {
      try {
        dismissFn()
      } catch (error) {
        console.warn(`Error dismissing notification ${id}:`, error)
      }
    }
    this.activeOngoingNotifications.clear()
  }

  /**
   * Private method to show a notification with the specified type
   */
  private showNotification(
    type: NotificationType,
    options: NotificationOptions
  ): void {
    Notify.create({
      type: type === 'ongoing' ? 'info' : type, // Map ongoing to info for regular notifications
      message: options.message,
      caption: options.caption,
      timeout: options.timeout || (type === 'negative' ? 5000 : 3000),
      position: options.position || 'bottom-right',
      actions: options.actions?.map(action => ({
        label: action.label,
        color: action.color || 'white',
        handler: action.handler,
      })),
      onDismiss: options.onDismiss,
    })
  }

  /**
   * Generate a unique ID for ongoing notifications
   */
  private generateNotificationId(): string {
    return `notification_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }
}

// Export a singleton instance for convenience
export const notificationService = NotificationService.getInstance()
