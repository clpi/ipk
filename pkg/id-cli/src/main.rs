pub(crate) mod cmd;
pub(crate) mod pkg;
pub(crate) mod repo; 
pub(crate) mod data;
pub(crate) mod config;
pub(crate) mod error;

pub use pkg::PkgConfig;
pub use error::{CliError, CliResult};
pub use cmd::Cmd;

fn main() -> CliResult<()> {
    let conf = config::Config::get()?;
    let cmd = cmd::RootCmd::with_config(conf);
    cmd.exec()?;
    Ok(())
}
