// GameBanana API types

/**
 * Represents a GameBanana ProfilePage API response.
 * This is used to fetch general mod details.
 */
export interface GBProfilePage {
  _idRow: number
  _nStatus: string
  _bIsPrivate: boolean
  _tsDateModified: number
  _tsDateAdded: number
  _sProfileUrl: string
  _aPreviewMedia: {
    _aImages?: GBImage[]
  }
  _sFeedbackInstructions?: string
  _sCommentsMode: string
  _bAccessorIsSubmittor?: boolean
  _bIsTrashed: boolean
  _bIsWithheld: boolean
  _sName: string
  _nUpdatesCount?: number
  _bHasUpdates: boolean
  _tsDateUpdated?: number
  _nAllTodosCount: number
  _bHasTodos: boolean
  _nPostCount?: number
  _aTags: GBTag[]
  _bCreatedBySubmitter?: boolean
  _bIsPorted?: boolean
  _nThanksCount: number
  _sInitialVisibility: string
  _sDownloadUrl: string
  _nDownloadCount: number
  _aFiles?: GBFile[]
  _nSubscriberCount: number
  _aStudio?: GBStudio
  _aContributingStudios: unknown[]
  _sLicense: string
  _aLicenseChecklist: {
    yes?: string[]
    ask?: string[]
    no?: string[]
  }
  _aContentRatings?: Record<string, string>
  _aEmbeddedMedia?: string[]
  _sDescription?: string
  _bGenerateTableOfContents: boolean
  _sText: string
  _bIsObsolete?: boolean
  _nLikeCount?: number
  _nViewCount?: number
  _sVersion?: string
  _bAcceptsDonations: boolean
  _bShowRipePromo: boolean
  _aEmbeddables: {
    _sEmbeddableImageBaseUrl: string
    _aVariants: string[]
  }
  _aSubmitter: GBSubmitter
  _bFollowLinks: boolean
  _aGame: GBGame
  _aCategory: GBCategory
  _aSuperCategory?: GBCategory
  _aAlternateFileSources?: GBAltFile[]
  _aCredits: GBCredit[]
  _bAdvancedRequirementsExist?: boolean
  _aRequirements?: GBRequirement[]
  _sDevNotes?: string
  _aFeaturings?: Record<string, GBFeaturing[]>
  // WIP Specific fields
  _akDevelopmentState?: string
  _sDevelopmentState?: string
  _iCompletionPercentage?: number
  _aFinishedWork?: GBFinishedWork
}

export interface GBStudio {
  _idRow: number
  _sName: string
  _sProfileUrl: string
  _sBannerUrl: string
  _nSubscriberCount: number
  _bAccessorIsSubscribed?: boolean
  _idAccessorSubscriptionRow?: number
}

export interface GBFeaturing {
  _sFeatureGroup: string
  _sIconClasses: string
  _sTitle: string
  _tsDate: number
}

export interface GBFinishedWork {
  _aFinishedWorksOnGameBanana: string[]
  _aRemoteFinishedWorkUrls: string[]
}

/**
 * Represents a GameBanana DownloadPage API response.
 * This is used to fetch mod download details.
 */
export interface GBDownloadPage {
  _bIsTrashed: boolean
  _bIsWithheld: boolean
  _aFiles: GBFile[]
  _bAcceptsDonations: boolean
  _bShowRipePromo: boolean
  _aAlternateFileSources: GBAltFile[]
  _sLicense: string
  _sSubmitterInstructions: string
  _aSupportedModManagers: Record<string, GBModManagerIntegration>
}

/**
 * Represents a GameBanana TopSubs API response.
 * This is used to fetch featured submissions.
 */
export interface GBTopSubs {
  _aMetadata: {
    _nRecordCount: number
    _nPerpage: number
    _bIsComplete: boolean
  }
  _aRecords: GBTopSubsItem[]
}

// Represents a Mod from TopSubs
export interface GBTopSubsItem {
  _idRow: number
  _sModelName: string
  _sName: string
  _sProfileUrl: string
  _sImageUrl: string
  _sThumbnailUrl: string
  _sInitialVisibility: string
  _sPeriod:
    | 'today'
    | 'week'
    | 'month'
    | '3month'
    | '6month'
    | 'year'
    | 'alltime'
  _aSubmitter: GBMiniSubmitter
  _nLikeCount?: number
  _nPostCount?: number
  _aRootCategory: GBMiniCategory
  _sDescription?: string
}

/**
 * Represents a GameBanana Subfeed API response.
 * This is used to fetch submissions in a list.
 */
