use std::{collections::HashMap, path::PathBuf};

use crate::{pkg::Pkg, Config, CliError, CliResult, PkgConfig};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pkgs {
    #[serde(skip_serializing_if = "HashMap::is_empty", default = "HashMap::new")]
    pub pkgs: std::collections::HashMap<String, Pkg>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default = "HashMap::new")]
    pub pkg_deps: std::collections::HashMap<String, Pkg>,
    #[serde(skip)]
    pub pkgs_dir: PathBuf,
    #[serde(skip)]
    pub deps_dir: PathBuf,
    #[serde(skip)]
    pub file: PathBuf,
    
}
impl Default for Pkgs {
    fn default() -> Self {
        let idir = dirs_next::home_dir().unwrap().join(".idla");
        return Self { 
            pkgs_dir: Self::pkgs_dir(&idir).unwrap(),
            deps_dir: Self::deps_dir(&idir).unwrap(),
            file: idir.join("Pkgs.toml"),
            pkgs: std::collections::HashMap::new(),
            pkg_deps: std::collections::HashMap::new(),
        };
    }
}
impl Pkgs {

    pub fn new(idir: PathBuf) -> CliResult<Self> {
        let file = idir.join("Pkgs.toml");
        let pdir = Self::pkgs_dir(&idir)?;
        let ddir = Self::deps_dir(&idir)?;
        let out = Self { pkgs_dir: pdir.clone(), deps_dir: ddir.clone(), file: file.clone(), ..Default::default() };
        println!("Got idla file: {} pdir: {}, ddir: {}", 
            file.to_str().unwrap(), pdir.to_str().unwrap(), ddir.to_str().unwrap());
        if !file.is_file() {
            println!("\x1b[33;1mCreating new pkgs repo list!\x1b[0m");
            std::fs::write(&file, &toml::to_string_pretty(&out)?)?;
            return Ok(out)
        } else {
            println!("\x1b[34;1mReading pkgs repo list!\x1b[0m");
            let ts = std::fs::read_to_string(&file)?;
            let res: Self = toml::from_str(&ts)?;
            Ok(Self { pkgs: res.pkgs, pkg_deps: res.pkg_deps, ..out })
        }
    }

    pub fn pkgs_dir(idla_dir: &PathBuf) -> CliResult<PathBuf> {
        let dir = idla_dir.join("pkgs");
        if !dir.is_dir() { std::fs::create_dir_all(&dir)? }
        return Ok(dir);
    }

    pub fn deps_dir(idla_dir: &PathBuf) -> CliResult<PathBuf> {
        let dir = idla_dir.join("deps");
        if !dir.is_dir() { std::fs::create_dir_all(&dir)? }
        return Ok(dir);
    }

    pub fn write(&self) -> CliResult<()> {
        std::fs::write(&self.file, toml::to_string_pretty(&self)?)?;
        Ok(())
    }

    pub fn add_pkg(&mut self, pkg: Pkg) -> CliResult<()> {
        self.pkgs.insert(pkg.name.clone(), pkg.clone());
        self.write()
    }
}
