use failure::{Context, ResultExt};
use std::{path::PathBuf, str};
use zip_extensions::*;

use crate::{constants::README, utils};

pub async fn publish() -> Result<(), Context<String>> {
    let releases_repo = utils::check_for_toml_field("releases_repo")?;
    if releases_repo.is_empty() {
        info!("Initiating first release...");
        first_release(&utils::check_for_toml_field("name")?).await?
    } else {
        info!("Creating a new release...");
        update_release(&releases_repo)?
    }

    Ok(())
}

fn create_public_repo(app_name: &str) -> Result<String, Context<String>> {
    let output = utils::run_command(
        &format!("cd .. && gh repo create {}_releases --public -y", app_name),
        "tapm publish failed at creating a public repo for releases",
    )?;
    info!(
        "{}",
        format!("Creating local git repo for releases at ../{}_releases...", app_name)
    );

    utils::make_default_file(
        &format!("../{}_releases/README.md", app_name),
        README,
        &app_name.to_string(),
    )?;

    utils::run_command(
        &format!("cd ../{}_releases && git branch -M main", app_name),
        "tapm publish failed at renaming master branch to main branch in the releases git repo",
    )?;

    utils::run_command(
        &format!("cd ../{}_releases && git add .", app_name),
        "tapm publish failed to add files to the releases git repo's index",
    )?;

    utils::run_command(
        &format!(
            "cd ../{}_releases && git commit --allow-empty-message -m ''",
            app_name
        ),
        "tapm publish failed to commit changes to the releases git repo",
    )?;

    utils::run_command(
        &format!("cd ../{}_releases && git push -u origin main", app_name),
        "tapm publish failed to push changes to the remote origin of the releases git repo",
    )?;

    let return_url = str::from_utf8(&output.stdout).unwrap().trim().to_string();
    info!("{}", format!("Created remote public git repo at {}", return_url));

    Ok(return_url)
}

fn create_release(app_name: &str, url: &str, extra_command: &str) -> Result<(), Context<String>> {
    let mut toml = utils::toml_to_struct("Tarantella.toml")?;
    toml.package.releases_repo = Some(url.to_string());
    utils::update_toml("Tarantella.toml", &toml)?;

    let version = utils::check_for_toml_field("version")?;
    let archive_file: PathBuf = PathBuf::from(format!("releases/{}-{}.zip", app_name, version));
    let source_dir: PathBuf =
        PathBuf::from(format!("{}", utils::check_for_toml_field("build_dir")?));
    zip_create_from_directory(&archive_file, &source_dir)
        .context("tapm publish failed at creating a zip file for the release".to_string())?;

    let mut child = utils::spawn_command(
        &format!(
            "{}gh release create {} ../{}/releases/{}-{}.zip",
            extra_command, version, app_name, app_name, version
        ),
        "tapm publish failed to add a README to the releases git repo",
    )?;

    child.wait().unwrap();

    Ok(())
}

async fn first_release(app_name: &str) -> Result<(), Context<String>> {
    utils::check_for_command("gh", "tapm depends on the GitHub CLI. To install it, see: https://github.com/cli/cli#installation")?;
    let auth_status = utils::run_command(
        "gh auth status",
        "tapm publish failed on verifying auth status",
    )?;
    if str::from_utf8(&auth_status.stderr).unwrap().contains("✓") {
        // ^^^ hacky way to check if user is logged in, could be improved
        let git_err_msg = "tapm depends on the git command — make sure you have got git installed. For instructions, visit: https://git-scm.com/downloads";
        utils::check_for_command("git", git_err_msg)?;
        let origin = utils::run_command("git remote show origin", git_err_msg)?;

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
                    create_release(app_name, url, "")?;
                    return Ok(());
                } else {
                    info!("Source code is private...");
                }
            } else {
                info!("Current repo is not hosted on GitHub...");
            }
        } else {
            info!("Current repo does not contain a remote origin...");
        }

        // code escapes here if:
        // - there was no remote origin found in current git repo.
        // - the remote origin is not hosted on GitHub.
        // - the remote origin repo is not public.

        let url = create_public_repo(&app_name)?;
        create_release(app_name, &url, &format!("cd ../{}_releases && ", app_name))?;
        return Ok(());
    } else {
        return Err(Context::from(
            "You must be logged in to use tapm publish — run: tapm login".to_string(),
        ));
    }
}

fn update_release(_url: &str) -> Result<(), Context<String>> {
    // get latest release version from repo
    // get version in toml file

    // compare both versions -> iff same, throw an error
    // iff different -> call create_release(url)
    Ok(())
}
