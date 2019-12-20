use crate::commands::registry::CommandRegistry;
use crate::config::ReplConfig;
use crate::state;

#[derive(Debug)]
enum Context {
    SITE,
    PROJECT,
    ISSUE,
}

#[derive(Debug)]
pub struct Shell {
    state: state::State,
    context: Context,
    commands: CommandRegistry,
}

impl Shell {
    pub fn from_config(config: ReplConfig) -> Shell {
        return Shell {
            context: Context::SITE,
            commands: CommandRegistry::new(),
            state: state::State {
                jira: state::Jira {
                    base_url: config.jira.base_url,
                    project: None,
                    user: state::User {
                        username: config.user.username,
                        token: config.user.token,
                    },
                },
            },
        };
    }
    pub fn build_prompt(&self) -> String {
        self.state.jira.get_prompt()
    }
    //pub fn run_command(&self) -> Result<u64, std::io::Error> {}
}
