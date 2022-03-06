use serde::{Deserialize, Serialize};

use super::Instruction;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInsertion {
    base64: String,
}

impl ImageInsertion {
    pub fn new(base64: &str) -> Self {
        ImageInsertion {
            base64: String::from(base64),
        }
    }
}

impl<'a> Instruction<'a> for ImageInsertion {}
