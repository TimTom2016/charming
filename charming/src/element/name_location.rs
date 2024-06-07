use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NameLocation {
    Start,
    Middle,
    Center,
    End,
}
