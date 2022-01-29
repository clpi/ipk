use crate::{Config, Cmd, CliResult, cmd::RootCmd};
use clap::{Parser, AppSettings, ArgSettings, App, IntoApp};

#[derive(Debug, clap::Parser)]
pub struct HelpCmd {
    cmd: Option<String>,
}

impl Cmd for HelpCmd {
    fn exec(&self, config: &mut Config) -> CliResult<()> {
        println!("\x1b[35;1mHelp cmd executing...\x1b[0m");
        match self.cmd {
            None => HelpCmd::root(),
            Some(_) => HelpCmd::root(),
        }
        Ok(())
    }
}

impl Default for HelpCmd {
    fn default() -> Self {
        HelpCmd { cmd: None }
    }
}

impl HelpCmd {

    fn _custom_root() -> () {
        println!("\n\x1b[32;1mid-cli\x1b[0m root help\n");
        println!("\x1b[33;1mSUBCOMMANDS:\x1b[0m");
        println!(" - b | build\t\tBuild a project/file");
        println!(" - i | init\t\tInitialize new project/file");
        println!(" - h | help\t\tGet help for command");
    }

    fn root() -> () {
        RootCmd::into_app().print_help().unwrap();
    }
}
