use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(tag = "todo")]
pub struct Todo {
    pub name: String,
    pub date_time: u64,
    pub grandparent: String,
    pub parent: String,
    pub id: String
}
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    MyCustomCommand { argument: String },
    SpitTodos { name: String,
        date_time: u64,
        grandparent: String,
        parent: String,
        id: String,
        callback: String,
        error: String
    }

}
