use std::path::PathBuf;

use clap::{Subcommand, Parser, ArgSettings, AppSettings, ValueHint};
use crate::{Cmd, CliResult, CliError};

#[derive(Parser, Debug)]
pub struct InitCmd {
    #[clap(required = false)]
    name: Option<String>,
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
}
impl Cmd for InitCmd {

    fn exec(&self) -> CliResult<()> {
        let (dir, name) = self.dir_name();
        let config_tgt = dir.join("Idla.pkg.toml");
        println!("\x1b[35;1mExecuting init:\x1b[0m\nDir: {:?}\n Name {:?}\nConf: {:?} ...", 
            &dir.to_str(), &name, &config_tgt.to_str());
        Ok(())
    }
}

impl Default for InitCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        InitCmd { dir: Some(cwd), name: None }
    }
}
