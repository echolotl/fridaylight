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
  _bCreatedBySubmitter: boolean
  _bIsPorted: boolean
  _nThanksCount: number
  _sInitialVisibility: string
  _sDownloadUrl: string
  _nDownloadCount: number
  _aFiles: GBFile[]
  _nSubscriberCount: number
  _aContributingStudios: unknown[]
  _sLicense: string
  _aLicenseChecklist: {
    yes: string[]
    ask: string[]
    no: string[]
  }
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
export type GBTopSubs = Array<GBTopSubsItem>

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
  _sDescription: string
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
  _aRecords: GBModPost[]
}

// Represents a post on a mod
interface GBModPost {
  _idRow: number
  _nStatus: number
  _tsDateAdded: number
  _tsDateModified: number
  _nReplyCount: number
  _iPinLevel: number
  _nStampScore: number
  _aPreviewMedia?: {
    _aImages: GBImage[]
  }
  _sText: string
  _aPoster: GBSubmitter
  _bFollowLinks: boolean
  _aStamps: GBStamp[]
  _aLabels: string[]
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
interface GBModUpdate {
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
  _aChangeLog: GBChangeLogEntry[]
  _aSubmission: GBMiniSubmission
  _aAccess: {
    Update_Edit: boolean
    Update_Trash: boolean
  }
}

// Reduced version of the normal submission, used in some places
interface GBMiniSubmission {
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
interface GBTag {
  _sTitle: string
  _sValue: string
}

// Represents a GameBanana stamp
interface GBStamp {
  _sIconClasses: string
  _sTitle: string
  _sCategory: string
  _nCount: number
}

// Represents a GameBanana submitter (author) profile
interface GBSubmitter {
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
  _aDonationMethods?: string[]
  _bAccessorIsBuddy?: boolean
  _bBuddyRequestExistsWithAccessor?: boolean
  _bAccessorIsSubscribed?: boolean
  _aBio?: GBBioItem[]
}

// Reduced version of a normal submitter, used in some places
interface GBMiniSubmitter {
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
}

// Represents a game in GameBanana API
interface GBGame {
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
interface GBImage {
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
interface GBFile {
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
  _sAvastAvState: string
  _sAvastAvResult: string
  _bHasContents: boolean
  _sDescription?: string
  _aModManagerIntegrations?: GBModManagerIntegration[]
}

// Represents a mod manager integration item
interface GBModManagerIntegration {
  _sIconClasses?: string
  _idToolRow: number
  _aGameRowIds: number[]
  _sInstallerName: string
  _sInstallerUrl: string
  _sIconUrl: string
  _sDownloadUrl: string
}

// Represents an alternate file source in GameBanana API
interface GBAltFile {
  url: string
  description: string
}

// Represents a category in GameBanana API
interface GBCategory {
  _idRow: number
  _sName: string
  _sModelName: string
  _sProfileUrl: string
  _sIconUrl: string
}

// Reduced version of a category, used in some places
type GBMiniCategory = Omit<GBCategory, '_idRow' | '_sModelName'>

// Represents a medal in GameBanana API
type GBMedal = [string, string, string, number]

// Represents a requirement in GameBanana API
type GBRequirement = [string, string]

// Represents a bio item in GameBanana API
type GBBioItem = {
  _sTitle: string
  _sValue: string
}

// Represents a credit group in GameBanana API
type GBCredit = {
  _sGroupName: string
  _aAuthors: GBCreditAuthor[]
}

// Represents an author in GameBanana API credit
type GBCreditAuthor = {
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