export interface GBSubfeed {
  _aMetadata: {
    _nRecordCount: number
    _nPerpage: number
    _bIsComplete: boolean
  }
  _aRecords: GBSubfeedRecord[]
}

// Represents a mod in the Subfeed
export interface GBSubfeedRecord {
  _idRow: number
  _sModelName: string
  _sSingularTitle: string
  _sIconClasses: string
  _sName: string
  _sProfileUrl: string
  _tsDateAdded: number
  _tsDateModified: number
  _bHasFiles: boolean
  _aTags: string[]
  _aPreviewMedia: {
    _aImages: GBImage[]
  }
  _aSubmitter: GBMiniSubmitter
  _aRootCategory: GBMiniCategory
  _sVersion?: string
  _tsDateUpdated?: number
  _bIsObsolete?: boolean
  _sInitialVisibility: string
  _bHasContentRatings: boolean
  _nLikeCount?: number
  _nPostCount?: number
  _bWasFeatured: boolean
  _nViewCount?: number
  _bIsOwnedByAccessor: boolean
}

/**
 * Represents a GameBanana Posts API response.
 * This is used to fetch mod posts.
 */
export interface GBModPosts {
  _aMetadata: {
    _nRecordCount: number
    _nPerpage: number
    _bIsComplete: boolean
  }
  _aRecords: (GBModPost | GBModTrashedPost)[]
}

// Represents a post on a mod
export interface GBModPost {
  _idRow: number
  _nStatus: object
  _tsDateAdded: number
  _tsDateModified: number
  _nReplyCount: number
  _iPinLevel: number
  _nStampScore: number
  _aPreviewMedia?: object[]
  _sText: string
  _aPoster: GBSubmitter
  _bFollowLinks: boolean
  _aStamps?: GBStamp[]
  _aLabels?: string[]
}

interface GBModTrashedPost {
  _idRow: number
  _nStatus: number
  _tsDateAdded: number
  _tsDateModified: number
  _nReplyCount: number
  _iPinLevel: number
  _nStampScore: number
  _aPreviewMedia?: object[]
  _sText: string
  _aStamps?: GBStamp[]
  _aLabels?: string[]
}

/**
 * Represents a GameBanana Updates API response.
 * This is used to fetch submitted mod updates.
 */
export interface GBModUpdates {
  _aMetadata: {
    _nRecordCount: number
    _nPerpage: number
    _bIsComplete: boolean
  }
  _aRecords: GBModUpdate[]
}

// Represents a mod update in GameBanana
export interface GBModUpdate {
  _idRow: number
  _nStatus: string
  _bIsPrivate: boolean
  _tsDateAdded: number
  _tsDateModified: number
  _sProfileUrl: string
  _aPreviewMedia: {
    _aMetadata: {
      _sSnippet: string
    }
  }
  _bAccessorIsSubmittor?: boolean
  _bIsTrashed: boolean
  _sName: string
  _nPostCount?: number
  _sInitialVisibility: string
  _bHasFiles: boolean
  _sText: string
  _bShowRipePromo: boolean
  _aSubmitter: GBSubmitter
  _bFollowLinks: boolean
  _sVersion?: string
  _aChangeLog?: GBChangeLogEntry[]
  _aSubmission: GBMiniSubmission
  _aAccess: {
    Update_Edit: boolean
    Update_Trash: boolean
  }
}

// Reduced version of the normal submission, used in some places
export interface GBMiniSubmission {
  _sName: string
  _sProfileUrl: string
  _sModelName: string
  _sDownloadUrl: string
}

// Represents a change log entry in GameBanana updates
export interface GBChangeLogEntry {
  text: string
  cat: string
}

// Represents a GameBanana tag
export interface GBTag {
  _sTitle: string
  _sValue: string
}

// Represents a GameBanana stamp
export interface GBStamp {
  _sIconClasses?: string
  _sTitle?: string
  _sCategory?: string
  _nCount: number
  _UnlockName?: string
}

// Represents a GameBanana submitter (author) profile
export interface GBSubmitter {
  _idRow: number
  _sName: string
  _sUserTitle: string
  _sHonoraryTitle: string
  _tsJoinDate: number
  _sAvatarUrl: string
  _sSigUrl?: string
  _sHdAvatarUrl?: string
  _sUpicUrl?: string
  _sSubjectShaper?: {
    _sBorderStyle: string
    _sFont: string
    _sTextColor: string
    _sTextHoverColor: string
  }
  _sSubjectShaperCssCode?: string
  _sHovatarUrl?: string
  _sMoreByUrl?: string
  _sProfileUrl: string
  _sPointsUrl?: string
  _sMedalsUrl?: string
  _bIsOnline: boolean
  _sLocation?: string
  _sOnlineTitle?: string
  _sOfflineTitle?: string
  _nPoints?: number
  _nPointsRank?: number
  _aNormalMedals?: GBMedal[]
  _aRareMedals?: GBMedal[]
  _aLegendaryMedals?: GBMedal[]
  _bHasRipe: boolean
  _nBuddyCount?: number
  _nSubscriberCount?: number
  _aDonationMethods?: GBDonationMethod[]
  _bAccessorIsBuddy?: boolean
  _bBuddyRequestExistsWithAccessor?: boolean
  _bAccessorIsSubscribed?: boolean
  _aBio?: GBBioItem[]
  _aClearanceLevels?: string[]
}

