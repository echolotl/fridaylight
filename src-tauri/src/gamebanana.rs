use crate::models::{GameBananaMod, GameBananaModImage, GameBananaResponse};
use log::{debug, error, info, warn};
use reqwest;
use serde_json;

// Command to fetch mods from GameBanana
pub async fn fetch_gamebanana_mods(query: String, page: i64) -> Result<GameBananaResponse, String> {
    info!("Fetching GameBanana mods. Query: '{}', Page: {}", query, page);
    let client = reqwest::Client::new();

    // Base URL for GameBanana's API
    let url = if query == "featured" {
        // Featured mods endpoint
        format!("https://gamebanana.com/apiv11/Game/8694/TopSubs")
    } else {
        if query == "latest" {
            format!("https://gamebanana.com/apiv11/Mod/Index?_nPerpage=15&_aFilters[Generic_Category]=3827&_nPage={}", page)
        } else if query == "_psychmodpack" {
            // Psych Engine modpack (mods folder) endpoint
            format!("https://gamebanana.com/apiv11/Mod/Index?_nPerpage=15&_aFilters[Generic_Category]=28367&_nPage={}", page)

        } else if query == "_codenamemodpack" {
            // Codename Engine modpack (mods folder) endpoint
            format!("https://gamebanana.com/apiv11/Mod/Index?_nPerpage=15&_aFilters%5BGeneric_Category%5D=34764&_nPage={}", page)
        } else if query == "_vslicemodpack" {
            // V-Slice modpack (mods folder) endpoint
            format!("https://gamebanana.com/apiv11/Mod/Index?_nPerpage=15&_aFilters%5BGeneric_Category%5D=29202&_nPage={}", page)
        } else {
            // Search endpoint
            format!("https://gamebanana.com/apiv11/Game/8694/Subfeed?_sSort=default&_sName={}&_nPage={}", query, page)
        }
    };
    
    debug!("Requesting URL: {}", url);
    
    // Make the API request
    let response = match client.get(&url).send().await {
        Ok(resp) => {
            debug!("Received response with status: {}", resp.status());
            if !resp.status().is_success() {
                let status = resp.status();
                let error_text = match resp.text().await {
                    Ok(text) => format!("Server returned error status: {} with body: {}", status, text),
                    Err(_) => format!("Server returned error status: {}", status),
                };
                error!("{}", error_text);
                return Err(error_text);
            }
            resp
        },
        Err(e) => {
            error!("Failed to fetch mods: {}", e);
            return Err(format!("Failed to fetch mods: {}", e));
        },
    };
      // Try to parse response as array first 
    let mods_array: Result<Vec<serde_json::Value>, _> = response.json().await;
    
    let (mods_data, total_count) = match mods_array {
        Ok(array) => {
            debug!("Successfully parsed API response as array of {} mods", array.len());
            // For direct arrays, we only know the number of mods in this page
            (array, None)
        },
        Err(_) => {
            // Try the old format with _aRecords
            debug!("Failed to parse as array, trying to parse as object with _aRecords");
            
            // Reset the response and try again
            let response = match client.get(&url).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    error!("Failed to fetch mods on retry: {}", e);
                    return Err(format!("Failed to fetch mods: {}", e));
                }
            };
            
            // Parse as object
            let api_data: Result<serde_json::Value, _> = response.json().await;
            
            match api_data {
                Ok(data) => {
                    // Extract total record count from _aMetadata if available
                    let total_count = data.get("_aMetadata")
                        .and_then(|metadata| metadata.get("_nRecordCount"))
                        .and_then(|count| count.as_i64());
                    
                    if let Some(count) = total_count {
                        debug!("Found _nRecordCount in metadata: {}", count);
                    }
                    
                    // Extract records from _aRecords if it exists
                    if let Some(records) = data.get("_aRecords").and_then(|r| r.as_array()) {
                        debug!("Found _aRecords with {} items", records.len());
                        (records.clone(), total_count)
                    } else {
                        // If neither format works, return empty array
                        error!("Invalid API response format: {}", data);
                        return Err(format!("Invalid API response format: {}", data));
                    }
                },
                Err(e) => {
                    error!("Failed to parse response as object: {}", e);
                    return Err(format!("Failed to parse response: {}", e));
                }
            }
        }
    };
    
    // Use the total_count from metadata if available, otherwise use the length of the data we got
    let total = total_count.unwrap_or(mods_data.len() as i64);
    debug!("Total mods found: {}", total);
    
    // Map the API data to GameBananaMod struct
    let mut mods = Vec::new();
    
    for record in &mods_data {
        let id = record.get("_idRow").and_then(|v| v.as_i64()).unwrap_or(0);
        let name = record.get("_sName").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
        
        // Extract owner info
        let owner = match record.get("_aSubmitter") {
            Some(submitter) => submitter.get("_sName").and_then(|v| v.as_str()).unwrap_or("Unknown"),
            None => "Unknown",
        }.to_string();
        
        // Extract statistics
        let views = record.get("_nViews").and_then(|v| v.as_i64()).unwrap_or(0);
        let downloads = record.get("_nDownloadCount").and_then(|v| v.as_i64()).unwrap_or(0);
        // We'll use the likes value directly in the struct creation
        
        // Try both methods of getting thumbnails
        let thumbnail_url = if let Some(url) = record.get("_sThumbnailUrl").and_then(|v| v.as_str()) {
            // New format has _sThumbnailUrl directly
            Some(url.to_string())
        } else {
            // Old format had nested _aPreviewMedia._aImages
            match record.get("_aPreviewMedia") {
                Some(media) => {
                    match media.get("_aImages") {
                        Some(images) => {
                            if let Some(images_array) = images.as_array() {
                                if !images_array.is_empty() {
                                    if let Some(image) = images_array.first() {
                                        let base_url = image.get("_sBaseUrl").and_then(|v| v.as_str()).unwrap_or("");
                                        
                                        // Try to get the original file first, then fall back to size variants
                                        let file = image.get("_sFile")
                                            .or_else(|| image.get("_sFile800"))
                                            .or_else(|| image.get("_sFile530"))
                                            .or_else(|| image.get("_sFile220"))
                                            .or_else(|| image.get("_sFile100"))
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");
                                        
                                        if !base_url.is_empty() && !file.is_empty() {
                                            Some(format!("{}/{}", base_url, file))
                                        } else {
                                            None
                                        }
                                    } else {
                                        Some("".to_string())
                                    }
                                } else {
                                    Some("".to_string())
                                }
                            } else {
                                Some("".to_string())
                            }
                        },
                        None => Some("".to_string()),
                    }
                },
                None => Some("".to_string()),
            }
        };
        
        // Extract download info
        let download_url = format!("https://gamebanana.com/mods/download/{}", id);
        
        // Extract description
        let description = record.get("_sDescription").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        debug!("Processed mod: {} (ID: {})", name, id);
        
        mods.push(GameBananaMod {
            // Existing fields
            id,
            name,
            owner: owner.clone(),
            description,
            thumbnailUrl: if let Some(url) = thumbnail_url {
                if url.is_empty() { 
                    "https://gamebanana.com/img/default/game.png".to_string() 
                } else { 
                    url 
                }
            } else {
                "https://gamebanana.com/img/default/game.png".to_string()
            },
            downloadUrl: download_url,
            views,
            downloads,
            likes: record.get("_nLikeCount").and_then(|v| v.as_i64()).unwrap_or(0),
            
            // New fields
            modelName: record.get("_sModelName").and_then(|v| v.as_str()).unwrap_or("Mod").to_string(),
            profileUrl: record.get("_sProfileUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            imageUrl: record.get("_sImageUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            initialVisibility: record.get("_sInitialVisibility").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            period: record.get("_sPeriod").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            
            // Submitter details
            submitterId: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_idRow").and_then(|v| v.as_i64()).unwrap_or(0),
                None => 0,
            },
            submitterName: owner, // Reuse the owner we already extracted
            submitterIsOnline: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_bIsOnline").and_then(|v| v.as_bool()).unwrap_or(false),
                None => false,
            },
            submitterHasRipe: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_bHasRipe").and_then(|v| v.as_bool()).unwrap_or(false),
                None => false,
            },
            submitterProfileUrl: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_sProfileUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            submitterAvatarUrl: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_sAvatarUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            submitterMoreByUrl: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_sMoreByUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            submitterUPic: match record.get("_aSubmitter") {
                Some(submitter) => submitter.get("_sUpicUrl").and_then(|v| v.as_str()).map(|s| s.to_string()),
                None => None,
            },
            // Post count
            postCount: record.get("_nPostCount").and_then(|v| v.as_i64()).unwrap_or(0),
            
            // Category details
            categoryName: match record.get("_aRootCategory") {
                Some(category) => category.get("_sName").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            categoryProfileUrl: match record.get("_aRootCategory") {
                Some(category) => category.get("_sProfileUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            categoryIconUrl: match record.get("_aRootCategory") {
                Some(category) => category.get("_sIconUrl").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                None => "".to_string(),
            },
            
            // Additional fields from normal mod data
            singularTitle: record.get("_sSingularTitle").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            iconClasses: record.get("_sIconClasses").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            dateAdded: record.get("_nDateAdded").and_then(|v| v.as_i64()).unwrap_or(0),
            dateModified: record.get("_nDateModified").and_then(|v| v.as_i64()).unwrap_or(0),
            dateUpdated: record.get("_nDateUpdated").and_then(|v| v.as_i64()).unwrap_or(0),
            hasFiles: record.get("_bHasFiles").and_then(|v| v.as_bool()).unwrap_or(false),
            tags: record.get("_aTags")
                .and_then(|v| v.as_array()) // Option<&Vec<Value>>
                .map(|tags_array| {
                    tags_array.iter()
                        .filter_map(|tag_value| tag_value.as_str()) // Iterator<&str>
                        .map(|s| s.to_string()) // Iterator<String>
                        .collect::<Vec<String>>() // Vec<String>
                }) // Option<Vec<String>>
                .unwrap_or_else(Vec::new), // Vec<String>
            previewImages: record.get("_aPreviewImages").and_then(|v| v.as_array()).map(|images| {
                images.iter().filter_map(|image| {
                    let image_type = image.get("_sType").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let base_url = image.get("_sBaseUrl").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let file_name = image.get("_sFileName").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let file100 = image.get("_sFile100").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let file220 = image.get("_sFile220").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let file530 = image.get("_sFile530").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let file800 = image.get("_sFile800").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let height100 = image.get("_nHeight100").and_then(|v| v.as_i64());
                    let width100 = image.get("_nWidth100").and_then(|v| v.as_i64());
                    let height220 = image.get("_nHeight220").and_then(|v| v.as_i64());
                    let width220 = image.get("_nWidth220").and_then(|v| v.as_i64());
                    let height530 = image.get("_nHeight530").and_then(|v| v.as_i64());
                    let width530 = image.get("_nWidth530").and_then(|v| v.as_i64());
                    let height800 = image.get("_nHeight800").and_then(|v| v.as_i64());
                    let width800 = image.get("_nWidth800").and_then(|v| v.as_i64());
                    
                    Some(GameBananaModImage {
                        imageType: image_type,
                        baseUrl: base_url,
                        fileName: file_name,
                        file100,
                        file220,
                        file530,
                        file800,
                        height100,
                        width100,
                        height220,
                        width220,
                        height530,
                        width530,
                        height800,
                        width800,
                    })
                }).collect()
            }).unwrap_or_else(Vec::new),
            version: record.get("_sVersion").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            isObsolete: record.get("_bIsObsolete").and_then(|v| v.as_bool()).unwrap_or(false),
            hasContentRatings: record.get("_bHasContentRatings").and_then(|v| v.as_bool()).unwrap_or(false),
            viewCount: record.get("_nViewCount").and_then(|v| v.as_i64()).unwrap_or(0),
            isOwnedByAccessor: record.get("_bIsOwnedByAccessor").and_then(|v| v.as_bool()).unwrap_or(false),
            wasFeatured: record.get("_bWasFeatured").and_then(|v| v.as_bool()).unwrap_or(false),
        });
    }
    
    info!("Fetched {} mods from GameBanana", mods.len());
    
    Ok(GameBananaResponse {
        mods,
        total,
    })
}

