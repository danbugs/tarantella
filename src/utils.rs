use failure::{Context, ResultExt};
use serde_derive::{Deserialize, Serialize};
use std::{fs::{self, File}, io::Write, path::Path, process::{Child, Command, Output}};

#[derive(Serialize, Deserialize)]
pub struct TarantellaToml {
    pub package: Package,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub module_type: Option<String>,
    pub build_dir: Option<String>,
    pub releases_repo: Option<String>,
}

pub fn toml_to_struct(toml_file_name: &str) -> Result<TarantellaToml, Context<String>> {
    let contents_as_str = fs::read_to_string(toml_file_name)
        .context("Failed to read Tarantella.toml file".to_string());
    let contents_as_toml: TarantellaToml = toml::from_str(&contents_as_str.unwrap()).unwrap();
    Ok(contents_as_toml)
}

pub fn update_toml(toml_file_name: &str, toml: &TarantellaToml) -> Result<(), Context<String>> {
    fs::write(toml_file_name, toml::to_string(&toml).unwrap())
        .context("Failed to update Tarantella.toml".to_string())?;
    Ok(())
}

pub fn check_for_command(command: &str, err_msg: &str) -> Result<(), Context<String>> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command, "--version"])
            .output()
            .context(err_msg.to_string())?;
    } else {
        Command::new("sh")
            .args(&["-c", command, "--version"])
            .output()
            .context(err_msg.to_string())?;
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
        Command::new("cmd")
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
    let child: Child = if cfg!(target_os = "windows") {
        Command::new("cmd")
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

pub fn make_default_folder(folder_path: &str) -> Result<(), Context<String>> {
    fs::create_dir(folder_path).context(format!("Failed while creating {} folder", folder_path))?;
    Ok(())
}

pub fn make_default_file(
    file_path: &str,
    content: &str,
    app_name: &str,
) -> Result<(), Context<String>> {
    let file = File::create(file_path).context(format!("Failed while creating {} file", file_path));
    let content = content.to_string().replace("<app_name>", &app_name);
    file.unwrap()
        .write_all(content.as_bytes())
        .context(format!(
            "Failed while writing contents to {} file",
            file_path
        ))?;

    Ok(())
}

pub fn check_for_toml_field(field_name: &str) -> Result<String, Context<String>> {
    let package = toml_to_struct("Tarantella.toml")
        .unwrap()
        .package;

        let field_opt = match field_name {
            "name" => package.name,
            "version" => package.version,
            "module_type" => package.module_type,
            "build_dir" => package.build_dir,
            "releases_repo" => package.releases_repo,
            _ => return Err(Context::from("Invalid field requested".to_string())),
         };

    let field;
    if field_opt.is_none() {
        return Err(Context::from(format!("Tarantella.toml is missing a {} field. Add `build_dir = <app_name>_latest` to your Tarantella.toml to continue", field_name)));
    } else {
        field = field_opt.unwrap();
    }

    Ok(field)
}