// Reduced version of a normal submitter, used in some places
export interface GBMiniSubmitter {
  _idRow: number
  _sName: string
  _bIsOnline: boolean
  _bHasRipe: boolean
  _sProfileUrl: string
  _sAvatarUrl: string
  _sHdAvatarUrl?: string
  _sUpicUrl?: string
  _sSubjectShaper?: {
    _sBorderStyle: string
    _sFont: string
    _sTextColor: string
    _sTextHoverColor: string
  }
  _sSubjectShaperCssCode?: string
  _sHovatarUrl?: string
  _sMoreByUrl?: string
  _aClearanceLevels?: string[]
}

export interface GBDonationMethodValidator {
  _regexValidPattern: string
  _sWarningMessage: string
}

export interface GBDonationMethod {
  _aValidator: GBDonationMethodValidator
  _bIsUrl: boolean
  _sFormattedValue?: string
  _sIconClasses: string
  _sInputPlaceholder: string
  _sInputType: string
  _sTitle: string
  _sValue: string
  _sValueTemplate?: string
}

// Represents a game in GameBanana API
export interface GBGame {
  _idRow: number
  _sName: string
  _sAbbreviation: string
  _sProfileUrl: string
  _sIconUrl: string
  _sBannerUrl: string
  _nSubscriberCount: number
  _bHasSubmissionQueue: boolean
  _bAccessorIsSubscribed: boolean
}

// Represents an image in _aImages
export interface GBImage {
  _sType: string
  _sBaseUrl: string
  _sCaption?: string
  _sFile: string
  _sFile220?: string
  _sFile530?: string
  _sFile100?: string
  _hFile220?: number
  _wFile220?: number
  _hFile530?: number
  _wFile530?: number
  _hFile100?: number
  _wFile100?: number
}

// Represents a file in _aFiles
export interface GBFile {
  _idRow: number
  _sFile: string
  _nFilesize: number
  _tsDateAdded: number
  _nDownloadCount: number
  _sDownloadUrl: string
  _sMd5Checksum: string
  _sAnalysisState: string
  _sAnalysisResult: string
  _sAnalysisResultVerbose: string
  _sAvastAvState?: string
  _sAvastAvResult?: string
  _sVersion?: string
  _bHasContents: boolean
  _sDescription?: string
  _aAnalysisWarnings?: {
    contains_exe?: string[]
    nested_archive?: string[]
  }
  _aModManagerIntegrations?: GBModManagerIntegration[]
}

// Represents a mod manager integration item
export interface GBModManagerIntegration {
  _sIconClasses?: string
  _idToolRow: number
  _aGameRowIds: number[]
  _sInstallerName: string
  _sInstallerUrl: string
  _sIconUrl: string
  _sDownloadUrl: string
}

// Represents an alternate file source in GameBanana API
export interface GBAltFile {
  url: string
  description?: string
}

// Represents a category in GameBanana API
export interface GBCategory {
  _idRow: number
  _sName: string
  _sModelName: string
  _sProfileUrl: string
  _sIconUrl: string
}

// Reduced version of a category, used in some places
type GBMiniCategory = Omit<GBCategory, '_idRow' | '_sModelName'>

// Represents a medal in GameBanana API
export type GBMedal = [string, string, string, number]

// Represents a requirement in GameBanana API
export type GBRequirement = string[]

// Represents a bio item in GameBanana API
export type GBBioItem = {
  _sTitle: string
  _sValue: string
}

// Represents a credit group in GameBanana API
export type GBCredit = {
  _sGroupName: string
  _aAuthors: GBCreditAuthor[]
}

// Represents an author in GameBanana API credit
export type GBCreditAuthor = {
  _sRole?: string
  _idRow?: number
  _sName: string
  _sProfileUrl?: string
  _sUrl?: string
  _bIsOnline?: boolean
  _sUpicUrl?: string
  _aAffiliatedStudio?: {
    _sProfileUrl: string
    _sName: string
    _sFlagUrl: string
    _sBannerUrl: string
  }
}