// Function to get mod information from GameBanana
pub async fn get_mod_info(mod_id: i64) -> Result<serde_json::Value, String> {
    let mod_info_url = format!("https://gamebanana.com/apiv11/Mod/{}/ProfilePage", mod_id);
    debug!("Fetching mod info from: {}", mod_info_url);
    
    let client = reqwest::Client::new();
    match client.get(&mod_info_url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                warn!("Failed to fetch mod info, status: {}", resp.status());
                return Err(format!("Failed to fetch mod info, status: {}", resp.status()));
            } 
            
            match resp.json::<serde_json::Value>().await {
                Ok(data) => Ok(data),
                Err(e) => {
                    warn!("Failed to parse mod info: {}", e);
                    Err(format!("Failed to parse mod info: {}", e))
                }
            }
        },
        Err(e) => {
            warn!("Failed to fetch mod info: {}", e);
            Err(format!("Failed to fetch mod info: {}", e))
        }
    }
}

// Function to get download information from GameBanana
pub async fn get_download_url(mod_id: i64) -> Result<String, String> {
    let download_page_url = format!("https://gamebanana.com/apiv11/Mod/{}/DownloadPage", mod_id);
    debug!("Download page API URL: {}", download_page_url);
    
    let client = reqwest::Client::new();
    let download_page_response = match client.get(&download_page_url).send().await {
        Ok(resp) => {
            debug!("Received download page response with status: {}", resp.status());
            if !resp.status().is_success() {
                let status = resp.status();
                let error_text = format!("Server returned error status: {}", status);
                error!("{}", error_text);
                return Err(error_text);
            }
            resp
        },
        Err(e) => {
            let error_msg = format!("Failed to fetch download page: {}", e);
            error!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // Parse the download page JSON
    let download_page: serde_json::Value = match download_page_response.json().await {
        Ok(data) => data,
        Err(e) => {
            let error_msg = format!("Failed to parse download page response: {}", e);
            error!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // Extract the first file's download URL as default behavior
    match download_page.get("_aFiles")
        .and_then(|files| files.as_array())
        .and_then(|files_array| {
            if !files_array.is_empty() {
                files_array.get(0)
                    .and_then(|file| file.get("_sDownloadUrl"))
                    .and_then(|url| url.as_str())
            } else {
                None
            }
        }) {
        Some(url) => Ok(url.to_string()),
        None => {
            let err_msg = "No download URLs found in the download page";
            error!("{}", err_msg);
            Err(err_msg.to_string())
        }
    }
}

// Function to get all download files information for a mod
pub async fn get_mod_download_files(mod_id: i64) -> Result<serde_json::Value, String> {
    let download_page_url = format!("https://gamebanana.com/apiv11/Mod/{}/DownloadPage", mod_id);
    debug!("Download page API URL: {}", download_page_url);
    
    let client = reqwest::Client::new();
    let download_page_response = match client.get(&download_page_url).send().await {
        Ok(resp) => {
            debug!("Received download page response with status: {}", resp.status());
            if !resp.status().is_success() {
                let status = resp.status();
                let error_text = format!("Server returned error status: {}", status);
                error!("{}", error_text);
                return Err(error_text);
            }
            resp
        },
        Err(e) => {
            let error_msg = format!("Failed to fetch download page: {}", e);
            error!("{}", error_msg);
            return Err(error_msg);
        }
    };
    
    // Parse the download page JSON
    match download_page_response.json().await {
        Ok(data) => Ok(data),
        Err(e) => {
            let error_msg = format!("Failed to parse download page response: {}", e);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

// Function to extract the first banner image URL from mod info
pub fn extract_banner_url(mod_info: &serde_json::Value, mod_id: i64) -> Option<String> {
    mod_info
        .get("_aPreviewMedia")
        .and_then(|media| media.get("_aImages"))
        .and_then(|images| images.as_array())
        .and_then(|images_array| {
            if !images_array.is_empty() {
                images_array.first().and_then(|image| {
                    let base_url = image.get("_sBaseUrl").and_then(|v| v.as_str()).unwrap_or("");
                    
                    // Try to get the highest quality image available
                    let file = image.get("_sFile800")
                        .or_else(|| image.get("_sFile530"))
                        .or_else(|| image.get("_sFile220"))
                        .or_else(|| image.get("_sFile100"))
                        .or_else(|| image.get("_sFile"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    if !base_url.is_empty() && !file.is_empty() {
                        Some(format!("{}/{}", base_url, file))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .or_else(|| {
            // Fallback to the embeddable thumbnail
            Some(format!("https://gamebanana.com/mods/embeddables/{}", mod_id))
        })
}

// Function to extract contributors from mod info
pub fn extract_contributors(mod_info: &serde_json::Value) -> Option<Vec<crate::models::Contributor>> {
    // Check if _aCredits exists
    if let Some(credits) = mod_info.get("_aCredits") {
        if let Some(credits_array) = credits.as_array() {
            // If there are no credits, return None
            if credits_array.is_empty() {
                return None;
            }

            let mut contributors = Vec::new();

            // Process each credit group
            for credit_group in credits_array {
                // Get the group name (which will be used as the role/title)
                let group_name = credit_group
                    .get("_sGroupName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contributor")
                    .to_string();

                // Process authors in this group
                if let Some(authors) = credit_group.get("_aAuthors").and_then(|v| v.as_array()) {
                    for author in authors {
                        // Get author name
                        let name = author
                            .get("_sName")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        // Get author role if available, otherwise use group name
                        let title = group_name.clone();
                        // Add the contributor
                        contributors.push(crate::models::Contributor {
                            name,
                            title: Some(title),
                            icon: None, // GameBanana API doesn't provide author icons in this context
                        });
                    }
                }
            }

            // Return only if we found any contributors
            if !contributors.is_empty() {
                debug!("Extracted {} contributors from mod info", contributors.len());
                return Some(contributors);
            }
        }
    }
    
    None
}