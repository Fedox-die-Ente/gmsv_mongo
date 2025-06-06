use serde::Deserialize;
use crate::logger::{log, LogLevel};

#[derive(Deserialize)]
struct GitHubTag {
    name: String,
}

pub(crate) fn check_latest_version() -> Result<(), Box<dyn std::error::Error>> {
    let repo_owner = "Fedox-die-Ente";
    let repo_name = "gmsv_mongo";
    let url = format!("https://api.github.com/repos/{}/{}/tags", repo_owner, repo_name);

    let response = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0")
        .call();

    if let Ok(response) = response {
        let tags: Vec<GitHubTag> = response.into_json()?;

        if let Some(latest_tag) = tags.first() {
            let current_version = env!("CARGO_PKG_VERSION");

            log(LogLevel::Info, "Checking for updates...");

            if latest_tag.name != current_version {
                log(LogLevel::Warning, &format!("A new version is available: {}", latest_tag.name));
            } else {
                log(LogLevel::Info, "You are using the latest version.");
            }
        } else {
            log(LogLevel::Error, "Failed to check for updates.");
        }
    } else {
        log(LogLevel::Error, "Failed to send request.");
    }

    Ok(())
}
