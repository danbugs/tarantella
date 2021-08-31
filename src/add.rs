use crate::utils;
use failure::Context;
use failure::ResultExt;
use fs_extra::dir;
use online::check;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;
use zip_extensions::*;

pub async fn add(
    owner_and_depname: String,
    mut raw_version: Option<String>,
) -> Result<(), Context<String>> {
    if !check(None).await.is_ok() {
        return Err(Context::from(
            "You need an internet connectivity to run tapm add".to_string(),
        ));
    }

    utils::check_ghlogin()?; // this already checks for the gh command
    let repo_view = utils::run_command(
        &format!("gh repo view {}", owner_and_depname),
        "tapm add failed to verify this dependency's availability",
    )?;
    if !str::from_utf8(&repo_view.stderr).unwrap().is_empty() {
        return Err(Context::from(
            "This dependency does not seem to exist on GitHub".to_string(),
        ));
    }
    if raw_version.is_none() {
        // get latest version to default to
        let release_list = utils::run_command(
            &format!("gh release list --repo {} --limit 1", owner_and_depname),
            "tapm add failed to check this dependency's latest version",
        )?;
        if str::from_utf8(&release_list.stdout).unwrap().is_empty() {
            return Err(Context::from(
                "This dependency has no releases yet".to_string(),
            ));
        }

        let latest_version = utils::get_latest_version(&owner_and_depname)?;
        raw_version = Some(latest_version);
    }

    let desired_version = raw_version.unwrap();

    info!(
        "Adding dependency {}-{}...",
        owner_and_depname, &desired_version
    );

    if utils::check_for_path("tmp/", "").is_ok() {
        return Err(Context::from(
            "tmp/ folder already exists — delete it before continuing".to_string(),
        ));
    }

    fs::create_dir_all("tmp/").context(
        "tapm add failed at creating tmp folder to store dependency for verification".to_string(),
    )?; // create tmp folder
    warn!("If this process crashes or you cancel it now, you will have to manually delete the tmp/ folder to avoid any adverse side-effects from the crash/cancel");

    let mut child = utils::spawn_command(
        &format!(
            "cd tmp/ && gh release download {} --pattern *.zip --repo {}",
            &desired_version, owner_and_depname
        ),
        "tapm add failed to download this dependency",
    )?;

    child.wait().unwrap(); // download release from gh to tmp folder

    let tmp_folder =
        fs::read_dir("tmp/").context("tapm failed to get items from tmp/".to_string())?;
    let mut zipped_file = PathBuf::from("");
    for path in tmp_folder {
        zipped_file = path.unwrap().path()
    }

    let target_dir: PathBuf = PathBuf::from("tmp/");
    zip_extract(&zipped_file, &target_dir)
        .context("tapm add failed to unzip dependency release".to_string())?; // unzip it in tmp
    fs::remove_file(zipped_file)
        .context("tapm add failed to delete zip post extracting contents".to_string())?; // delete the .zip file

    let dep_name = utils::toml_to_struct("tmp/Tarantella.toml")?
        .package
        .name
        .unwrap(); // get app_name from tmp folder's Tarantella.toml (let's call it dep_name)

    if utils::check_for_path(&format!("dependencies/{}", &dep_name), "").is_ok() {
        fs::remove_dir_all(format!("dependencies/{}", &dep_name))
            .context("tapm add failed to delete previous dependency version".to_string())?;
    }

    let options = dir::CopyOptions::new();
    dir::move_dir("tmp/", "dependencies/", &options).context(format!(
        "tapm add failed to move tmp/ contents to dependencies/{}",
        dep_name
    ))?;

    fs::rename("dependencies/tmp", &format!("dependencies/{}", dep_name)).context(format!(
        "tapm add failed to rename dependencies/tmp/ to dependencies/{}",
        dep_name
    ))?;

    warn!("Deleted tmp/ folder");
    info!(
        "Added dependency at: {}",
        &format!("dependencies/{}", dep_name)
    );
    warn!("{}", format!("If this process crashes or you cancel it now, you will have to manually delete the dependencies/{} folder", dep_name));

    let mut toml = OpenOptions::new()
        .write(true)
        .append(true)
        .open("Tarantella.toml")
        .context("tapm add failed to open Tarantella.toml to add dependency".to_string())?;

    let toml_contents = fs::read_to_string("Tarantella.toml")
        .context("tapm failed to read Tarantella.toml".to_string())?;
    if utils::check_for_string(&toml_contents, &dep_name, "").is_ok() {
        utils::remove_string_in_file("Tarantella.toml", &format!(r#"{}\s*=\s*".+"#, dep_name), "tapm add couldn't remove previous reference to this dependency from Tarantella.toml")?;
    }

    if let Err(_) = write!(
        toml,
        "\n{} = \"{} {}\"",
        dep_name, owner_and_depname, desired_version
    ) {
        return Err(Context::from(
            "tapm add failed to open Tarantella.toml to add dependency".to_string(),
        ));
    }
    // ^^^ add dependency to Tarantella.toml

    info!("Added dependency to Tarantella.toml file");
    warn!("If this process crashes or you cancel it now, you will have to manually delete the new dependency's field from Tarantella.toml");
    
    let makefile_contents =
        fs::read_to_string("Makefile").context("tapm failed to failed to open Makefile to check for dependency".to_string())?;
    
    if utils::check_for_string(&makefile_contents, &format!("dependencies/{}/{}.o ", dep_name, dep_name), "tapm add failed to verify if dependency was listed in Makefile").is_err() {
        utils::insert_string_in_file(
            "Makefile",
            r#"DEPENDENCIES\s*="#,
            &format!("dependencies/{}/{}.o ", dep_name, dep_name),
            "tapm add failed — Add DEPENDENCIES= variable to your Makefile.",
        )?;
    }
    // ^^^ add dependency to the Makefile list of dependencies

    // if this is a update, we leave the Makefile as is <-- subject to change
    info!("Added dependency to Makefile");

    Ok(())
}
