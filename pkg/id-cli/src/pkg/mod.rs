use std::{fs, path::PathBuf, str::FromStr, collections::HashMap};

use serde::{Serialize, Deserialize};
use crate::{CliResult, CliError, Config};

#[derive(Serialize, Deserialize, Clone, Debug, )]
pub struct Pkg {
    pub name: String,
    pub dir: PathBuf,
}
impl Pkg {

    pub fn new(name: String, dir: PathBuf) -> Self {
        Pkg { 
            name: name.clone(), 
            dir, 
        }
    }

    pub fn write_src_entry(&self, ty: &PkgType) -> CliResult<()> {
        match ty {
            PkgType::Lib => {
                let (fname, cont): (PathBuf, String) = (
                    self.dir.join("src").join(&String::from("lib.idla")), 
                    "pub mod main\nexport fn main[]: [] = \nprint \"Hello lib\"".into());
                std::fs::write(&fname, cont)?;
            },
            PkgType::Bin => { 
                let (fname, cont): (PathBuf, String) = (
                    self.dir.join("src").join(&String::from("main.idla")), 
                    "mod main\nfn main[]: [] =\nprint \"Hello world!\"".into());
                std::fs::write(&fname, cont)?;
            }
            PkgType::Hybrid => {
                self.write_src_entry(&PkgType::Lib)?;
                self.write_src_entry(&PkgType::Bin)?;
            }
        }
        Ok(())
    }

    pub fn create(&self, config: &mut Config) -> CliResult<()> {
        if !self.dir.is_dir() { std::fs::create_dir_all(&self.dir)?; }

        let pkg_cfg = PkgConfig::new(self.clone().name);
        pkg_cfg.create(&self.dir, config)?;
        fs::create_dir(self.dir.join("src"))?;
        fs::create_dir_all(self.dir.join("dist").join("bin"))?;
        self.write_src_entry(&pkg_cfg.info.pkg_type)?;

        Ok(())
    }

    pub fn read_config(&self, cfg: &Config) -> CliResult<PkgConfig> {
        let file = self.dir.join(cfg.pkg_config_name.clone().unwrap_or("Idla.pkg.toml".into()));
        let res: PkgConfig = toml::from_str(&std::fs::read_to_string(&file)?)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, )]
pub struct PkgConfig {
    info: PkgInfo,
    deps: DepConfig,
    #[serde(skip)]
    pub local_idla_config: Option<Config>,
}
#[derive(Serialize, Deserialize, Clone, Debug, )]
pub struct PkgInfo {
    name: String,
    pkg_type: PkgType,
    idla_version: String,
    version: String,
}
#[derive(clap::ArgEnum, Serialize, Deserialize, Clone, Debug, )]
#[clap(name = "type")]
pub enum PkgType {
    Lib, 
    Bin, 
    Hybrid,
}
impl FromStr for PkgType {
    type Err = clap::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "l"| "lib" => Ok(PkgType::Lib),
            "b"| "bin" => Ok(PkgType::Bin),
            "h"| "hybrid" => Ok(PkgType::Hybrid),
            _ => Err(clap::Error::raw(clap::ErrorKind::InvalidValue, &format!("No type {}", s))),
        }    
    }
}
impl Default for PkgType { 
    fn default() -> Self { Self::Bin }
}

impl PkgInfo {
    pub fn new(name: String) -> Self {
        Self { name: name.to_owned(), ..Default::default() }

    }
}
impl PkgConfig {

    pub fn new(name: String) -> Self {
        Self { info: PkgInfo::new(name), ..Default::default() }
    }

    pub fn create(&self, dir: &PathBuf, cfg: &Config) -> CliResult<()> {
        let pkgs = toml::to_string_pretty(&self)?;
        std::fs::write(dir.join(cfg.pkg_cfg_file()), pkgs)?;
        Ok(())
    }

}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DepConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    dev: Vec<Dep>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    release: Vec<Dep>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    test: Vec<Dep>,
    
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Dep {
    name: String,
    version: String,
    features: Vec<String>,
}
impl Default for PkgInfo {
    fn default() -> Self {
        Self {
            name: "".into(),
            pkg_type: PkgType::Bin,
            version: "0.0.1".into(),
            idla_version: "2022".into(),
        }
    }
}
impl Default for PkgConfig {
    fn default() -> Self {
        Self {
            local_idla_config: None,
            info: PkgInfo::default(),
            deps: DepConfig::default(),
        }
    }
}

impl ToString for PkgConfig {
    fn to_string(&self) -> String {
        return toml::to_string_pretty(&self).unwrap_or("".into())
    }
}
