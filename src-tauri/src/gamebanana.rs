use crate::models::{
  GBDownloadPage,
  GBModPosts,
  GBModUpdates,
  GBProfilePage,
  GBSubfeed,
  GBTopSubs,
};
use log::{ debug, error, info, warn };
use reqwest;
use serde_json;

// Command to fetch mods from GameBanana
pub async fn fetch_gamebanana_mods(
  query: String,
  page: i64
) -> Result<GBSubfeed, String> {
  info!("Fetching GameBanana mods. Query: '{}', Page: {}", query, page);
  let client = reqwest::Client::new();

  // Base URL for GameBanana's API
  let url = if query == "latest" {
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
    format!(
      "https://gamebanana.com/apiv11/Game/8694/Subfeed?_sSort=default&_sName={}&_nPage={}",
      query,
      page
    )
  };

  debug!("Requesting URL: {}", url);

  // Make the API request
  let response = match client.get(&url).send().await {
    Ok(resp) => {
      debug!("Received response with status: {}", resp.status());
      if !resp.status().is_success() {
        let status = resp.status();
        let error_text = match resp.text().await {
          Ok(text) =>
            format!(
              "Server returned error status: {} with body: {}",
              status,
              text
            ),
          Err(_) => format!("Server returned error status: {}", status),
        };
        error!("{}", error_text);
        return Err(error_text);
      }
      resp
    }
    Err(e) => {
      error!("Failed to fetch mods: {}", e);
      return Err(format!("Failed to fetch mods: {}", e));
    }
  };
  // Parse the JSON response
  let json: serde_json::Value = match response.json().await {
    Ok(data) => data,
    Err(e) => {
      error!("Failed to parse JSON response: {}", e);
      return Err(format!("Failed to parse JSON response: {}", e));
    }
  };
  debug!("Successfully fetched and parsed mods from GameBanana: {}", json);

  // Convert JSON response to GBSubfeed
  match serde_json::from_value::<GBSubfeed>(json.clone()) {
    Ok(subfeed) => Ok(subfeed),
    Err(e) => {
      error!(
        "Failed to convert JSON to GBSubfeed: {} | JSON response: {}",
        e,
        json
      );
      Err(e.to_string())
    }
  }
}

// Command to fetch featured (TopSubs) mods from GameBanana
pub async fn fetch_featured_mods() -> Result<GBTopSubs, String> {
  info!("Fetching featured mods from GameBanana");
  let client = reqwest::Client::new();

  // URL for the featured mods endpoint
  let url = "https://gamebanana.com/apiv11/Game/8694/TopSubs";

  debug!("Requesting URL: {}", url);

  // Make the API request
  let response = match client.get(url).send().await {
    Ok(resp) => {
      debug!("Received response with status: {}", resp.status());
      if !resp.status().is_success() {
        let status = resp.status();
        let error_text = match resp.text().await {
          Ok(text) =>
            format!(
              "Server returned error status: {} with body: {}",
              status,
              text
            ),
          Err(_) => format!("Server returned error status: {}", status),
        };
        error!("{}", error_text);
        return Err(error_text);
      }
      resp
    }
    Err(e) => {
      error!("Failed to fetch featured mods: {}", e);
      return Err(format!("Failed to fetch featured mods: {}", e));
    }
  };

  // Parse the JSON response
  let json: serde_json::Value = match response.json().await {
    Ok(data) => data,
    Err(e) => {
      error!("Failed to parse JSON response: {}", e);
      return Err(format!("Failed to parse JSON response: {}", e));
    }
  };
  debug!("Successfully fetched and parsed featured mods from GameBanana: {}", json);

  // Convert JSON response to GBTopSubs
  match serde_json::from_value::<GBTopSubs>(json.clone()) {
    Ok(top_subs) => Ok(top_subs),
    Err(e) => {
      error!(
        "Failed to convert JSON to GBTopSubs: {} | JSON response: {}",
        e,
        json
      );
      Err(e.to_string())
    }
  }
}
// Function to get mod information from GameBanana
pub async fn get_mod_info(
  mod_id: i64,
  model_type: &str
) -> Result<GBProfilePage, String> {
  let mod_info_url = format!(
    "https://gamebanana.com/apiv11/{}/{}/ProfilePage",
    model_type,
    mod_id
  );
  debug!("Fetching mod info from: {}", mod_info_url);

  let client = reqwest::Client::new();
  match client.get(&mod_info_url).send().await {
    Ok(resp) => {
      if !resp.status().is_success() {
        warn!("Failed to fetch mod info, status: {}", resp.status());
        return Err(
          format!("Failed to fetch mod info, status: {}", resp.status())
        );
      }
      // Convert JSON response to GBProfilePage
      let data = match resp.json::<serde_json::Value>().await {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
          warn!("Failed to parse mod info: {}", e);
          return Err(format!("Failed to parse mod info: {}", e));
        }
      };

      match serde_json::from_value::<GBProfilePage>(data.clone()) {
        Ok(profile_page) => Ok(profile_page),
        Err(e) => {
          let error_msg = format!(
            "Failed to convert JSON to GBProfilePage: {} | JSON response: {}",
            e,
            data
          );
          error!("{}", error_msg);
          Err(error_msg)
        }
      }
    }
    Err(e) => {
      warn!("Failed to fetch mod info: {}", e);
      Err(format!("Failed to fetch mod info: {}", e))
    }
  }
}

