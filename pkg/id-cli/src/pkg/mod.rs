use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, )]
pub struct PkgConfig {
    name: String,
    version: String,
    deps: DepConfig,
}
impl PkgConfig {

    fn default_file_name() -> String { "Idla.pkg.toml".into() }

    fn create(dir: std::path::PathBuf, name: String) -> crate::CliResult<()> {
        let pkgc = Self { name, ..Default::default() };
        let pkgs = toml::to_string_pretty(&pkgc)?;
        std::fs::write(dir.join(Self::default_file_name()), pkgs)?;
        Ok(())
    }

}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DepConfig {
    dev: Vec<Dep>,
    prod: Vec<Dep>,
    
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Dep {
    name: String,
    version: String,
    features: Vec<String>,
}
impl Default for PkgConfig {
    fn default() -> Self {
        Self {
            name: "".into(),
            version: "0.0.1".into(),
            deps: DepConfig::default(),
        }
    }
}

