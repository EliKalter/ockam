mod addon;
mod authenticate;
mod create;
mod delete;
mod enroll;
mod info;
mod list;
mod show;
pub mod util;

pub use info::ProjectInfo;
pub use util::config;

use clap::{Args, Subcommand};

pub use crate::credential::get::GetCommand;
pub use addon::AddonCommand;
pub use create::CreateCommand;
pub use delete::DeleteCommand;
pub use enroll::EnrollCommand;
pub use info::InfoCommand;
pub use list::ListCommand;
pub use show::ShowCommand;

use crate::project::authenticate::AuthenticateCommand;
use crate::CommandGlobalOpts;

/// Manage Projects in Ockam Orchestrator
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct ProjectCommand {
    #[command(subcommand)]
    subcommand: ProjectSubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ProjectSubcommand {
    Create(CreateCommand),
    Delete(DeleteCommand),
    List(ListCommand),
    Show(ShowCommand),
    Information(InfoCommand),
    Enroll(EnrollCommand),
    Addon(AddonCommand),
    Authenticate(AuthenticateCommand),
}

impl ProjectCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        match self.subcommand {
            ProjectSubcommand::Create(c) => c.run(options),
            ProjectSubcommand::Delete(c) => c.run(options),
            ProjectSubcommand::List(c) => c.run(options),
            ProjectSubcommand::Show(c) => c.run(options),
            ProjectSubcommand::Enroll(c) => c.run(options),
            ProjectSubcommand::Information(c) => c.run(options),
            ProjectSubcommand::Addon(c) => c.run(options),
            ProjectSubcommand::Authenticate(c) => c.run(options),
        }
    }
}
