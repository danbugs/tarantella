use failure::{Context, ResultExt};
use regex::{Match, Regex};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::{Child, Command, Output},
    str,
};

#[derive(Serialize, Deserialize)]
pub struct TarantellaToml {
    pub package: Package,
    pub dependencies: Option<Dependencies>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub module_type: Option<String>,
    pub build_dir: Option<String>,
    pub releases_repo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Dependencies {}

pub fn toml_to_struct(toml_file_name: &str) -> Result<TarantellaToml, Context<String>> {
    let contents_as_str = fs::read_to_string(toml_file_name)
        .context("Failed to read Tarantella.toml file".to_string());
    let contents_as_toml: TarantellaToml = toml::from_str(&contents_as_str.unwrap()).unwrap();
    Ok(contents_as_toml)
}

pub fn check_for_string<'a> (
    checked_str: &'a str,
    checking_str: &str,
    err_msg: &str,
) -> Result<Match<'a>, Context<String>> {
    let regex_match = Regex::new(checking_str).unwrap().find(checked_str);
    if regex_match.is_none() {
        return Err(Context::from(err_msg.to_string()));
    }
    Ok(regex_match.unwrap())
}

pub fn insert_string_in_file(
    file_name: &str,
    marker_string: &str,
    insert_str: &str,
    err_msg: &str,
) -> Result<(), Context<String>> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(file_name)
        .context(format!("tapm failed to open {}", file_name))?;
    let file_contents =
        fs::read_to_string(file_name).context(format!("tapm failed to read {}", file_name))?;

    let marker_index = check_for_string(&file_contents, marker_string, err_msg).unwrap().end();
    let mut altered_file = file_contents[..marker_index].to_string();
    let postmarker_file = &file_contents[marker_index..];

    altered_file.push_str(insert_str);
    altered_file.push_str(postmarker_file);
    file.write(altered_file.as_bytes())
        .context(format!("tapm failed to write to {}", file_name))?;
    Ok(())
}

pub fn remove_string_in_file(
    file_name: &str,
    remove_str: &str,
    err_msg: &str,
) -> Result<(), Context<String>> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(file_name)
        .context(format!("tapm failed to open {}", file_name))?;
    let file_contents =
        fs::read_to_string(file_name).context(format!("tapm failed to read {}", file_name))?;

    let marker_index = check_for_string(&file_contents, remove_str, err_msg)?;

    let mut altered_file = file_contents[..marker_index.start()].to_string();
    let postmarker_file = &file_contents[marker_index.end()..];

    altered_file.push_str(postmarker_file);
    file.set_len(0).context("tapm failed to erase old file's contents".to_string())?;
    file.write_all(altered_file.as_bytes())
        .context(format!("tapm failed to write to {}", file_name))?;
    Ok(())
}

pub fn check_for_command(command: &str, err_msg: &str) -> Result<Output, Context<String>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command, "--version"])
            .output()
            .unwrap()
    } else {
        Command::new("sh")
        .args(&["-c", &format!(r#"{} --version"#, command)])
        .output()
        .unwrap()
    };

    if str::from_utf8(&output.stdout).unwrap().is_empty() {
        return Err(Context::from(err_msg.to_string()));
    }

    Ok(output)
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
    let package = toml_to_struct("Tarantella.toml").unwrap().package;

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
        return Err(Context::from(format!(
            "Tarantella.toml is missing a {} field.",
            field_name
        )));
    } else {
        field = field_opt.unwrap();
    }

    Ok(field)
}

pub fn find_str_between(
    full_str: &str,
    a: &str,
    b: &str,
    offset: usize,
) -> Result<String, Context<String>> {
    let start_bytes = full_str.find(a).unwrap_or(0);
    let end_bytes = full_str.find(b).unwrap_or(b.len());

    Ok(full_str[(start_bytes + offset)..end_bytes].to_string())
}

pub fn check_ghlogin() -> Result<(), Context<String>> {
    check_for_command("gh", "tapm depends on the GitHub CLI. To install it, see: https://github.com/cli/cli#installation")?;
    let auth_status = run_command("gh auth status", "tapm failed to verify auth status")?;
    if !str::from_utf8(&auth_status.stderr).unwrap().contains("✓") {
        // ^^^ hacky way to check if user is logged in, could be improved
        return Err(Context::from(
            "You must be logged in to use this command — run: tapm login".to_string(),
        ));
    }

    return Ok(());
}

pub fn get_latest_version(repo_code: &str) -> Result<String, Context<String>> {
    let release_view = run_command(
        &format!("gh release view --repo {}", repo_code),
        "tapm publish failed to get information about the previous release",
    )?;
    let output = str::from_utf8(&release_view.stdout).unwrap();
    Ok(find_str_between(output, "tag:", "draft:", "tag:".len())
        .unwrap()
        .trim()
        .to_string())
}
