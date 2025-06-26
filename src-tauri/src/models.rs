use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt;

// Constants for metadata versioning
pub const MIN_METADATA_VERSION: u32 = 1;
pub const CURRENT_METADATA_VERSION: u32 = 1;

// Error type for metadata validation
#[derive(Debug, Clone)]
pub enum MetadataError {
  MissingVersion,
  VersionTooLow {
    provided: u32,
    minimum: u32,
  },
}

impl fmt::Display for MetadataError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MetadataError::MissingVersion =>
        write!(f, "Missing required metadata_version field"),
      MetadataError::VersionTooLow { provided, minimum } =>
        write!(
          f,
          "Metadata version {} is too old. Minimum supported version is {}",
          provided,
          minimum
        ),
    }
  }
}

// GameBanana API types

// Represents a medal in GameBanana API
pub type GBMedal = (String, String, String, i64);

// Represents a requirement in GameBanana API
pub type GBRequirement = Vec<String>;

/// Represents a GameBanana TopSubs API response.
/// This is used to fetch featured submissions.
pub type GBTopSubs = Vec<GBTopSubsItem>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GBPeriod {
  Today,
  Week,
  Month,
  #[serde(rename = "3month")]
  ThreeMonth,
  #[serde(rename = "6month")]
  SixMonth,
  Year,
  Alltime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBPreviewMedia {
  #[serde(rename = "_aImages", default)]
  pub images: Option<Vec<GBImage>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBLicenseChecklist {
  pub yes: Vec<String>,
  pub ask: Vec<String>,
  pub no: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBEmbeddables {
  #[serde(rename = "_sEmbeddableImageBaseUrl")]
  pub embeddable_image_base_url: String,
  #[serde(rename = "_aVariants")]
  pub variants: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBListMetadata {
  #[serde(rename = "_nRecordCount")]
  pub record_count: i64,
  #[serde(rename = "_nPerpage")]
  pub perpage: i64,
  #[serde(rename = "_bIsComplete")]
  pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModUpdatePreviewMediaMetadata {
  #[serde(rename = "_sSnippet")]
  pub snippet: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModUpdatePreviewMedia {
  #[serde(rename = "_aMetadata")]
  pub metadata: GBModUpdatePreviewMediaMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GBModUpdateAccess {
  #[serde(rename = "Update_Edit")]
  pub update_edit: bool,
  #[serde(rename = "Update_Trash")]
  pub update_trash: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBSubjectShaper {
  #[serde(rename = "_sBorderStyle")]
  pub border_style: String,
  #[serde(rename = "_sFont")]
  pub font: String,
  #[serde(rename = "_sTextColor")]
  pub text_color: String,
  #[serde(rename = "_sTextHoverColor")]
  pub text_hover_color: String,
}

/// Represents a GameBanana ProfilePage API response.
/// This is used to fetch general mod details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBProfilePage {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_nStatus")]
  pub status: String,
  #[serde(rename = "_bIsPrivate")]
  pub is_private: bool,
  #[serde(rename = "_tsDateModified")]
  pub date_modified: i64,
  #[serde(rename = "_tsDateAdded")]
  pub date_added: i64,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_aPreviewMedia")]
  pub preview_media: GBPreviewMedia,
  #[serde(rename = "_sFeedbackInstructions")]
  pub feedback_instructions: Option<String>,
  #[serde(rename = "_sCommentsMode")]
  pub comments_mode: String,
  #[serde(rename = "_bAccessorIsSubmittor")]
  pub accessor_is_submittor: Option<bool>,
  #[serde(rename = "_bIsTrashed")]
  pub is_trashed: bool,
  #[serde(rename = "_bIsWithheld")]
  pub is_withheld: bool,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_nUpdatesCount")]
  pub updates_count: Option<i64>,
  #[serde(rename = "_bHasUpdates")]
  pub has_updates: bool,
  #[serde(rename = "_tsDateUpdated")]
  pub date_updated: Option<i64>,
  #[serde(rename = "_nAllTodosCount")]
  pub all_todos_count: i64,
  #[serde(rename = "_bHasTodos")]
  pub has_todos: bool,
  #[serde(rename = "_nPostCount")]
  pub post_count: Option<i64>,
  #[serde(rename = "_aTags")]
  pub tags: Vec<GBTag>,
  #[serde(rename = "_bCreatedBySubmitter")]
  pub created_by_submitter: Option<bool>,
  #[serde(rename = "_bIsPorted")]
  pub is_ported: Option<bool>,
  #[serde(rename = "_nThanksCount")]
  pub thanks_count: i64,
  #[serde(rename = "_sInitialVisibility")]
  pub initial_visibility: String,
  #[serde(rename = "_sDownloadUrl")]
  pub download_url: String,
  #[serde(rename = "_nDownloadCount")]
  pub download_count: i64,
  #[serde(rename = "_aFiles")]
  pub files: Option<Vec<GBFile>>,
  #[serde(rename = "_nSubscriberCount")]
  pub subscriber_count: i64,
  #[serde(rename = "_aStudio")]
  pub studio: Option<GBStudio>,
  #[serde(rename = "_aContributingStudios")]
  pub contributing_studios: Vec<serde_json::Value>,
  #[serde(rename = "_sLicense")]
  pub license: String,
  #[serde(rename = "_aLicenseChecklist")]
  pub license_checklist: GBLicenseChecklist,
  #[serde(rename = "_aContentRatings")]
  pub content_ratings: Option<serde_json::Value>,
  #[serde(rename = "_aEmbeddedMedia")]
  pub embedded_media: Option<Vec<String>>,
  #[serde(rename = "_sDescription")]
  pub description: Option<String>,
  #[serde(rename = "_bGenerateTableOfContents")]
  pub generate_table_of_contents: bool,
  #[serde(rename = "_sText")]
  pub text: String,
  #[serde(rename = "_bIsObsolete")]
  pub is_obsolete: Option<bool>,
  #[serde(rename = "_nLikeCount")]
  pub like_count: Option<i64>,
  #[serde(rename = "_nViewCount")]
  pub view_count: Option<i64>,
  #[serde(rename = "_sVersion")]
  pub version: Option<String>,
  #[serde(rename = "_bAcceptsDonations")]
  pub accepts_donations: bool,
  #[serde(rename = "_bShowRipePromo")]
  pub show_ripe_promo: bool,
  #[serde(rename = "_aEmbeddables")]
  pub embeddables: GBEmbeddables,
  #[serde(rename = "_aSubmitter")]
  pub submitter: GBSubmitter,
  #[serde(rename = "_bFollowLinks")]
  pub follow_links: bool,
  #[serde(rename = "_aGame")]
  pub game: GBGame,
  #[serde(rename = "_aCategory")]
  pub category: GBCategory,
  #[serde(rename = "_aSuperCategory")]
  pub super_category: Option<GBCategory>,
  #[serde(rename = "_aAlternateFileSources")]
  pub alternate_file_sources: Option<Vec<GBAltFile>>,
  #[serde(rename = "_aCredits")]
  pub credits: Vec<GBCredit>,
  #[serde(rename = "_bAdvancedRequirementsExist")]
  pub advanced_requirements_exist: Option<bool>,
  #[serde(rename = "_aRequirements")]
  pub requirements: Option<Vec<GBRequirement>>,
  #[serde(rename = "_sDevNotes")]
  pub dev_notes: Option<String>,
  #[serde(rename = "_aFeaturings")]
  pub featurings: Option<HashMap<String, GBFeaturing>>,
  // WIP Specific Fields
  #[serde(rename = "_akDevelopmentState")]
  pub small_development_state: Option<String>,
  #[serde(rename = "_sDevelopmentState")]
  pub development_state: Option<String>,
  #[serde(rename = "_iCompletionPercentage")]
  pub completion_percentage: Option<u32>,
  #[serde(rename = "_aFinishedWork")]
  pub finished_work: Option<GBFinishedWork>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBStudio {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sBannerUrl")]
  pub banner_url: String,
  #[serde(rename = "_nSubscriberCount")]
  pub subscriber_count: i64,
  #[serde(rename = "_bAccessorIsSubscribed")]
  pub accessor_is_subscribed: Option<bool>,
  #[serde(rename = "_idAccessorSubscriptionRow")]
  pub id_accessor_subscription_row: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBFinishedWork {
  #[serde(rename = "_aFinishedWorksOnGameBanana")]
  pub finished_works_on_game_banana: Vec<String>,
  #[serde(rename = "_aRemoteFinishedWorkUrls")]
  pub remote_finished_work_urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBFeaturing {
  #[serde(rename = "_sFeatureGroup")]
  pub feature_group: String,
  #[serde(rename = "_sIconClasses")]
  pub icon_classes: String,
  #[serde(rename = "_sTitle")]
  pub title: String,
  #[serde(rename = "_tsDate")]
  pub date: i64,
}

/// Represents a GameBanana DownloadPage API response.
/// This is used to fetch mod download details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBDownloadPage {
  #[serde(rename = "_bIsTrashed")]
  pub is_trashed: bool,
  #[serde(rename = "_bIsWithheld")]
  pub is_withheld: bool,
  #[serde(rename = "_aFiles")]
  pub files: Vec<GBFile>,
  #[serde(rename = "_bAcceptsDonations")]
  pub accepts_donations: bool,
  #[serde(rename = "_bShowRipePromo")]
  pub show_ripe_promo: bool,
  #[serde(rename = "_aAlternateFileSources")]
  pub alternate_file_sources: Vec<GBAltFile>,
  #[serde(rename = "_sLicense")]
  pub license: String,
  #[serde(rename = "_sSubmitterInstructions")]
  pub submitter_instructions: String,
  #[serde(rename = "_aSupportedModManagers")]
  pub supported_mod_managers: HashMap<String, GBModManagerIntegration>,
}

// Represents a Mod from TopSubs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBTopSubsItem {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sModelName")]
  pub model_name: String,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sImageUrl")]
  pub image_url: String,
  #[serde(rename = "_sThumbnailUrl")]
  pub thumbnail_url: String,
  #[serde(rename = "_sInitialVisibility")]
  pub initial_visibility: String,
  #[serde(rename = "_sPeriod")]
  pub period: GBPeriod,
  #[serde(rename = "_aSubmitter")]
  pub submitter: GBMiniSubmitter,
  #[serde(rename = "_nLikeCount")]
  pub like_count: Option<i64>,
  #[serde(rename = "_nPostCount")]
  pub post_count: Option<i64>,
  #[serde(rename = "_aRootCategory")]
  pub root_category: GBMiniCategory,
  #[serde(rename = "_sDescription")]
  pub description: Option<String>,
}

/// Represents a GameBanana Subfeed API response.
/// This is used to fetch submissions in a list.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBSubfeed {
  #[serde(rename = "_aMetadata")]
  pub metadata: GBListMetadata,
  #[serde(rename = "_aRecords")]
  pub records: Vec<GBSubfeedRecord>,
}

// Represents a mod in the Subfeed
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBSubfeedRecord {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sModelName")]
  pub model_name: String,
  #[serde(rename = "_sSingularTitle")]
  pub singular_title: String,
  #[serde(rename = "_sIconClasses")]
  pub icon_classes: String,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_tsDateAdded")]
  pub date_added: i64,
  #[serde(rename = "_tsDateModified")]
  pub date_modified: i64,
  #[serde(rename = "_bHasFiles")]
  pub has_files: bool,
  #[serde(rename = "_aTags")]
  pub tags: Vec<String>,
  #[serde(rename = "_aPreviewMedia")]
  pub preview_media: GBPreviewMedia,
  #[serde(rename = "_aSubmitter")]
  pub submitter: GBMiniSubmitter,
  #[serde(rename = "_aRootCategory")]
  pub root_category: GBMiniCategory,
  #[serde(rename = "_sVersion")]
  pub version: Option<String>,
  #[serde(rename = "_tsDateUpdated")]
  pub date_updated: Option<i64>,
  #[serde(rename = "_bIsObsolete")]
  pub is_obsolete: Option<bool>,
  #[serde(rename = "_sInitialVisibility")]
  pub initial_visibility: String,
  #[serde(rename = "_bHasContentRatings")]
  pub has_content_ratings: bool,
  #[serde(rename = "_nLikeCount")]
  pub like_count: Option<i64>,
  #[serde(rename = "_nPostCount")]
  pub post_count: Option<i64>,
  #[serde(rename = "_bWasFeatured")]
  pub was_featured: bool,
  #[serde(rename = "_nViewCount")]
  pub view_count: Option<i64>,
  #[serde(rename = "_bIsOwnedByAccessor")]
  pub is_owned_by_accessor: bool,
}

/// Represents a GameBanana Posts API response.
/// This is used to fetch mod posts.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModPosts {
  #[serde(rename = "_aMetadata")]
  pub metadata: GBListMetadata,
  #[serde(rename = "_aRecords")]
  pub records: Vec<GBModPost>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GBPreviewMediaField {
  EmptyArray(Vec<serde_json::Value>),
  Object(GBPreviewMedia),
}
// Represents a post on a mod
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModPost {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_nStatus")]
  pub status: String,
  #[serde(rename = "_tsDateAdded")]
  pub date_added: i64,
  #[serde(rename = "_tsDateModified")]
  pub date_modified: i64,
  #[serde(rename = "_nReplyCount")]
  pub reply_count: i64,
  #[serde(rename = "_iPinLevel")]
  pub pin_level: i64,
  #[serde(rename = "_nStampScore")]
  pub stamp_score: i64,
  #[serde(rename = "_aPreviewMedia", default)]
  pub preview_media: Option<GBPreviewMediaField>,
  #[serde(rename = "_sText")]
  pub text: String,
  #[serde(rename = "_aPoster")]
  pub poster: Option<GBSubmitter>,
  #[serde(rename = "_bFollowLinks")]
  pub follow_links: Option<bool>,
  #[serde(rename = "_aStamps")]
  pub stamps: Option<Vec<GBStamp>>,
  #[serde(rename = "_aLabels")]
  pub labels: Option<Vec<String>>,
}

/// Represents a GameBanana Updates API response.
/// This is used to fetch submitted mod updates.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModUpdates {
  #[serde(rename = "_aMetadata")]
  pub metadata: GBListMetadata,
  #[serde(rename = "_aRecords")]
  pub records: Vec<GBModUpdate>,
}

// Represents a mod update in GameBanana
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModUpdate {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_nStatus")]
  pub status: String,
  #[serde(rename = "_bIsPrivate")]
  pub is_private: bool,
  #[serde(rename = "_tsDateAdded")]
  pub date_added: i64,
  #[serde(rename = "_tsDateModified")]
  pub date_modified: i64,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_aPreviewMedia")]
  pub preview_media: GBModUpdatePreviewMedia,
  #[serde(rename = "_bAccessorIsSubmittor")]
  pub accessor_is_submittor: Option<bool>,
  #[serde(rename = "_bIsTrashed")]
  pub is_trashed: bool,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_nPostCount")]
  pub post_count: Option<i64>,
  #[serde(rename = "_sInitialVisibility")]
  pub initial_visibility: String,
  #[serde(rename = "_bHasFiles")]
  pub has_files: bool,
  #[serde(rename = "_sText")]
  pub text: String,
  #[serde(rename = "_bShowRipePromo")]
  pub show_ripe_promo: bool,
  #[serde(rename = "_aSubmitter")]
  pub submitter: GBSubmitter,
  #[serde(rename = "_bFollowLinks")]
  pub follow_links: bool,
  #[serde(rename = "_sVersion")]
  pub version: Option<String>,
  #[serde(rename = "_aChangeLog")]
  pub change_log: Option<Vec<GBChangeLogEntry>>,
  #[serde(rename = "_aSubmission")]
  pub submission: GBMiniSubmission,
  #[serde(rename = "_aAccess")]
  pub access: GBModUpdateAccess,
}

// Reduced version of the normal submission, used in some places
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBMiniSubmission {
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sModelName")]
  pub model_name: String,
  #[serde(rename = "_sDownloadUrl")]
  pub download_url: String,
}

// Represents a change log entry in GameBanana updates
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBChangeLogEntry {
  pub text: String,
  pub cat: String,
}

// Represents a GameBanana tag
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBTag {
  #[serde(rename = "_sTitle")]
  pub title: String,
  #[serde(rename = "_sValue")]
  pub value: String,
}

// Represents a GameBanana stamp
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBStamp {
  #[serde(rename = "_sIconClasses")]
  pub icon_classes: Option<String>,
  #[serde(rename = "_sTitle")]
  pub title: Option<String>,
  #[serde(rename = "_sCategory")]
  pub category: Option<String>,
  #[serde(rename = "_nCount")]
  pub count: i64,
  #[serde(rename = "_sUnlockName")]
  pub unlock_name: Option<String>,
}

// Represents a GameBanana submitter (author) profile
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBSubmitter {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sUserTitle")]
  pub user_title: String,
  #[serde(rename = "_sHonoraryTitle")]
  pub honorary_title: String,
  #[serde(rename = "_tsJoinDate")]
  pub join_date: i64,
  #[serde(rename = "_sAvatarUrl")]
  pub avatar_url: String,
  #[serde(rename = "_sSigUrl")]
  pub sig_url: Option<String>,
  #[serde(rename = "_sHdAvatarUrl")]
  pub hd_avatar_url: Option<String>,
  #[serde(rename = "_sUpicUrl")]
  pub upic_url: Option<String>,
  #[serde(rename = "_aSubjectShaper")]
  pub subject_shaper: Option<GBSubjectShaper>,
  #[serde(rename = "_sSubjectShaperCssCode")]
  pub subject_shaper_css_code: Option<String>,
  #[serde(rename = "_sHovatarUrl")]
  pub hovatar_url: Option<String>,
  #[serde(rename = "_sMoreByUrl")]
  pub more_by_url: Option<String>,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sPointsUrl")]
  pub points_url: Option<String>,
  #[serde(rename = "_sMedalsUrl")]
  pub medals_url: Option<String>,
  #[serde(rename = "_bIsOnline")]
  pub is_online: bool,
  #[serde(rename = "_sLocation")]
  pub location: Option<String>,
  #[serde(rename = "_sOnlineTitle")]
  pub online_title: Option<String>,
  #[serde(rename = "_sOfflineTitle")]
  pub offline_title: Option<String>,
  #[serde(rename = "_nPoints")]
  pub points: Option<i64>,
  #[serde(rename = "_nPointsRank")]
  pub points_rank: Option<i64>,
  #[serde(rename = "_aNormalMedals")]
  pub normal_medals: Option<Vec<GBMedal>>,
  #[serde(rename = "_aRareMedals")]
  pub rare_medals: Option<Vec<GBMedal>>,
  #[serde(rename = "_aLegendaryMedals")]
  pub legendary_medals: Option<Vec<GBMedal>>,
  #[serde(rename = "_bHasRipe")]
  pub has_ripe: bool,
  #[serde(rename = "_nBuddyCount")]
  pub buddy_count: Option<i64>,
  #[serde(rename = "_nSubscriberCount")]
  pub subscriber_count: Option<i64>,
  #[serde(rename = "_aDonationMethods")]
  pub donation_methods: Option<Vec<GBDonationMethod>>,
  #[serde(rename = "_bAccessorIsBuddy")]
  pub accessor_is_buddy: Option<bool>,
  #[serde(rename = "_bBuddyRequestExistsWithAccessor")]
  pub buddy_request_exists_with_accessor: Option<bool>,
  #[serde(rename = "_bAccessorIsSubscribed")]
  pub accessor_is_subscribed: Option<bool>,
  #[serde(rename = "_aBio")]
  pub bio: Option<Vec<GBBioItem>>,
  #[serde(rename = "_aClearanceLevels")]
  pub clearance_levels: Option<Vec<String>>,
}

// Reduced version of a normal submitter, used in some places
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBMiniSubmitter {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_bIsOnline")]
  pub is_online: bool,
  #[serde(rename = "_bHasRipe")]
  pub has_ripe: bool,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sAvatarUrl")]
  pub avatar_url: String,
  #[serde(rename = "_sHdAvatarUrl")]
  pub hd_avatar_url: Option<String>,
  #[serde(rename = "_sUpicUrl")]
  pub upic_url: Option<String>,
  #[serde(rename = "_sSubjectShaperCssCode")]
  pub subject_shaper_css_code: Option<String>,
  #[serde(rename = "_sHovatarUrl")]
  pub hovatar_url: Option<String>,
  #[serde(rename = "_sMoreByUrl")]
  pub more_by_url: Option<String>,
  #[serde(rename = "_aClearanceLevels")]
  pub clearance_levels: Option<Vec<String>>,
}

// Represents a validator for a donation method
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBDonationMethodValidator {
  #[serde(rename = "_regexValidPattern")]
  pub regex_valid_pattern: String,
  #[serde(rename = "_sWarningMessage")]
  pub warning_message: String,
}

// Represents a donation method in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBDonationMethod {
  #[serde(rename = "_aValidator")]
  pub validator: Option<GBDonationMethodValidator>,
  #[serde(rename = "_bIsUrl")]
  pub is_url: bool,
  #[serde(rename = "_sFormattedValue")]
  pub formatted_value: Option<String>,
  #[serde(rename = "_sIconClasses")]
  pub icon_classes: String,
  #[serde(rename = "_sInputPlaceholder")]
  pub input_placeholder: String,
  #[serde(rename = "_sInputType")]
  pub input_type: String,
  #[serde(rename = "_sTitle")]
  pub title: String,
  #[serde(rename = "_sValue")]
  pub value: String,
  #[serde(rename = "_sValueTemplate")]
  pub value_template: Option<String>,
}

// Represents a game in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBGame {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sAbbreviation")]
  pub abbreviation: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sIconUrl")]
  pub icon_url: String,
  #[serde(rename = "_sBannerUrl")]
  pub banner_url: String,
  #[serde(rename = "_nSubscriberCount")]
  pub subscriber_count: i64,
  #[serde(rename = "_bHasSubmissionQueue")]
  pub has_submission_queue: bool,
  #[serde(rename = "_bAccessorIsSubscribed")]
  pub accessor_is_subscribed: bool,
}

// Represents an image in _aImages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBImage {
  #[serde(rename = "_sType")]
  pub r#type: Option<String>,
  #[serde(rename = "_sBaseUrl")]
  pub base_url: String,
  #[serde(rename = "_sCaption")]
  pub caption: Option<String>,
  #[serde(rename = "_sFile")]
  pub file: String,
  #[serde(rename = "_sFile220")]
  pub file_220: Option<String>,
  #[serde(rename = "_sFile530")]
  pub file_530: Option<String>,
  #[serde(rename = "_sFile100")]
  pub file_100: Option<String>,
  #[serde(rename = "_hFile220")]
  pub h_file_220: Option<i64>,
  #[serde(rename = "_wFile220")]
  pub w_file_220: Option<i64>,
  #[serde(rename = "_hFile530")]
  pub h_file_530: Option<i64>,
  #[serde(rename = "_wFile530")]
  pub w_file_530: Option<i64>,
  #[serde(rename = "_hFile100")]
  pub h_file_100: Option<i64>,
  #[serde(rename = "_wFile100")]
  pub w_file_100: Option<i64>,
}

// Represents a file in _aFiles
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBFile {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sFile")]
  pub file: String,
  #[serde(rename = "_nFilesize")]
  pub filesize: i64,
  #[serde(rename = "_tsDateAdded")]
  pub date_added: i64,
  #[serde(rename = "_nDownloadCount")]
  pub download_count: i64,
  #[serde(rename = "_sDownloadUrl")]
  pub download_url: String,
  #[serde(rename = "_sMd5Checksum")]
  pub md5_checksum: String,
  #[serde(rename = "_sAnalysisState")]
  pub analysis_state: String,
  #[serde(rename = "_sAnalysisResult")]
  pub analysis_result: String,
  #[serde(rename = "_sAnalysisResultVerbose")]
  pub analysis_result_verbose: String,
  #[serde(rename = "_sAvastAvState")]
  pub avast_av_state: String,
  #[serde(rename = "_sAvastAvResult")]
  pub avast_av_result: String,
  #[serde(rename = "_bHasContents")]
  pub has_contents: bool,
  #[serde(rename = "_sDescription")]
  pub description: Option<String>,
  #[serde(rename = "_aAnalysisWarnings")]
  pub analysis_warnings: Option<GBAnalysisWarnings>,
  #[serde(rename = "_aModManagerIntegrations")]
  pub mod_manager_integrations: Option<Vec<GBModManagerIntegration>>,
}

// Represents analysis warnings for a file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBAnalysisWarnings {
  pub contains_exe: Option<Vec<String>>,
  pub nested_archives: Option<Vec<String>>,
}

// Represents a mod manager integration item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBModManagerIntegration {
  #[serde(rename = "_sIconClasses")]
  pub icon_classes: Option<String>,
  #[serde(rename = "_idToolRow")]
  pub tool_row_id: i64,
  #[serde(rename = "_aGameRowIds")]
  pub game_row_ids: Vec<i64>,
  #[serde(rename = "_sInstallerName")]
  pub installer_name: String,
  #[serde(rename = "_sInstallerUrl")]
  pub installer_url: String,
  #[serde(rename = "_sIconUrl")]
  pub icon_url: String,
  #[serde(rename = "_sDownloadUrl")]
  pub download_url: String,
}

// Represents an alternate file source in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBAltFile {
  pub url: String,
  pub description: Option<String>,
}

