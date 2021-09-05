use crate::constants::{GIT_IGNORE, INDEX_HTML, INDEX_JS, MAIN_C_MM, MAIN_C_SM, MAKEFILE_MM, MAKEFILE_SM, TARANTELLA_MM_TOML, TARANTELLA_MM_SERVER_TOML, TARANTELLA_SM_TOML};
use crate::utils;
use failure::Context;
use std::path::Path;

pub fn new(app_name: String, side_module: bool, no_git: bool, server: bool) -> Result<(), Context<String>> {
    if side_module && server {
        return Err(Context::from("A side module cannot be a server-side app.".to_string()));
    }
    utils::check_for_command(
        "git",
        "tapm depends on git. To install it, see: https://git-scm.com/downloads",
    )?;
    if Path::new(&app_name).exists() {
        return Err(Context::from(format!("folder {} already exists", app_name)));
    } else {
        utils::make_default_folder(&app_name)?;
        utils::make_default_folder(&format!("{}/src", app_name))?;
        utils::make_default_folder(&format!("{}/dependencies", app_name))?;
        utils::make_default_folder(&format!("{}/releases", app_name))?;
        if !no_git {
            utils::run_command(
                &format!("git init {}/", app_name),
                "tapm failed to initialize a git repository",
            )?;
        }
        utils::make_default_folder(&format!("{}/build", app_name))?;
        if !side_module {
            utils::make_default_file(&format!("{}/src/main.c", app_name), MAIN_C_MM, &app_name)?;
            if !server {
                utils::make_default_file(&format!("{}/index.html", app_name), INDEX_HTML, &app_name)?;
            } else {
                utils::make_default_file(&format!("{}/index.js", app_name), INDEX_JS, &app_name)?;
            }
        } else {
            utils::make_default_file(&format!("{}/src/main.c", app_name), MAIN_C_SM, &app_name)?;
        }

        utils::make_default_file(
            &format!("{}/Tarantella.toml", app_name),
            if side_module {
                TARANTELLA_SM_TOML
            } else if server {
                TARANTELLA_MM_SERVER_TOML
            } else {
                TARANTELLA_MM_TOML
            },
            &app_name,
        )?;
        utils::make_default_file(
            &format!("{}/Makefile", app_name),
            if side_module {
                MAKEFILE_SM
            } else {
                MAKEFILE_MM
            },
            &app_name,
        )?;
        utils::make_default_file(&format!("{}/.gitignore", app_name), GIT_IGNORE, &app_name)?;

        info!("{}", &format!("Created new WASM app at {}/", app_name));

        Ok(())
    }
}
