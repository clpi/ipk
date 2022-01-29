use std::path::PathBuf;

use clap::{Subcommand, Parser, ArgSettings, AppSettings, ValueHint};
use crate::{error::PkgError, pkg::{PkgType, Pkg, PkgConfig}, Config, Cmd, CliResult, CliError};

#[derive(Parser, Debug)]
pub struct InitCmd {
    #[clap(required = false)]
    name: Option<String>,
    #[clap(short = 't', long = "type", required = false)]
    pkg_type: Option<PkgType>,
    #[clap(short, long, required = false)]
    dir: Option<PathBuf>,
}

impl InitCmd{
    fn cwd(name: Option<String>) -> PathBuf {
        let pb = std::env::current_dir().unwrap_or(PathBuf::from("."));
        if let Some(n) = name { pb.join(&n) } else { pb }
    }
    fn dir_name(&self) -> (PathBuf, String) {
        let dir = if let Some(d) = self.dir.clone() { d } else { Self::cwd(self.name.clone()) };
        let name = if let Some(n) = self.name.clone() { n }
        else { dir.components().last().unwrap().as_os_str().to_str().unwrap().into() };
        return (dir, name);

    }
    fn prompt(pstr: &str, col: &str) -> std::io::Result<String> {
        let mut nm = String::new();
        print!("\x1b[{}m{}\x1b[0m:\x1b[0m  ", col, pstr, );
        std::io::stdin().read_line(&mut nm)?;
        Ok(nm)
    }

    fn is_pkg(dir: PathBuf, config: &Config) -> bool {
        if dir.is_dir() && dir.join(config.pkg_cfg_file()).is_file() { true }
        else { false }
    }
    fn create_pkg(&self, cfg: &mut Config) -> CliResult<Pkg> {
        let (dir, name) = self.dir_name();
        println!("\x1b[35;1mCreating package:\x1b[0m\nDir: {:#?}\n Name {:?}...", dir, name);
        let pk = cfg.create_pkg(dir, name)?;
        return Ok(pk)
    }
}
impl Cmd for InitCmd {

    fn exec(&self, config: &mut Config) -> CliResult<()> {
        let (d, n) = self.dir_name();
        if Self::is_pkg(d.clone(), &config) {
            return Err(CliError::Pkg(PkgError::AlreadyExists(d.clone())))
        } 
        let pkg = self.create_pkg(config)?;
        Ok(())
    }
}

impl Default for InitCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        InitCmd { dir: Some(cwd), pkg_type: Some(PkgType::Bin), name: None }
    }
}