// Represents a category in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBCategory {
  #[serde(rename = "_idRow")]
  pub id_row: i64,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sModelName")]
  pub model_name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sIconUrl")]
  pub icon_url: String,
}

// Reduced version of a category, used in some places
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBMiniCategory {
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sIconUrl")]
  pub icon_url: String,
}

// Represents a bio item in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBBioItem {
  #[serde(rename = "_sTitle")]
  pub title: String,
  #[serde(rename = "_sValue")]
  pub value: String,
}

// Represents a credit group in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBCredit {
  #[serde(rename = "_sGroupName")]
  pub group_name: String,
  #[serde(rename = "_aAuthors")]
  pub authors: Vec<GBCreditAuthor>,
}

// Represents an author in GameBanana API credit
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBCreditAuthor {
  #[serde(rename = "_sRole")]
  pub role: Option<String>,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: Option<String>,
  #[serde(rename = "_sUrl")]
  pub url: Option<String>,
  #[serde(rename = "_bIsOnline")]
  pub is_online: Option<bool>,
  #[serde(rename = "_sUpicUrl")]
  pub upic_url: Option<String>,
  #[serde(rename = "_aAffiliatedStudio")]
  pub affiliated_studio: Option<GBAffiliatedStudio>,
}

// Represents an affiliated studio in GameBanana API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GBAffiliatedStudio {
  #[serde(rename = "_sBannerUrl")]
  pub banner_url: String,
  #[serde(rename = "_sName")]
  pub name: String,
  #[serde(rename = "_sProfileUrl")]
  pub profile_url: String,
  #[serde(rename = "_sFlagUrl")]
  pub flag_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Engine {
  pub engine_type: Option<String>,
  pub engine_name: Option<String>,
  pub engine_icon: Option<String>, // Base64 encoded engine icon
  pub mods_folder: Option<bool>,
  pub mods_folder_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Contributor {
  pub name: String,
  pub icon: Option<String>, // Path to contributor icon or base64 data
  pub role: Option<String>, // Role description of the contributor
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContributorGroup {
  pub group: String,
  pub members: Vec<Contributor>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModInfo {
  pub id: String,
  pub name: String,
  pub path: String,
  pub metadata_version: Option<u32>,
  pub description: Option<String>,
  pub executable_path: Option<String>,
  pub icon_data: Option<String>, // Base64 encoded icon data
  pub banner_data: Option<String>, // Base64 encoded banner image data
  pub logo_data: Option<String>, // Base64 encoded logo image data
  pub logo_position: Option<String>, // Position of logo in banner: left_bottom, left_middle, middle
  pub version: Option<String>,
  pub engine: Option<Engine>, // Extended engine information
  pub display_order: Option<i64>,
  pub process_id: Option<u32>, // Track the running process ID
  pub contributors: Option<Vec<ContributorGroup>>, // List of contributor groups
  pub last_played: Option<i64>, // Unix timestamp when mod was last played
  pub date_added: Option<i64>, // Unix timestamp when mod was added
  pub gamebanana: Option<ModInfoGBData>, // GameBanana mod data
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModInfoGBData {
  pub url: String,
  pub id: i64,
  pub model_type: String,
}

impl ModInfo {
  pub fn validate_metadata_version(&self) -> Result<(), MetadataError> {
    match self.metadata_version {
      None => Err(MetadataError::MissingVersion),
      Some(version) if version < MIN_METADATA_VERSION => {
        Err(MetadataError::VersionTooLow {
          provided: version,
          minimum: MIN_METADATA_VERSION,
        })
      }
      Some(_) => Ok(()),
    }
  }
}

// Define a structure for mod metadata files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModMetadataFile {
  pub name: String,
  pub description: Option<String>,
  pub folder_path: String,
  pub config_file_path: Option<String>,
  pub icon_file_path: Option<String>,
  pub icon_data: Option<String>, // Base64 encoded icon data
  pub enabled: Option<bool>, // Whether the mod is enabled
  pub version: Option<String>, // Version of the mod
  pub homepage: Option<String>, // Homepage/website URL
  pub contributors: Option<Vec<ContributorMetadata>>,
  pub license: Option<String>, // License information
  pub restart_required: Option<bool>, // Whether a restart is required
  pub dependencies: Option<HashMap<String, String>>, // Dependencies of the mod with version requirements
  pub parent_mod_id: Option<String>, // ID of the parent mod this metadata belongs to
}

// Contributor structure for mod metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContributorMetadata {
  pub name: String,
  pub role: String,
  pub email: Option<String>,
  pub url: Option<String>,
}

// Response structure for mod disable/enable operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModDisableResult {
  pub success: bool,
  pub enabled: bool,
  pub message: String,
}

// Structure for the engine mods response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineModsResponse {
  pub engine_type: String,
  pub executable_path: String,
  pub mods: Vec<ModMetadataFile>,
}

