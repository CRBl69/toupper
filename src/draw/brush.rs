use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct Brush {
    pub brush_type: BrushType,
    pub color: Color,
    pub width: f32,
    pub opacity: f32,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            brush_type: BrushType::Solid,
            color: Color::default(),
            width: 1.0,
            opacity: 1.0,
        }
    }
}

impl Brush {
    pub fn new(brush_type: BrushType, color: Color, width: f32, opacity: f32) -> Self {
        Brush {
            brush_type,
            color,
            width,
            opacity,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    pub fn set_brush_type(&mut self, brush_type: BrushType) {
        self.brush_type = brush_type;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BrushType {
    Solid,
    Eraser,
    Brush(usize),
}
