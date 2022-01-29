use std::path::PathBuf;
use dirs_next::{home_dir, data_dir, config_dir, document_dir};
use serde::{Serialize, Deserialize};

use crate::CliResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub idla_dir: PathBuf,
    #[serde(skip)]
    pub config_dir: PathBuf,
    #[serde(default = "Config::version")]
    pub version: String,
    #[serde(default = "UserConfig::default")]
    pub user: UserConfig,
    #[serde(default = "SystemConfig::default")]
    pub system: SystemConfig,
}
#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct SystemConfig {

}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: Config::version(),
            idla_dir: Self::default_idla_dir().unwrap(),
            config_dir: Self::default_config_dir().unwrap(),
            user: UserConfig::default(),
            system: SystemConfig::default()
        }
    }
}

impl Config {

    pub fn get() -> CliResult<Self> { 
        let mut res = Self::get_create_file()?;
        res.get_env();
        return Ok(res)
    }

    fn version() -> String {
        "0.0.1".into()
    }

    pub fn get_env(&mut self) -> () {
        let env_vars: Vec<(&str, Box<dyn FnMut(String) -> ()>)> = vec![
            ("IDLA_HOME_DIR", Box::new(|val| self.idla_dir = PathBuf::from(&val))),
            ("IDLA_USER_NAME", Box::new(|name| self.user.name = Some(name.to_string()))),
            ("IDLA_USER_EMAIL", Box::new(|em| self.user.email = Some(em.to_string())))

        ];
        for (ev, mut func) in env_vars {
            if let Ok(val) = std::env::var(ev) { func(val) }
        }
    }

    pub fn get_create_file() -> CliResult<Self> {
        let path = Self::default_config_dir()?.join("Idla.toml");
        match std::fs::read(&path) {
            Ok(_content) => Ok(Self::default()),
            Err(_e) => Self::cp_default(&path),
        }
    }

    pub fn cp_default(path: &PathBuf) -> CliResult<Self> {
        std::fs::copy("res/Idla.toml", path)?;
        Ok(Self::default())
    }

    pub fn default_config_dir() -> CliResult<PathBuf> {
        let cd = config_dir()
            .unwrap_or(PathBuf::from("~/.config/"))
            .join("idla");
        if !cd.is_dir() { std::fs::create_dir_all(&cd)?; }
        return Ok(cd);
    }

    pub fn default_idla_dir() -> CliResult<PathBuf> {
            let id = home_dir()
                .or(config_dir())
                .or(data_dir())
                .or(document_dir())
                .unwrap_or(PathBuf::from("~/"))
                .join(".idla");
        if !id.is_dir() { std::fs::create_dir_all(&id)?; }
        return Ok(id);
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        return "idla_dir = \"\"\n".into()
    }
}