// Create a state to manage our mods
pub struct ModsState(pub Mutex<HashMap<String, ModInfo>>);

// Event structures for download progress tracking
#[derive(Clone, Serialize)]
pub struct DownloadStarted {
  pub mod_id: i64,
  pub name: String,
  pub content_length: usize,
  pub thumbnail_url: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct DownloadProgress {
  pub mod_id: i64,
  pub name: String,
  pub bytes_downloaded: usize,
  pub total_bytes: usize,
  pub percentage: u8,
  pub step: String,
}

#[derive(Clone, Serialize)]
pub struct DownloadFinished {
  pub mod_id: i64,
  pub name: String,
  pub mod_info: ModInfo,
}

#[derive(Clone, Serialize)]
pub struct DownloadError {
  pub mod_id: i64,
  pub name: String,
  pub error: String,
}

// Create a state to manage terminal output for each running mod
pub struct TerminalOutputState(pub Mutex<HashMap<String, String>>);

// Global thread-safe instance of ModsState for use across threads
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref GLOBAL_MODS_STATE: Arc<Mutex<HashMap<String, ModInfo>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Global app handle that can be accessed from anywhere, thread-safe
lazy_static! {
  pub static ref GLOBAL_APP_HANDLE: Mutex<Option<tauri::AppHandle>> =
    Mutex::new(None);
}
