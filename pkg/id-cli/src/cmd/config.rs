use std::path::PathBuf;

use clap::{Parser, ValueHint, AppSettings, ArgSettings};
use crate::{CliError, CliResult, Cmd};

#[derive(Debug, Parser)]
#[clap()]
pub struct CfgCmd {
    dir: PathBuf,
    file: Option<PathBuf>,

}
impl Cmd for CfgCmd {
    fn exec(&self) -> CliResult<()> {
        println!("\x1b[33;1mExecuting Cfg cmd...");
        
        Ok(())
    }

}

impl Default for CfgCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        CfgCmd { dir: cwd, file: None }
    }
}