// Function to get a Gamebanana mod's version
pub async fn get_mod_version(
  mod_id: i64,
  model_type: &str
) -> Result<String, String> {
  let mod_info = match get_mod_info(mod_id, model_type).await {
    Ok(info) => info,
    Err(e) => {
      return Err(e);
    }
  };

  // Extract the version from the mod info
  match mod_info.version {
    Some(version) => Ok(version),
    None => Err("Version not found".to_string()),
  }
}

// Function to get download information from GameBanana
pub async fn get_download_url(
  mod_id: i64,
  model_type: &str
) -> Result<String, String> {
  let download_page_url = format!(
    "https://gamebanana.com/apiv11/{}/{}/DownloadPage",
    model_type,
    mod_id
  );
  debug!("Download page API URL: {}", download_page_url);

  let client = reqwest::Client::new();
  let download_page_response = match
    client.get(&download_page_url).send().await
  {
    Ok(resp) => {
      debug!("Received download page response with status: {}", resp.status());
      if !resp.status().is_success() {
        let status = resp.status();
        let error_text = format!("Server returned error status: {}", status);
        error!("{}", error_text);
        return Err(error_text);
      }
      resp
    }
    Err(e) => {
      let error_msg = format!("Failed to fetch download page: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Parse the download page JSON
  let data = match download_page_response.json::<serde_json::Value>().await {
    Ok(data) => data,
    Err(e) => {
      let error_msg = format!("Failed to parse download page response: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Convert JSON response to GBDownloadPage
  let download_page = match serde_json::from_value::<GBDownloadPage>(data) {
    Ok(download_page) => {
      debug!("Successfully fetched download page for mod ID: {}", mod_id);
      Ok(download_page)
    }
    Err(e) => {
      let error_msg =
        format!("Failed to convert JSON to GBDownloadPage: {}", e);
      error!("{}", error_msg);
      Err(error_msg)
    }
  };
  // Extract the download URL from the download page
  match download_page {
    Ok(page) => {
      if let Some(download) = page.files.first() {
        debug!("Download URL found: {}", download.file);
        Ok(download.file.clone())
      } else {
        let error_msg = "No downloads available for this mod".to_string();
        error!("{}", error_msg);
        Err(error_msg)
      }
    }
    Err(e) => Err(e),
  }
}

// Function to get all download files information for a mod
pub async fn get_mod_download_files(
  mod_id: i64,
  model_type: &str
) -> Result<GBDownloadPage, String> {
  let download_page_url = format!(
    "https://gamebanana.com/apiv11/{}/{}/DownloadPage",
    model_type,
    mod_id
  );
  debug!("Download page API URL: {}", download_page_url);

  let client = reqwest::Client::new();
  let download_page_response = match
    client.get(&download_page_url).send().await
  {
    Ok(resp) => {
      debug!("Received download page response with status: {}", resp.status());
      if !resp.status().is_success() {
        let status = resp.status();
        let error_text = format!("Server returned error status: {}", status);
        error!("{}", error_text);
        return Err(error_text);
      }
      resp
    }
    Err(e) => {
      let error_msg = format!("Failed to fetch download page: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Parse the download page JSON
  let data = match download_page_response.json::<serde_json::Value>().await {
    Ok(data) => data,
    Err(e) => {
      let error_msg = format!("Failed to parse download page response: {}", e);
      error!("{}", error_msg);
      return Err(error_msg);
    }
  };

  // Convert JSON response to GBDownloadPage
  let download_page = match serde_json::from_value::<GBDownloadPage>(data) {
    Ok(download_page) => {
      debug!("Successfully fetched download page for mod ID: {}", mod_id);
      Ok(download_page)
    }
    Err(e) => {
      let error_msg =
        format!("Failed to convert JSON to GBDownloadPage: {}", e);
      error!("{}", error_msg);
      Err(error_msg)
    }
  };

  download_page
}
// Function to get a mod's posts
pub async fn get_mod_posts(
  mod_id: i64,
  page: i64,
  model_type: &str
) -> Result<GBModPosts, String> {
  let mod_posts_url = format!(
    "https://gamebanana.com/apiv11/{}/{}/Posts?_nPage={}&_nPerpage=15&_sSort=popular",
    model_type,
    mod_id,
    page
  );
  debug!("Fetching mod posts from: {}", mod_posts_url);

  let client = reqwest::Client::new();
  match client.get(&mod_posts_url).send().await {
    Ok(resp) => {
      if !resp.status().is_success() {
        warn!("Failed to fetch mod posts, status: {}", resp.status());
        return Err(
          format!("Failed to fetch mod posts, status: {}", resp.status())
        );
      }
      // Parse the JSON response
      let data = match resp.json::<serde_json::Value>().await {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
          warn!("Failed to parse mod posts: {}", e);
          return Err(format!("Failed to parse mod posts: {}", e));
        }
      };

      // Convert JSON response to GBModPosts
      let mod_posts = match serde_json::from_value::<GBModPosts>(data.clone()) {
        Ok(posts) => {
          debug!("Successfully fetched mod posts for mod ID: {}", mod_id);
          Ok(posts)
        }
        Err(e) => {
          let error_msg = format!(
            "Failed to convert JSON to GBModPosts: {} | JSON Response: {}",
            e,
            data
          );
          error!("{}", error_msg);
          Err(error_msg)
        }
      };
      mod_posts
    }
    Err(e) => {
      warn!("Failed to fetch mod posts: {}", e);
      Err(format!("Failed to fetch mod posts: {}", e))
    }
  }
}
// Function to get a mod's updates
pub async fn get_mod_updates(
  mod_id: i64,
  page: i64,
  model_type: &str
) -> Result<GBModUpdates, String> {
  let mod_updates_url = format!(
    "https://gamebanana.com/apiv11/{}/{}/Updates?_nPage={}&_nPerpage=5",
    model_type,
    mod_id,
    page
  );
  debug!("Fetching mod updates from: {}", mod_updates_url);

  let client = reqwest::Client::new();
  match client.get(&mod_updates_url).send().await {
    Ok(resp) => {
      if !resp.status().is_success() {
        warn!("Failed to fetch mod updates, status: {}", resp.status());
        return Err(
          format!("Failed to fetch mod updates, status: {}", resp.status())
        );
      }

      // Parse the JSON response
      let data = match resp.json::<serde_json::Value>().await {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
          warn!("Failed to parse mod updates: {}", e);
          return Err(format!("Failed to parse mod updates: {}", e));
        }
      };
      // Convert JSON response to GBModUpdates
      let mod_updates = match serde_json::from_value::<GBModUpdates>(data) {
        Ok(updates) => {
          debug!("Successfully fetched mod updates for mod ID: {}", mod_id);
          Ok(updates)
        }
        Err(e) => {
          let error_msg =
            format!("Failed to convert JSON to GBModUpdates: {}", e);
          error!("{}", error_msg);
          Err(error_msg)
        }
      };
      mod_updates
    }
    Err(e) => {
      warn!("Failed to fetch mod updates: {}", e);
      Err(format!("Failed to fetch mod updates: {}", e))
    }
  }
}
// Function to extract the first banner image URL from mod info
pub fn extract_banner_url(mod_info: GBProfilePage) -> Option<String> {
  mod_info.preview_media.images?.first().and_then(|image| {
    let base_url = image.base_url.clone();
    let file = image.file.clone();

    if !base_url.is_empty() && !file.is_empty() {
      Some(format!("{}/{}", base_url, file))
    } else {
      None
    }
  })
}

// Function to extract contributors from mod info
pub fn extract_contributors(
  mod_info: &GBProfilePage
) -> Option<Vec<crate::models::ContributorGroup>> {
  let mut contributor_groups = Vec::new();

  // Process each credit group
  for credit_group in &mod_info.credits {
    // Get the group name
    let group_name = &credit_group.group_name;

    let mut members = Vec::new();

    // Process authors in this group
    for author in &credit_group.authors {
      // Get author name
      let name = &author.name;

      // Get author role if available
      let role = author.role.clone();

      // Add the contributor to the group's members
      members.push(crate::models::Contributor {
        name: name.to_string(),
        icon: None, // GameBanana API doesn't provide author icons in this context
        role: role,
      });
    }

    // Only add groups with members
    if !members.is_empty() {
      contributor_groups.push(crate::models::ContributorGroup {
        group: group_name.to_string(),
        members,
      });
    }
  }

  // Return only if we found any contributors
  if !contributor_groups.is_empty() {
    debug!(
      "Extracted {} contributor groups from mod info",
      contributor_groups.len()
    );
    return Some(contributor_groups);
  } else {
    debug!("No contributors found in mod info");
    return None;
  }
}
