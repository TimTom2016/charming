use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkTarget {
    #[serde(rename = "self")]
    Self_,
    Blank,
}
