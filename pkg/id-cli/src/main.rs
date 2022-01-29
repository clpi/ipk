pub(crate) mod cmd;
pub(crate) mod pkg;
pub(crate) mod repo; 
pub(crate) mod data;
pub(crate) mod config;
pub(crate) mod error;

pub use pkg::PkgConfig;
pub use error::{CliError, CliResult};
pub use self::{cmd::Cmd, config::Config};

fn main() -> CliResult<()> {
    let cmd = cmd::RootCmd::get();
    cmd.exec(&mut config::Config::get()?)?;
    Ok(())
}
