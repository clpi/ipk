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
    fn get() -> Self { Self::parse() }    
    fn exec(&self, config: &mut Config) -> CliResult<()> {
        Ok(())
    }
}

/// Root command
#[derive(Debug, Parser)]
#[clap(name = "idc", author, version, about, long_about = None)]
pub struct RootCmd {
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    profile: Option<String>,
    #[clap(subcommand)]
    cmd: Option<RootSubCmd>,
}
impl Cmd for RootCmd {
    fn exec(&self, config: &mut Config) -> CliResult<()> {
        println!("\x1b[32;1mRoot cmd executing...");
        let res: RootCmd = Self::parse();
        match res.cmd {
            Some(RootSubCmd::Help(hc)) => hc.exec(config),
            Some(RootSubCmd::Build(bc)) => bc.exec(config),
            Some(RootSubCmd::Init(ic)) => ic.exec(config),
            Some(RootSubCmd::Config(cc)) => cc.exec(config),
            Some(RootSubCmd::Run(rc)) => rc.exec(config),
            Some(RootSubCmd::Repl(rc)) => rc.exec(config),
            None => HelpCmd::default().exec(config)
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

