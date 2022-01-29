use std::path::PathBuf;

use clap::{Parser, ValueHint, AppSettings, ArgSettings};
use crate::{Config, CliError, CliResult, config, Cmd};

#[derive(Debug, Parser)]
#[clap()]
pub struct ReplCmd {
    dir: PathBuf,
    file: Option<PathBuf>,

}
impl Cmd for ReplCmd {
    fn exec(&self, config: &mut config::Config) -> CliResult<()> {
        println!("\x1b[33;1mExecuting Repl cmd...");
        
        Ok(())
    }

}

impl Default for ReplCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        ReplCmd { dir: cwd, file: None }
    }
}
