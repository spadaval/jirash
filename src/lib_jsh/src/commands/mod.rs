pub mod api;
pub mod registry;
use registry::CommandRegistry;

mod whoami;

pub fn build_command_registry() -> CommandRegistry {
    let mut reg = CommandRegistry::new();
    reg.add_command(whoami::build_command());
    reg
}
