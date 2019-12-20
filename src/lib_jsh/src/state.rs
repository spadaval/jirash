use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Project {
    project_key: String,
}

#[derive(Debug)]
pub struct Jira {
    pub base_url: String,
    pub user: User,
    pub project: Option<Project>,
}

impl Jira {
    pub fn get_sitename(&self) -> String {
        return String::from(self.base_url.split(".").next().unwrap());
    }
    pub fn get_prompt(&self) -> String {
        match &self.project {
            Some(project) => format!("{}: {} >> ", self.get_sitename(), project.project_key),
            None => format!("{} >> ", self.get_sitename()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub token: String,
}

#[derive(Debug)]
pub struct State {
    pub jira: Jira,
}
