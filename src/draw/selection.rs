use serde::{Deserialize, Serialize};

use super::{Instruction, Point};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Select {
    pub up_left: Point,
    pub down_right: Point,
}

impl Select {
    pub fn new(up_left: Point, down_right: Point) -> Self {
        Select {
            up_left,
            down_right,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deselect {}

impl Deselect {
    pub fn new() -> Self {
        Deselect {}
    }
}

impl<'a> Instruction<'a> for Select {}
impl<'a> Instruction<'a> for Deselect {}
