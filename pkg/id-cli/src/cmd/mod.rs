pub mod init;
pub mod build;
pub mod help;
pub mod config;
pub mod run;
pub mod repl;

use clap::{Parser, Subcommand, ValueHint, AppSettings, ArgSettings};
use crate::{CliResult, config::Config};
pub use self::{
    config::CfgCmd, 
    repl::ReplCmd, 
    run::RunCmd, 
    help::HelpCmd, 
    build::BuildCmd, 
    init::InitCmd
};

pub trait Cmd: Parser + Default {
    fn with_config(cfg: Config) -> Self {
        return Self::parse();
    }
    fn get() -> Self { Self::parse() }    
    fn exec(&self) -> CliResult<()> {
        Ok(())
    }
}

#[derive(Debug, Parser)]
#[clap()]
pub struct RootCmd {
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    profile: Option<String>,
    #[clap(subcommand)]
    cmd: Option<RootSubCmd>,
}
impl Cmd for RootCmd {
    fn exec(&self) -> CliResult<()> {
        println!("\x1b[32;1mRoot cmd executing...");
        let res: RootCmd = Self::parse();
        match res.cmd {
            Some(RootSubCmd::Help(hc)) => hc.exec(),
            Some(RootSubCmd::Build(bc)) => bc.exec(),
            Some(RootSubCmd::Init(ic)) => ic.exec(),
            Some(RootSubCmd::Config(cc)) => cc.exec(),
            Some(RootSubCmd::Run(rc)) => rc.exec(),
            Some(RootSubCmd::Repl(rc)) => rc.exec(),
            None => HelpCmd::default().exec()
        }
    }
}
impl Default for RootCmd {
    fn default() -> Self {
        Self { 
            debug: false, 
            cmd: None,
            profile: None,
        }
    }
}

#[derive(Debug, Subcommand)]
#[clap()]
pub enum RootSubCmd {
    Init(init::InitCmd),
    Build(build::BuildCmd),
    Help(help::HelpCmd),
    Config(CfgCmd),
    Repl(ReplCmd),
    Run(RunCmd),
}

impl RootCmd {

}

