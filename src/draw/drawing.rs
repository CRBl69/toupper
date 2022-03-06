use std::{collections::hash_map::HashMap, sync::Mutex};

use super::{instruction::Instruction, Layer};

#[derive(Debug)]
pub struct Drawing<'a> {
    pub layers: HashMap<String, Mutex<Layer<'a>>>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Default for Drawing<'a> {
    fn default() -> Drawing<'a> {
        Drawing::new(500, 500)
    }
}

impl<'a> Drawing<'a> {
    pub fn new(height: u32, width: u32) -> Drawing<'a> {
        let mut layers = HashMap::new();
        layers.insert(String::from("first layer"), Mutex::new(Layer::new()));
        Drawing {
            layers,
            width,
            height,
        }
    }

    pub fn new_layer(mut self, name: &str) -> Self {
        self.layers
            .insert(name.to_string(), Mutex::new(Layer::new()));
        self
    }

    pub fn instruct(&mut self, instruction: Box<dyn Instruction<'a>>, layer_name: &str) -> &Self {
        let layer = self.layers.get(layer_name);
        if let Some(l) = layer {
            let mut l = l.lock().unwrap();
            l.instruct(instruction);
        }
        self
    }
}
