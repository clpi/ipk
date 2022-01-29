use std::path::PathBuf;

use clap::{Parser, ValueHint, AppSettings, ArgSettings};
use crate::{CliError, CliResult, Cmd};

#[derive(Debug, Parser)]
#[clap()]
pub struct BuildCmd {
    dir: PathBuf,
    file: Option<PathBuf>,

}
impl Cmd for BuildCmd {
    fn exec(&self) -> CliResult<()> {
        println!("\x1b[33;1mExecuting build cmd...");
        
        Ok(())
    }

}

impl Default for BuildCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        BuildCmd { dir: cwd, file: None }
    }
}
