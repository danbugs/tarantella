use failure::{Context, ResultExt};
use serde_derive::{Deserialize};
use std::{fs, path::Path, process::{Child, Command, Output}};

#[derive(Deserialize)]
pub struct TarantellaToml {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub module_type: String,
    pub build_dir: String,
    pub releases_repo: String,
}

pub fn toml_to_struct(toml_file_name: &str) -> Result<TarantellaToml, Context<String>>{
    let contents_as_str = fs::read_to_string(toml_file_name).context("Failed to read Tarantella.toml file".to_string());
    let contents_as_toml : TarantellaToml = toml::from_str(&contents_as_str.unwrap()).unwrap();
    Ok(contents_as_toml)
}

pub fn check_for_command(command: &str, err_msg: &str) -> Result<(), Context<String>> {
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(&["/C", command, "--version"]).output().context(err_msg.to_string())?;
    } else {
        Command::new("sh").args(&["-c", command, "--version"]).output().context(err_msg.to_string())?;
    }
    Ok(())
}

pub fn check_for_path(path: &str, err_msg: &str) -> Result<(), Context<String>> {
    if !Path::new(path).exists() {
        return Err(Context::from(err_msg.to_string()));
    }

    Ok(())
}

pub fn run_command(command: &str, err_msg: &str) -> Result<Output, Context<String>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["/C", command])
            .output()
            .context(err_msg.to_string())?
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .output()
            .context(err_msg.to_string())?
    };
    Ok(output)
}

pub fn spawn_command(command: &str, err_msg: &str) -> Result<Child, Context<String>> {
    let child : Child = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["/C", command])
            .spawn()
            .context(err_msg.to_string())?
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .spawn()
            .context(err_msg.to_string())?
    };

    Ok(child)
}