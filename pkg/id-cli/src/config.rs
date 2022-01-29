use std::{path::PathBuf, os::macos::fs};
use dirs_next::{home_dir, data_dir, config_dir, document_dir};
use serde::{Serialize, Deserialize};

use crate::{CliResult, PkgConfig, pkg::Pkg, repo::Pkgs};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub idla_dir: PathBuf,
    #[serde(skip)]
    pub config_dir: PathBuf,
    #[serde(skip)]
    pub pkgs: Pkgs,
    #[serde(skip)]
    pub pkg_config_name: Option<String>,
    #[serde(default = "Config::version")]
    pub version: String,
    #[serde(default = "UserConfig::default")]
    pub user: UserConfig,
    #[serde(default = "SystemConfig::default")]
    pub system: SystemConfig,
}
#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct SystemConfig {
    debug: bool,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pkg_config_name: Some("Idla.pkg.toml".into()),
            pkgs: Pkgs::new(Self::idla_dir().unwrap()).unwrap(),
            version: Config::version(),
            idla_dir: Self::idla_dir().unwrap(),
            config_dir: Self::conf_dir(true).unwrap(),
            user: UserConfig::default(),
            system: SystemConfig::default()
        }
    }
}

impl Config {

    pub fn get() -> CliResult<Self> { 
        let cpath = Self::conf_dir(false)?;
        let mut conf = if !cpath.is_file() { 
            Self::create_config(&cpath)? 
        } else { 
            Self::read_config(&cpath)? 
        };
        // println!("CONFIG: {:#?}", &conf);
        conf.get_env();
        Ok(conf)
    }

    pub fn pkg_cfg_file(&self) -> String {
        if let Some(pn) = self.clone().pkg_config_name { pn }
        else { "Idla.pkg.toml".into() }
    }

    pub fn pkgs_cfg(&self) -> CliResult<Pkgs> {
        return Pkgs::new(self.clone().idla_dir)
    }

    fn version() -> String {
        "0.0.1".into()
    }

    pub fn get_env(&mut self) -> () {
        let env_vars: Vec<(&str, Box<dyn FnMut(String) -> ()>)> = vec![
            ("IDLA_HOME_DIR", Box::new(|val| self.idla_dir = PathBuf::from(&val))),
            ("IDLA_USER_NAME", Box::new(|name| self.user.name = Some(name.to_string()))),
            ("IDLA_USER_EMAIL", Box::new(|em| self.user.email = Some(em.to_string()))),

        ];
        for (ev, mut func) in env_vars {
            if let Ok(val) = std::env::var(ev) { func(val) }
        }
    }

    pub fn read_config(path: &PathBuf) -> CliResult<Self> {
        let r = toml::from_str(&std::fs::read_to_string(&path)?);
        return Ok(Self {
            pkg_config_name: Some("Idla.pkg.toml".into()),
            pkgs: Pkgs::new(Self::idla_dir()?)?,
            idla_dir: Self::idla_dir()?,
            config_dir: Self::conf_dir(false)?,
            ..r?
        })
    }
    pub fn create_config(path: &PathBuf) -> CliResult<Self> {
        let s = Self { 
            idla_dir: Self::idla_dir()?, 
            config_dir: Self::conf_dir(true)?,
            pkg_config_name: Some("Idla.pkg.toml".into()),
            pkgs: Pkgs::new(Self::idla_dir().unwrap()).unwrap(),
            ..Default::default()
        };
        std::fs::write(&path, toml::to_string_pretty(&s)?)?;
        // println!("GOT CONFIG: {:#?}", s);
        return Ok(s);
    }

    pub fn path(path: String, dir: bool, fallback: Option<String>) -> CliResult<PathBuf> {
        let cd = Self::idla_dir()?.join(path);
        if dir && !cd.is_dir() { std::fs::create_dir_all(&cd)?; }
        if !dir && !cd.is_file() { std::fs::write(&cd, fallback.unwrap_or("".into()))?; }
        return Ok(cd);
    }
    pub fn conf_dir(dir: bool) -> CliResult<PathBuf> {
        let cd = Self::idla_dir()?;
        if dir {
            if !cd.join("config").is_dir() { std::fs::create_dir_all(&cd.join("config"))?; }
            return Ok(cd.join("config"))
        } else { 
            let f = cd.join("Idla.toml");
            return Ok(f)
        }
    }

    pub fn idla_dir() -> CliResult<PathBuf> {
        let id = dirs_next::home_dir()
            .unwrap_or(PathBuf::from("~/"))
            .join(".idla");
        if !id.is_dir() { std::fs::create_dir_all(&id)?; }
        if !id.join("config").is_dir() { std::fs::create_dir_all(&id.join("config"))?; }
        if !id.join("pkgs").is_dir() { std::fs::create_dir_all(&id.join("pkgs"))?; }
        if !id.join("data").is_dir() { std::fs::create_dir_all(&id.join("data"))?; }
        return Ok(id);
    }

    pub fn create_pkg(&mut self, dir: PathBuf, name: String) -> CliResult<Pkg> {
        let pkg = Pkg::new(name.clone(), dir.clone());
        pkg.create(self)?;
        self.pkgs.add_pkg(pkg.clone())?;
        return Ok(pkg);
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        return toml::to_string_pretty(&self).unwrap_or("".into())
    }
}
