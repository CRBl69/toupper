use crate::draw::{instruction::Instruction, ImageInsertion, Motion, Stroke};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    Stroke(Stroke),
    Image(ImageInsertion),
    Motion(Motion),
}
