use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Serialize,Clone,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
