use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::{thread_rng, Rng};

mod city;
mod ant;
mod pheromone;
mod renderer;

use city::{Nest, FoodSource, Location};
use ant::{Ant, AntState};
use pheromone::PheromoneMatrix;
use renderer::Renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct EducationalACO {
    nest: Nest,
    food_sources: Vec<FoodSource>,
    ants: Vec<Ant>,
    pheromones: PheromoneMatrix,
    renderer: Renderer,
    
    // ACO Parameters
    num_ants: usize,
    alpha: f64,      // Pheromone importance
    beta: f64,       // Distance importance  
    evaporation_rate: f64,
    q: f64,          // Pheromone deposit factor
    
    // Animation settings
    animation_speed: f64,
    show_trails: bool,
    show_pheromones: bool,
    
    // State
    is_running: bool,
    total_food_collected: f64,
    
    // Random number generator
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl EducationalACO {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<EducationalACO, JsValue> {
        console_error_panic_hook::set_once();
        console::log_1(&"Initializing Educational Ant Foraging Simulation".into());
        
        let renderer = Renderer::new(canvas_id)?;
        let pheromones = PheromoneMatrix::new(1, 1.0); // Start with 1 (nest)
        
        // Create nest at center of canvas
        let nest = Nest::new(400.0, 300.0);
        
        Ok(EducationalACO {
            nest,
            food_sources: Vec::new(),
            ants: Vec::new(),
            pheromones,
            renderer,
            
            // Default ACO parameters
            num_ants: 15,
            alpha: 1.0,
            beta: 2.0,
            evaporation_rate: 0.1,
            q: 100.0,
            
            // Default animation settings
            animation_speed: 1.0,
            show_trails: true,
            show_pheromones: true,
            
            // Initial state
            is_running: false,
            total_food_collected: 0.0,
            
            rng: thread_rng(),
        })
    }
    
    pub fn add_city(&mut self, x: f64, y: f64) {
        console::log_1(&format!("Adding food source at ({}, {})", x, y).into());
        
        // Don't allow food sources too close to nest
        let distance_to_nest = Location::new(x, y).distance_to(&self.nest.location);
        if distance_to_nest < 30.0 {
            console::log_1(&"Food source too close to nest, ignoring".into());
            return;
        }
        
        let food_amount = 50.0 + self.rng.gen::<f64>() * 50.0; // 50-100 units of food
        self.food_sources.push(FoodSource::new(x, y, food_amount));
        
        // Reinitialize pheromone matrix and ants
        self.initialize_simulation();
    }
    
    pub fn remove_city(&mut self, x: f64, y: f64) {
        let click_radius = 15.0;
        let initial_len = self.food_sources.len();
        
        self.food_sources.retain(|food_source| {
            let distance = ((food_source.location.x - x).powi(2) + (food_source.location.y - y).powi(2)).sqrt();
            distance > click_radius
        });
        
        if self.food_sources.len() != initial_len {
            console::log_1(&format!("Removed food source near ({}, {})", x, y).into());
            self.initialize_simulation();
        }
    }
    
    pub fn clear_cities(&mut self) {
        console::log_1(&"Clearing all food sources".into());
        self.food_sources.clear();
        self.clear_simulation();
    }
    
    pub fn start(&mut self) {
        if !self.food_sources.is_empty() {
            console::log_1(&"Starting ant foraging simulation".into());
            self.is_running = true;
        } else {
            console::log_1(&"Need at least 1 food source to start simulation".into());
        }
    }
    
    pub fn pause(&mut self) {
        console::log_1(&"Pausing simulation".into());
        self.is_running = false;
    }
    
    pub fn reset(&mut self) {
        console::log_1(&"Resetting simulation".into());
        self.is_running = false;
        self.total_food_collected = 0.0;
        
        // Reset food sources to full
        for food_source in &mut self.food_sources {
            food_source.food_amount = food_source.max_food;
        }
        
        if !self.food_sources.is_empty() {
            self.initialize_simulation();
        }
    }
    
    pub fn step(&mut self) {
        if !self.is_running || self.food_sources.is_empty() {
            return;
        }
        
        // Update each ant
        for i in 0..self.ants.len() {
            if self.ants[i].is_moving {
                // Continue moving towards target
                self.ants[i].update_movement(self.animation_speed * 0.02);
            } else {
                // Ant has reached its destination, decide what to do next
                match self.ants[i].state {
                    AntState::SearchingForFood => {
                        // Try to find food at current location
                        if self.ants[i].is_at_food_source(&self.food_sources) {
                            // Try to collect food
                            if self.ants[i].collect_food(&mut self.food_sources) {
                                console::log_1(&format!("Ant {} collected food", self.ants[i].id).into());
                                self.ants[i].start_return_to_nest();
                            }
                        } else {
                            // Look for a food source to visit
                            let selected_food = {
                                let ant = &self.ants[i];
                                ant.select_food_source(&self.food_sources, self.pheromones.get_matrix(), self.alpha, self.beta, &mut self.rng)
                            };
                            if let Some(food_idx) = selected_food {
                                self.ants[i].start_move_to_food(food_idx, &self.food_sources);
                            }
                        }
                    }
                    AntState::CarryingFood => {
                        // Check if at nest
                        if self.ants[i].is_at_nest() {
                            // Deliver food
                            let delivered = self.ants[i].deliver_food();
                            self.total_food_collected += delivered;
                            console::log_1(&format!("Ant {} delivered {:.1} food. Total: {:.1}", self.ants[i].id, delivered, self.total_food_collected).into());
                            
                            // Start searching for food again
                            let selected_food = {
                                let ant = &self.ants[i];
                                ant.select_food_source(&self.food_sources, self.pheromones.get_matrix(), self.alpha, self.beta, &mut self.rng)
                            };
                            if let Some(food_idx) = selected_food {
                                self.ants[i].start_move_to_food(food_idx, &self.food_sources);
                            }
                        }
                    }
                }
            }
        }
        
        // Update pheromones periodically
        if self.rng.gen::<f64>() < 0.1 { // 10% chance each step
            self.update_pheromones();
        }
    }
    
    pub fn render(&self) {
        // Safety check before rendering
        if self.ants.len() > 100 {
            console::log_1(&"Warning: Large number of ants detected".into());
        }
        
        self.renderer.clear();
        
        // Draw pheromone trails first (background)
        if self.show_pheromones && !self.food_sources.is_empty() {
            self.renderer.draw_pheromone_trails(self.pheromones.get_matrix(), &self.nest, &self.food_sources, true);
        }
        
        // Draw nest
        self.renderer.draw_nest(&self.nest);
        
        // Draw food sources
        self.renderer.draw_food_sources(&self.food_sources);
        
        // Draw ants
        self.renderer.draw_ants(&self.ants, self.show_trails);
    }
    
    pub fn get_stats(&self) -> JsValue {
        let stats = js_sys::Object::new();
        
        js_sys::Reflect::set(&stats, &"iteration".into(), &0.into()).unwrap(); // Not used in foraging
        js_sys::Reflect::set(&stats, &"cities_count".into(), &self.food_sources.len().into()).unwrap();
        js_sys::Reflect::set(&stats, &"ants_count".into(), &self.ants.len().into()).unwrap();
        js_sys::Reflect::set(&stats, &"best_distance".into(), &self.total_food_collected.into()).unwrap();
        
        let state = if self.is_running { "running" } else { "idle" };
        js_sys::Reflect::set(&stats, &"state".into(), &state.into()).unwrap();
        
        stats.into()
    }
    
    // Parameter setters
    pub fn set_animation_speed(&mut self, speed: f64) {
        self.animation_speed = speed.max(0.1).min(5.0);
    }
    
    pub fn set_show_ant_trails(&mut self, show: bool) {
        self.show_trails = show;
    }
    
    pub fn set_show_pheromone_levels(&mut self, show: bool) {
        self.show_pheromones = show;
    }
    
    
    pub fn set_aco_param(&mut self, param: &str, value: f64) {
        match param {
            "alpha" => self.alpha = value.max(0.1).min(5.0),
            "beta" => self.beta = value.max(0.1).min(5.0),
            "evaporation" => self.evaporation_rate = value.max(0.01).min(0.5),
            "num_ants" => {
                self.num_ants = (value as usize).max(5).min(50);
                if !self.food_sources.is_empty() {
                    self.initialize_simulation();
                }
            },
            _ => {}
        }
    }
    
    // Private helper methods
    fn initialize_simulation(&mut self) {
        // Safety check: limit food sources to prevent excessive memory usage
        const MAX_FOOD_SOURCES: usize = 20;
        if self.food_sources.len() > MAX_FOOD_SOURCES {
            self.food_sources.truncate(MAX_FOOD_SOURCES);
        }
        
        // Initialize pheromone matrix (nest + food sources)
        let matrix_size = 1 + self.food_sources.len(); // 1 for nest + food sources
        self.pheromones = PheromoneMatrix::new(matrix_size, 1.0);
        
        // Create ants at nest
        self.ants.clear();
        for i in 0..self.num_ants {
            let ant = Ant::new(i, &self.nest);
            self.ants.push(ant);
        }
        
        console::log_1(&format!("Initialized simulation with {} food sources and {} ants", self.food_sources.len(), self.num_ants).into());
    }
    
    fn clear_simulation(&mut self) {
        self.ants.clear();
        self.pheromones = PheromoneMatrix::new(1, 1.0); // Keep nest
        self.is_running = false;
        self.total_food_collected = 0.0;
    }
    
    fn update_pheromones(&mut self) {
        // Safety check for pheromone matrix size
        let expected_size = 1 + self.food_sources.len();
        if self.pheromones.get_matrix().len() != expected_size {
            // Reinitialize if size mismatch
            self.pheromones = PheromoneMatrix::new(expected_size, 1.0);
            return;
        }
        
        // Evaporate pheromones
        self.pheromones.evaporate(self.evaporation_rate);
        
        // Deposit pheromones from ants carrying food
        for ant in &self.ants {
            if ant.state == AntState::CarryingFood && ant.carrying_food > 0.0 {
                // Deposit pheromone on the path from current position towards nest
                if let Some(target_food_idx) = ant.current_target_food {
                    if target_food_idx < self.food_sources.len() {
                        // Deposit pheromone on nest->food path based on food carried
                        let pheromone_amount = self.q * ant.carrying_food;
                        let matrix_idx = target_food_idx + 1; // +1 because nest is index 0
                        if matrix_idx < expected_size {
                            self.pheromones.deposit(0, matrix_idx, pheromone_amount);
                        }
                    }
                }
            }
        }
    }
}