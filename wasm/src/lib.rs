pub mod aco;
pub mod geometry;
pub mod rendering;
pub mod simulation;
pub mod input;

use wasm_bindgen::prelude::*;
use aco::colony::{Colony, ACOParameters};
use geometry::city::City;
use rendering::{CanvasRenderer, AnimationManager};
use web_sys::HtmlCanvasElement;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct ACOEngine {
    colony: Option<Colony>,
    cities: Vec<City>,
    renderer: Option<CanvasRenderer>,
    animation_manager: AnimationManager,
    is_running: bool,
}

#[wasm_bindgen]
impl ACOEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ACOEngine {
        ACOEngine {
            colony: None,
            cities: Vec::new(),
            renderer: None,
            animation_manager: AnimationManager::new(),
            is_running: false,
        }
    }

    #[wasm_bindgen]
    pub fn initialize_canvas(&mut self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
        let renderer = CanvasRenderer::new(canvas)?;
        self.renderer = Some(renderer);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize_canvas(&mut self, width: u32, height: u32) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height);
        }
    }

    #[wasm_bindgen]
    pub fn add_city(&mut self, x: f64, y: f64) -> u32 {
        let id = self.cities.len() as u32;
        let city = City::new(id, x, y);
        self.cities.push(city);
        
        // Re-render safely if canvas is initialized
        if self.renderer.is_some() {
            self.safe_render();
        }
        
        id
    }

    #[wasm_bindgen]
    pub fn clear_cities(&mut self) {
        self.cities.clear();
        self.colony = None;
        self.animation_manager.clear();
        
        // Re-render safely if canvas is initialized
        if self.renderer.is_some() {
            self.safe_render();
        }
    }

    #[wasm_bindgen]
    pub fn get_city_count(&self) -> usize {
        self.cities.len()
    }

    #[wasm_bindgen]
    pub fn initialize_colony(&mut self, num_ants: usize, max_generations: usize, evaporation_rate: f64, alpha: f64, beta: f64) {
        if self.cities.len() < 3 {
            return;
        }

        let parameters = ACOParameters {
            num_ants,
            max_generations,
            evaporation_rate,
            alpha,
            beta,
            initial_pheromone: 1.0,
        };

        self.colony = Some(Colony::new(self.cities.clone(), parameters));
        
        // Initialize ant animations
        self.animation_manager.clear();
        for i in 0..num_ants {
            let start_city = &self.cities[i % self.cities.len()];
            self.animation_manager.add_ant(i as u32, start_city.x(), start_city.y());
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.is_running = true;
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.is_running = false;
        // Reset animation state safely
        if self.colony.is_some() {
            self.animation_manager.clear();
        }
    }

    #[wasm_bindgen]
    pub fn run_iteration(&mut self) -> bool {
        if !self.is_running {
            return false;
        }

        if let Some(colony) = &mut self.colony {
            colony.run_iteration()
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        // Always use safe rendering to prevent index out of bounds
        self.safe_render();
    }

    #[wasm_bindgen]
    pub fn update_animation(&mut self, timestamp: f64) -> bool {
        self.animation_manager.update(timestamp)
    }

    #[wasm_bindgen]
    pub fn set_animation_speed(&mut self, speed: f64) {
        self.animation_manager.set_animation_speed(speed);
    }

    #[wasm_bindgen]
    pub fn get_best_distance(&self) -> f64 {
        if let Some(colony) = &self.colony {
            colony.best_distance()
        } else {
            f64::INFINITY
        }
    }

    #[wasm_bindgen]
    pub fn get_generation(&self) -> usize {
        if let Some(colony) = &self.colony {
            colony.generation()
        } else {
            0
        }
    }

    #[wasm_bindgen]
    pub fn get_best_route(&self) -> Vec<u32> {
        if let Some(colony) = &self.colony {
            if let Some(route) = colony.best_route() {
                route.iter().map(|&x| x as u32).collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    #[wasm_bindgen]
    pub fn is_complete(&self) -> bool {
        if let Some(colony) = &self.colony {
            colony.is_complete()
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn get_pheromone_matrix_flat(&self) -> Vec<f64> {
        if let Some(_colony) = &self.colony {
            // This would need to be implemented in Colony to expose pheromone matrix
            // For now, return empty vec
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn get_max_pheromone(&self) -> f64 {
        1.0 // Default max pheromone value
    }

    fn cities_to_json(&self) -> String {
        let mut json = String::from("[");
        for (i, city) in self.cities.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                "{{\"id\":{},\"x\":{},\"y\":{}}}",
                city.id(),
                city.x(),
                city.y()
            ));
        }
        json.push(']');
        json
    }

    // Safe rendering method that doesn't cause index out of bounds
    fn safe_render(&mut self) {
        if let Some(renderer) = &self.renderer {
            renderer.clear();
            
            // Draw cities first - this is always safe
            for city in &self.cities {
                renderer.draw_city(city, false);
            }
            
            // Only draw complex elements if we have a proper colony setup
            if let Some(colony) = &self.colony {
                // Draw pheromone trails and best route only if we have enough cities
                if self.cities.len() >= 3 {
                    let pheromone_matrix = self.get_pheromone_matrix_flat();
                    let max_pheromone = self.get_max_pheromone();
                    let cities_json = self.cities_to_json();
                    renderer.draw_pheromone_trail_simple(&cities_json, &pheromone_matrix, max_pheromone);

                    // Draw best route if available
                    if let Some(best_route) = colony.best_route() {
                        let route: Vec<u32> = best_route.iter().map(|&x| x as u32).collect();
                        renderer.draw_route_simple(&cities_json, &route, "#ef4444", 3.0);
                    }
                }
                
                // Draw ants only if animation manager is properly initialized
                let ant_count = self.animation_manager.get_active_ant_count();
                if ant_count > 0 && ant_count <= self.cities.len() as u32 {
                    for i in 0..ant_count {
                        let position = self.animation_manager.get_ant_position(i);
                        if position.len() == 2 {
                            let angle = self.animation_manager.get_ant_angle(i);
                            renderer.draw_ant(position[0], position[1], angle);
                        }
                    }
                }
            }
        }
    }
}

// Export public types for external use
// Note: These are already imported above, so we don't need to re-export them