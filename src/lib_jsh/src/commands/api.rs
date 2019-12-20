use std::collections::HashMap;
type Callback = Box<FnMut(String)>;

pub struct Command {
    keyword: String,
    callback: Callback,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ keyword: {} }}", self.keyword)
    }
}

impl Command {
    pub fn new(keyword: String, callback: Callback) -> Command {
        Command {
            keyword: keyword,
            callback: callback,
        }
    }
    pub fn keyword(&self) -> String {
        self.keyword.clone()
    }
}

struct ExecutionResult {}
