use std::path::PathBuf;

use clap::{Parser, ValueHint, AppSettings, ArgSettings};
use crate::{CliError, CliResult, Cmd, Config, };

#[derive(Debug, Parser)]
#[clap()]
pub struct RunCmd {
    dir: PathBuf,
    file: Option<PathBuf>,

}
impl Cmd for RunCmd {
    fn exec(&self, config: &mut Config) -> CliResult<()> {
        println!("\x1b[33;1mExecuting Run cmd...");
        
        Ok(())
    }

}

impl Default for RunCmd {
    fn default() -> Self {
        let cwd = std::env::current_dir().unwrap_or(PathBuf::from("~/"));
        RunCmd { dir: cwd, file: None }
    }
}
