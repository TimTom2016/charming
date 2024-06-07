use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid3D {}

impl Grid3D {
    pub fn new() -> Self {
        Self {}
    }
}
