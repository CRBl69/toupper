use super::Instruction;
use super::Stroke;

#[derive(Default, Debug)]
pub struct Layer<'a> {
    pub history: Vec<Box<dyn Instruction<'a>>>,
    pub history_index: usize,
}

impl<'a> Layer<'a> {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.history_index = 0;
    }

    pub fn get_rendered_strokes(&self) -> Vec<&dyn Instruction<'a>> {
        let mut instructions = Vec::new();
        for i in 0..self.history_index {
            instructions.push((*self.history.get(i).unwrap()).as_ref());
        }
        instructions
    }

    pub fn instruct(&mut self, instruction: Box<dyn Instruction<'a>>) -> &Self {
        self.history.push(instruction);
        self
    }
}
