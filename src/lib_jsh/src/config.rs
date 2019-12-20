use crate::state;

use serde::{Deserialize, Serialize};
//use crate::state::state;
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraConfig {
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplConfig {
    pub user: state::User,
    pub jira: JiraConfig,
}

impl Default for ReplConfig {
    fn default() -> ReplConfig {
        return ReplConfig {
            user: state::User {
                username: String::from(""),
                token: String::from(""),
            },
            jira: JiraConfig {
                base_url: String::from(""),
            },
        };
    }
}
