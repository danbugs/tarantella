use failure::Context;
use std::str;

use crate::utils;

pub async fn publish() -> Result<(), Context<String>> {
    let releases_repo = utils::toml_to_struct("Tarantella.toml").unwrap().package.releases_repo;

    if releases_repo.is_empty() {
        first_release().await?
    } else {
        update_release(&releases_repo)?
    }

    Ok(())
}

fn create_public_repo(app_name: &str) -> Result<String, Context<String>> {
    // gh create repo <app_name>_releases
    let url = "".to_string();
    Ok(url)
}

fn create_release(url: &str) -> Result<(), Context<String>> {
    // add "git_releases" field to toml.
    Ok(())
}

async fn first_release() -> Result<(), Context<String>> {
    utils::check_for_command("gh", "`tapm` depends on the GitHub CLI. To install it, see: https://github.com/cli/cli#installation")?;
    let auth_status = utils::run_command(
        "gh auth status",
        "tapm publish failed on verifying auth status",
    )?;
    if str::from_utf8(&auth_status.stderr).unwrap().contains("✓") {
        // ^^^ hacky way to check if user is logged in, could be improved
        utils::check_for_command("git", "`tapm` depends on the `git` command — make sure you have got git installed. For instructions, visit: https://git-scm.com/downloads")?;
        let origin = utils::run_command("git remote show origin", "`tapm` depends on the `git` command — make sure you have got git installed. For instructions, visit: https://git-scm.com/downloads")?;

        if str::from_utf8(&origin.stderr).unwrap().is_empty() {
            // ^^^ origin exists
            if str::from_utf8(&origin.stdout)
                .unwrap()
                .contains("github.com")
            {
                // ^^^ git repo is hosted on GitHub

                let start_bytes = str::from_utf8(&origin.stdout)
                    .unwrap()
                    .find("https://github.com/")
                    .unwrap_or(0);
                let end_bytes = str::from_utf8(&origin.stdout)
                    .unwrap()
                    .find(".git")
                    .unwrap_or(str::from_utf8(&origin.stdout).unwrap().len());
                let url = &str::from_utf8(&origin.stdout).unwrap()[start_bytes..end_bytes];
                // ^^^ hacky way to get current repo's url

                let privacy_status = reqwest::get(url).await.unwrap().status();

                if privacy_status.is_success() {
                    // ^^^ repo is public
                    create_release(url)?;
                    return Ok(());
                }
            }
        }
    }
    let app_name = utils::toml_to_struct("Tarantella.toml").unwrap().package.name;
    let url = create_public_repo(&app_name)?;
    create_release(&url)?;
    return Ok(());
}

fn update_release(url: &str) -> Result<(), Context<String>> {
    // get latest release version from repo
    // get version in toml file

    // compare both versions -> iff same, throw an error
    // iff different -> call create_release(url)
    Ok(())
}
