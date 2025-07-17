use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::city::{Nest, FoodSource};
use crate::ant::{Ant, AntState};

pub struct Renderer {
    context: CanvasRenderingContext2d,
    canvas_width: f64,
    canvas_height: f64,
}

impl Renderer {
    pub fn new(canvas_id: &str) -> Result<Renderer, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;
        
        Ok(Renderer {
            context,
            canvas_width,
            canvas_height,
        })
    }
    
    pub fn clear(&self) {
        self.context.set_fill_style(&"#1a1a1a".into());
        self.context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
    }
    
    pub fn draw_nest(&self, nest: &Nest) {
        // Draw nest as a large brown circle
        self.context.begin_path();
        self.context.set_fill_style(&"#8B4513".into());
        self.context.arc(nest.location.x, nest.location.y, 15.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.context.fill();
        
        // Draw nest border
        self.context.begin_path();
        self.context.set_stroke_style(&"#654321".into());
        self.context.set_line_width(3.0);
        self.context.arc(nest.location.x, nest.location.y, 15.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        self.context.stroke();
        
        // Draw "巣" (nest) label
        self.context.set_fill_style(&"#FFFFFF".into());
        self.context.set_font("16px Arial");
        self.context.set_text_align("center");
        self.context.fill_text("巣", nest.location.x, nest.location.y + 5.0).unwrap();
    }
    
    pub fn draw_food_sources(&self, food_sources: &[FoodSource]) {
        for (i, food_source) in food_sources.iter().enumerate() {
            let food_ratio = food_source.food_ratio();
            let radius = 8.0 + food_ratio * 4.0; // Size based on food amount
            
            // Color based on food amount (green = full, red = empty)
            let green_component = (food_ratio * 255.0) as u8;
            let red_component = ((1.0 - food_ratio) * 255.0) as u8;
            let color = format!("rgb({}, {}, 0)", red_component, green_component);
            
            // Draw food source circle
            self.context.begin_path();
            self.context.set_fill_style(&color.into());
            self.context.arc(food_source.location.x, food_source.location.y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            self.context.fill();
            
            // Draw food source border
            self.context.begin_path();
            self.context.set_stroke_style(&"#333333".into());
            self.context.set_line_width(2.0);
            self.context.arc(food_source.location.x, food_source.location.y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            self.context.stroke();
            
            // Draw food amount text
            self.context.set_fill_style(&"#000000".into());
            self.context.set_font("10px Arial");
            self.context.set_text_align("center");
            self.context.fill_text(&format!("{:.0}", food_source.food_amount), food_source.location.x, food_source.location.y + 3.0).unwrap();
        }
    }
    
    pub fn draw_pheromone_trails(&self, pheromones: &[Vec<f64>], nest: &Nest, food_sources: &[FoodSource], show_pheromones: bool) {
        if !show_pheromones || food_sources.is_empty() {
            return;
        }
        
        let max_pheromone = pheromones.iter()
            .flat_map(|row| row.iter())
            .fold(0.0f64, |a, &b| a.max(b));
        
        if max_pheromone <= 0.0 {
            return;
        }
        
        // Draw pheromone trails from nest to each food source
        for (i, food_source) in food_sources.iter().enumerate() {
            if i < pheromones.len() && pheromones.len() > 0 {
                let pheromone_level = pheromones[0][i]; // From nest (index 0) to food source i
                let intensity = (pheromone_level / max_pheromone * 255.0).min(255.0) as u8;
                
                if intensity > 20 { // Only draw visible pheromone trails
                    self.context.begin_path();
                    self.context.set_stroke_style(&format!("rgba(0, 255, 255, {})", intensity as f64 / 255.0 * 0.6).into());
                    self.context.set_line_width(intensity as f64 / 255.0 * 3.0 + 1.0);
                    self.context.move_to(nest.location.x, nest.location.y);
                    self.context.line_to(food_source.location.x, food_source.location.y);
                    self.context.stroke();
                }
            }
        }
    }
    
    pub fn draw_ants(&self, ants: &[Ant], show_trails: bool) {
        // Draw ant trails
        if show_trails {
            for ant in ants {
                if ant.path.len() > 1 {
                    self.context.begin_path();
                    self.context.set_stroke_style(&self.get_ant_color(ant.id).into());
                    self.context.set_line_width(2.0);
                    
                    if let Some((start_x, start_y)) = ant.path.first() {
                        self.context.move_to(*start_x, *start_y);
                        for (x, y) in ant.path.iter().skip(1) {
                            self.context.line_to(*x, *y);
                        }
                        self.context.stroke();
                    }
                }
            }
        }
        
        // Draw ants
        for ant in ants {
            let color = self.get_ant_color(ant.id);
            let radius = if ant.state == AntState::CarryingFood { 5.0 } else { 4.0 };
            
            // Draw ant body
            self.context.begin_path();
            self.context.set_fill_style(&color.into());
            self.context.arc(ant.x, ant.y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            self.context.fill();
            
            // Draw ant border
            self.context.begin_path();
            self.context.set_stroke_style(&"#000000".into());
            self.context.set_line_width(1.0);
            self.context.arc(ant.x, ant.y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            self.context.stroke();
            
            // Draw food indicator if carrying food
            if ant.state == AntState::CarryingFood && ant.carrying_food > 0.0 {
                self.context.begin_path();
                self.context.set_fill_style(&"#FFD700".into());
                self.context.arc(ant.x, ant.y - 8.0, 2.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                self.context.fill();
            }
        }
    }
    
    
    fn get_ant_color(&self, ant_id: usize) -> &str {
        match ant_id % 6 {
            0 => "#FF6B6B",
            1 => "#4ECDC4", 
            2 => "#45B7D1",
            3 => "#96CEB4",
            4 => "#FFEAA7",
            _ => "#DDA0DD",
        }
    }
}