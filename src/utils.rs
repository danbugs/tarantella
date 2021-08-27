use failure::{Context, ResultExt};
use serde_derive::{Deserialize};
use std::fs;

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
}

pub fn toml_to_struct(toml_file_name: &str) -> Result<TarantellaToml, Context<String>>{
    let contents_as_str = fs::read_to_string(toml_file_name).context("Failed to read Tarantella.toml file".to_string());
    let contents_as_toml : TarantellaToml = toml::from_str(&contents_as_str.unwrap()).unwrap();
    Ok(contents_as_toml)
}