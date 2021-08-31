use failure::{Context, ResultExt};
use online::check;
use std::{path::PathBuf, str};
use zip_extensions::*;
use std::fs;

use crate::{constants::README, utils};

pub async fn publish() -> Result<(), Context<String>> {
    let releases_repo = utils::check_for_toml_field("releases_repo")?;
    let app_name = utils::check_for_toml_field("name")?;
    let build_dir = utils::check_for_toml_field("build_dir")?;
    if !check(None).await.is_ok() {
        return Err(Context::from(
            "You need an internet connectivity to run tapm publish".to_string(),
        ));
    }

    if PathBuf::from(&build_dir)
        .read_dir()
        .unwrap()
        .next()
        .is_none()
    {
        return Err(Context::from(format!(
            "There's nothing to build in {}/",
            build_dir
        )));
    }

    if releases_repo.is_empty() {
        info!("Initiating first release...");
        first_release(&app_name).await?
    } else {
        info!("Creating a new release...");
        update_release().await?
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
        format!(
            "Creating local git repo for releases at ../{}_releases/...",
            app_name
        )
    );

    warn!(
        "{}",
        format!(
            "If this process crashes or you cancel after this, you must also manually delete the local git repo for releases at ../{}_releases/",
            app_name
        )
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
    info!(
        "{}",
        format!("Created remote public git repo at {}", return_url)
    );
    warn!(
        "{}",
        format!(
            "If this process crashes or you cancel it now, you must also manually delete the git repo at {}",
            return_url
        )
    );

    Ok(return_url)
}

fn create_release(app_name: &str, url: &str, extra_command: &str) -> Result<(), Context<String>> {
    info!("Specify release details below:");
    utils::insert_string_in_file("Tarantella.toml", r#"releases_repo\s*=\s*""#, url, "tapm publish failed to add url to releases_repo field")?;

    warn!("If this process crashes or you cancel it now and this is your first release, you might want to manually set the release_repo field from Tarantella.toml to \"\".");

    fs::copy("Tarantella.toml", "build/Tarantella.toml").context("tapm build failed to copy Tarantella.toml to build directory".to_string())?;
    let version = utils::check_for_toml_field("version")?;
    utils::check_for_path("releases/", "tapm publish failed to find releases/ folder")?;
    let archive_file: PathBuf = PathBuf::from(format!("releases/{}-{}.zip", app_name, version));
    let source_dir: PathBuf =
        PathBuf::from(format!("{}", utils::check_for_toml_field("build_dir")?));
    zip_create_from_directory(&archive_file, &source_dir)
        .context("tapm publish failed at creating a zip file for the release".to_string())?;
    fs::remove_file("build/Tarantella.toml").context("tapm build failed to copy Tarantella.toml to build directory".to_string())?;
    
    info!(
        "{}",
        format!(
            "Created new release at releases/{}-{}.zip",
            app_name, version
        )
    );
    let path_fragment = if extra_command.is_empty() {
        "".to_string()
    } else {
        format!("../{}/", app_name).to_string()
    };

    let mut child = utils::spawn_command(
        &format!(
            "{}gh release create {} {}releases/{}-{}.zip",
            extra_command, version, path_fragment, app_name, version
        ),
        "tapm publish failed to add a README to the releases git repo",
    )?;

    child.wait().unwrap();

    Ok(())
}

async fn first_release(app_name: &str) -> Result<(), Context<String>> {
    utils::check_ghlogin()?;
    let mut url = get_repo_url().await.unwrap();

    if !url.is_empty() {
        create_release(app_name, &url, "")?;
    } else {
        url = create_public_repo(&app_name)?;
        create_release(app_name, &url, &format!("cd ../{}_releases && ", app_name))?;
    }
    return Ok(());
}

async fn update_release() -> Result<(), Context<String>> {
    let app_name = utils::check_for_toml_field("name").unwrap();

    let external_repo = get_repo_url().await.unwrap();
    if external_repo.is_empty() {
        utils::check_for_path(
            &format!("../{}_releases", app_name),
            &format!("../{}_releases folder is missing", app_name),
        )?;
    }
    // ^^^ bad naming here but I'm pretty much just checking to see if the project is using another repo for releases or not

    let releases_repo = utils::check_for_toml_field("releases_repo").unwrap(); // releases_repo here is empty if tapm is using the <app_name>_releases repo
    let repo_code = &releases_repo
        [(releases_repo.find("https://github.com/").unwrap_or(0) + "https://github.com/".len())..];
    // ^^^ might want to improve this in the future to use utils::find_str_between

    let published_version = utils::get_latest_version(repo_code)?;
    let current_version = utils::check_for_toml_field("version").unwrap(); // releases_repo here is empty if tapm is using the <app_name>_releases repo

    if current_version.eq(&published_version) {
        return Err(Context::from("Tarantella.toml's version is the same as your latest published version —— update it before publishing a new release.".to_string()));
    } else {
        create_release(
            &app_name,
            &releases_repo,
            &if external_repo.is_empty() {
                format!("cd ../{}_releases && ", app_name).to_string()
            } else {
                "".to_string()
            },
        )?;
    }

    Ok(())
}

// returns false if:
// - there was no remote origin found in current git repo.
// - the remote origin is not hosted on GitHub.
// - the remote origin repo is not public.

async fn get_repo_url() -> Result<String, Context<String>> {
    let git_err_msg = "tapm depends on the git command — make sure you have got git installed. For instructions, visit: https://git-scm.com/downloads";
    utils::check_for_command("git", git_err_msg)?;
    let origin = utils::run_command("git remote show origin", git_err_msg)?;

    if str::from_utf8(&origin.stderr).unwrap().is_empty() {
        // ^^^ origin exists
        let stdout = str::from_utf8(&origin.stdout).unwrap();
        if stdout.contains("github.com") {
            // ^^^ git repo is hosted on GitHub
            let url = utils::find_str_between(stdout, "https://github.com/", ".git", 0)?; // hacky way to get current repo's url

            let privacy_status = reqwest::get(&url).await.unwrap().status();

            if privacy_status.is_success() {
                // ^^^ repo is public
                return Ok(url);
            } else {
                info!("Source code is private...");
            }
        } else {
            info!("Current repo is not hosted on GitHub...");
        }
    } else {
        info!("Current repo does not contain a remote origin...");
    }

    return Ok("".to_string());
}
