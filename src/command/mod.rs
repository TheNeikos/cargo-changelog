use std::path::Path;

use crate::config::Configuration;

mod common;

mod add_command;
pub use self::add_command::AddCommand;

mod generate_command;
pub use self::generate_command::GenerateCommand;

mod release_command;
pub use self::release_command::ReleaseCommand;
pub use self::release_command::VersionData;

mod show;
pub use self::show::Show;

mod verify_metadata_command;
pub use self::verify_metadata_command::VerifyMetadataCommand;

pub trait Command {
    fn execute(self, workdir: &Path, config: &Configuration) -> Result<(), crate::error::Error>;
}
