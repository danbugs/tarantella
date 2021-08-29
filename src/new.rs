use crate::constants::{INDEX_HTML, MAIN_C, MAKEFILE_MM, MAKEFILE_SM, TARANTELLA_MM_TOML, TARANTELLA_SM_TOML};
use crate::utils;
use failure::{Context, ResultExt};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

pub fn new(app_name: String, side_module: bool) -> Result<(), Context<String>> {
    utils::check_for_command("git", "`tapm` depends on git. To install it, see: https://git-scm.com/downloads")?;
    if Path::new(&app_name).exists() {
        return Err(Context::from(format!("folder {} already exists", app_name)));
    } else {
        make_default_folder(&app_name)?;
        make_default_folder(&format!("{}/src", app_name))?;
        make_default_folder(&format!("{}/dependencies", app_name))?;
        if side_module {
            utils::run_command("git init", "tapm failed to initialize a git repository")?;
            make_default_folder(&format!("{}/{}_latest", app_name, app_name))?;
            make_default_folder(&format!("{}/releases", app_name))?;
        } else {
            make_default_folder(&format!("{}/build", app_name))?;
            make_default_file("index.html", INDEX_HTML, &app_name)?;
        }

        make_default_file(
            "Tarantella.toml",
            if side_module {
                TARANTELLA_SM_TOML
            } else {
                TARANTELLA_MM_TOML
            },
            &app_name,
        )?;
        make_default_file("src/main.c", MAIN_C, &app_name)?;
        make_default_file(
            "Makefile",
            if side_module {
                MAKEFILE_SM
            } else {
                MAKEFILE_MM
            },
            &app_name,
        )?;
        Ok(())
    }
}

fn make_default_folder(folder_path: &String) -> Result<(), Context<String>> {
    fs::create_dir(folder_path).context(format!("failed while creating {} folder", folder_path))?;
    Ok(())
}

fn make_default_file(
    file_name: &str,
    content: &str,
    app_name: &String,
) -> Result<(), Context<String>> {
    let file = File::create(format!("{}/{}", app_name, file_name))
        .context(format!("failed while creating {} file", file_name));
    let content = content.to_string().replace("<app_name>", &app_name);
    file.unwrap()
        .write_all(content.as_bytes())
        .context(format!(
            "failed while writing contents to {} file",
            file_name
        ))?;

    Ok(())
}