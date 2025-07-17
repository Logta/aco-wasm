use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::{thread_rng, Rng};

mod city;
mod ant;
mod pheromone;
mod renderer;

use city::{Nest, FoodSource, Location};
use ant::{Ant, AntState};
// use pheromone::PheromoneMatrix; // Not used in safe implementation
use renderer::Renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Safe wrapper that avoids Rc issues by copying data
#[wasm_bindgen]
pub struct SafeEducationalACO {
    // Core state as simple values (no Rc)
    nest_x: f64,
    nest_y: f64,
    food_sources: Vec<(f64, f64, f64, f64)>, // (x, y, food_amount, max_food)
    ants: Vec<(usize, f64, f64, f64, f64, bool, f64, u8, f64, f64, usize)>, // (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food)
    pheromone_data: Vec<f64>, // Flattened matrix
    pheromone_size: usize,
    
    // Parameters
    num_ants: usize,
    alpha: f64,
    beta: f64,
    evaporation_rate: f64,
    q: f64,
    animation_speed: f64,
    show_trails: bool,
    show_pheromones: bool,
    
    // State
    is_running: bool,
    total_food_collected: f64,
    canvas_id: String,
}

#[wasm_bindgen]
impl SafeEducationalACO {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<SafeEducationalACO, JsValue> {
        console_error_panic_hook::set_once();
        console::log_1(&"Initializing Safe Educational Ant Foraging Simulation".into());
        
        Ok(SafeEducationalACO {
            nest_x: 400.0,
            nest_y: 300.0,
            food_sources: Vec::new(),
            ants: Vec::new(),
            pheromone_data: vec![1.0], // Start with single element for nest
            pheromone_size: 1,
            
            num_ants: 15,
            alpha: 1.0,
            beta: 2.0,
            evaporation_rate: 0.1,
            q: 100.0,
            animation_speed: 1.0,
            show_trails: true,
            show_pheromones: true,
            
            is_running: false,
            total_food_collected: 0.0,
            canvas_id: canvas_id.to_string(),
        })
    }
    
    pub fn add_city(&mut self, x: f64, y: f64) {
        // Don't allow food sources too close to nest
        let distance_to_nest = ((x - self.nest_x).powi(2) + (y - self.nest_y).powi(2)).sqrt();
        if distance_to_nest < 30.0 {
            return;
        }
        
        let mut rng = thread_rng();
        let food_amount = 50.0 + rng.gen::<f64>() * 50.0;
        self.food_sources.push((x, y, food_amount, food_amount));
        
        self.initialize_simulation();
    }
    
    pub fn remove_city(&mut self, x: f64, y: f64) {
        let click_radius = 15.0;
        let initial_len = self.food_sources.len();
        
        self.food_sources.retain(|(fx, fy, _, _)| {
            let distance = ((fx - x).powi(2) + (fy - y).powi(2)).sqrt();
            distance > click_radius
        });
        
        if self.food_sources.len() != initial_len {
            self.initialize_simulation();
        }
    }
    
    pub fn clear_cities(&mut self) {
        self.food_sources.clear();
        self.clear_simulation();
    }
    
    pub fn start(&mut self) {
        if !self.food_sources.is_empty() {
            self.is_running = true;
        }
    }
    
    pub fn pause(&mut self) {
        self.is_running = false;
    }
    
    pub fn reset(&mut self) {
        self.is_running = false;
        self.total_food_collected = 0.0;
        
        // Reset food sources to full
        for (_, _, food_amount, max_food) in &mut self.food_sources {
            *food_amount = *max_food;
        }
        
        if !self.food_sources.is_empty() {
            self.initialize_simulation();
        }
    }
    
    pub fn step(&mut self) {
        if !self.is_running || self.food_sources.is_empty() {
            return;
        }
        
        // Update each ant using a copy-based approach to avoid borrowing conflicts
        let mut ant_updates = Vec::new();
        
        // First pass: determine updates needed
        for i in 0..self.ants.len() {
            let ant = &self.ants[i];
            let (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, _total_food, _current_target_food) = *ant;
            
            if is_moving {
                // Continue moving towards target
                let new_progress = move_progress + self.animation_speed * 0.02;
                if new_progress >= 1.0 {
                    // Reached target
                    ant_updates.push((i, "move_complete", target_x, target_y, 0.0, 0.0, 0));
                } else {
                    // Interpolate position
                    let new_x = x + (target_x - x) * new_progress;
                    let new_y = y + (target_y - y) * new_progress;
                    ant_updates.push((i, "move_update", new_x, new_y, new_progress, 0.0, 0));
                }
            } else {
                // Ant has reached its destination
                match state {
                    0 => { // SearchingForFood
                        if let Some(food_idx) = self.ant_is_at_food_source(i) {
                            // Try to collect food
                            if self.food_sources[food_idx].2 > 0.0 {
                                let food_taken = 1.0_f64.min(self.food_sources[food_idx].2);
                                ant_updates.push((i, "collect_food", food_taken, 0.0, 0.0, food_idx as f64, 1));
                            }
                        } else {
                            // Look for a food source to visit
                            if let Some(food_idx) = self.select_food_source_for_ant(i) {
                                let target_x = self.food_sources[food_idx].0;
                                let target_y = self.food_sources[food_idx].1;
                                ant_updates.push((i, "move_to_food", target_x, target_y, 0.0, food_idx as f64, 0));
                            }
                        }
                    }
                    1 => { // CarryingFood
                        if self.ant_is_at_nest(i) {
                            // Deliver food
                            ant_updates.push((i, "deliver_food", carrying_food, 0.0, 0.0, 0.0, 0));
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Second pass: apply updates
        for (ant_idx, update_type, val1, val2, val3, val4, val5) in ant_updates {
            if ant_idx < self.ants.len() {
                let ant = &mut self.ants[ant_idx];
                match update_type {
                    "move_complete" => {
                        ant.1 = val1; // x
                        ant.2 = val2; // y
                        ant.5 = false; // is_moving
                        ant.6 = 1.0; // move_progress
                    }
                    "move_update" => {
                        ant.1 = val1; // x
                        ant.2 = val2; // y
                        ant.6 = val3; // move_progress
                    }
                    "collect_food" => {
                        let food_idx = val4 as usize;
                        if food_idx < self.food_sources.len() {
                            self.food_sources[food_idx].2 -= val1;
                            ant.8 = val1; // carrying_food
                            ant.9 += val1; // total_food
                            ant.3 = self.nest_x; // target_x
                            ant.4 = self.nest_y; // target_y
                            ant.5 = true; // is_moving
                            ant.6 = 0.0; // move_progress
                            ant.7 = 1; // state = CarryingFood
                            ant.10 = food_idx; // current_target_food
                            console::log_1(&format!("Ant {} collected food", ant.0).into());
                        }
                    }
                    "move_to_food" => {
                        ant.3 = val1; // target_x
                        ant.4 = val2; // target_y
                        ant.5 = true; // is_moving
                        ant.6 = 0.0; // move_progress
                        ant.10 = val4 as usize; // current_target_food
                    }
                    "deliver_food" => {
                        self.total_food_collected += val1;
                        ant.8 = 0.0; // carrying_food
                        ant.7 = 0; // state = SearchingForFood
                        console::log_1(&format!("Ant {} delivered {:.1} food. Total: {:.1}", ant.0, val1, self.total_food_collected).into());
                        
                        // Note: Next food search will happen in next step cycle
                    }
                    _ => {}
                }
            }
        }
        
        // Update pheromones periodically
        let mut rng = thread_rng();
        if rng.gen::<f64>() < 0.1 {
            self.update_pheromones();
        }
    }
    
    pub fn render(&self) {
        if let Ok(renderer) = Renderer::new(&self.canvas_id) {
            renderer.clear();
            
            // Draw nest
            let nest = Nest { location: Location::new(self.nest_x, self.nest_y) };
            renderer.draw_nest(&nest);
            
            // Draw food sources
            let food_sources: Vec<FoodSource> = self.food_sources.iter().map(|(x, y, amount, max)| {
                FoodSource {
                    location: Location::new(*x, *y),
                    food_amount: *amount,
                    max_food: *max,
                }
            }).collect();
            renderer.draw_food_sources(&food_sources);
            
            // Draw ants
            let ants: Vec<Ant> = self.ants.iter().map(|(id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food)| {
                Ant {
                    id: *id,
                    x: *x,
                    y: *y,
                    target_x: *target_x,
                    target_y: *target_y,
                    path: vec![], // Empty path for performance
                    is_moving: *is_moving,
                    move_progress: *move_progress,
                    state: if *state == 0 { AntState::SearchingForFood } else { AntState::CarryingFood },
                    carrying_food: *carrying_food,
                    total_food_collected: *total_food,
                    nest_location: Location::new(self.nest_x, self.nest_y),
                    current_target_food: if *current_target_food == usize::MAX { None } else { Some(*current_target_food) },
                }
            }).collect();
            renderer.draw_ants(&ants, self.show_trails);
        }
    }
    
    pub fn get_stats(&self) -> JsValue {
        let stats = js_sys::Object::new();
        
        js_sys::Reflect::set(&stats, &"iteration".into(), &0.into()).unwrap();
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
        // Limit food sources 
        const MAX_FOOD_SOURCES: usize = 20;
        if self.food_sources.len() > MAX_FOOD_SOURCES {
            self.food_sources.truncate(MAX_FOOD_SOURCES);
        }
        
        // Initialize pheromone matrix
        let matrix_size = 1 + self.food_sources.len();
        self.pheromone_size = matrix_size;
        self.pheromone_data = vec![1.0; matrix_size * matrix_size];
        
        // Create ants at nest
        self.ants.clear();
        for i in 0..self.num_ants {
            // (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food)
            self.ants.push((i, self.nest_x, self.nest_y, self.nest_x, self.nest_y, false, 0.0, 0, 0.0, 0.0, usize::MAX));
        }
    }
    
    fn clear_simulation(&mut self) {
        self.ants.clear();
        self.pheromone_size = 1;
        self.pheromone_data = vec![1.0];
        self.is_running = false;
        self.total_food_collected = 0.0;
    }
    
    fn ant_is_at_food_source(&self, ant_idx: usize) -> Option<usize> {
        if ant_idx >= self.ants.len() {
            return None;
        }
        
        let ant = &self.ants[ant_idx];
        for (i, (fx, fy, amount, _)) in self.food_sources.iter().enumerate() {
            if *amount > 0.0 {
                let distance = ((ant.1 - fx).powi(2) + (ant.2 - fy).powi(2)).sqrt();
                if distance < 10.0 {
                    return Some(i);
                }
            }
        }
        None
    }
    
    fn ant_is_at_nest(&self, ant_idx: usize) -> bool {
        if ant_idx >= self.ants.len() {
            return false;
        }
        
        let ant = &self.ants[ant_idx];
        let distance = ((ant.1 - self.nest_x).powi(2) + (ant.2 - self.nest_y).powi(2)).sqrt();
        distance < 10.0
    }
    
    fn select_food_source_for_ant(&self, ant_idx: usize) -> Option<usize> {
        if ant_idx >= self.ants.len() || self.ants[ant_idx].7 != 0 {
            return None;
        }
        
        let ant = &self.ants[ant_idx];
        let mut rng = thread_rng();
        let mut probabilities = Vec::new();
        let mut total_prob = 0.0;
        
        for (i, (fx, fy, amount, max_amount)) in self.food_sources.iter().enumerate() {
            if *amount > 0.0 {
                let distance = ((ant.1 - fx).powi(2) + (ant.2 - fy).powi(2)).sqrt();
                let food_ratio = if *max_amount > 0.0 { amount / max_amount } else { 0.0 };
                
                // Get pheromone level
                let pheromone = self.get_pheromone(0, i + 1);
                
                let distance_attractiveness = if distance > 0.0 { 1.0 / distance } else { 1.0 };
                let prob = pheromone.powf(self.alpha) * distance_attractiveness.powf(self.beta) * (1.0 + food_ratio * 2.0);
                
                if prob.is_finite() && prob > 0.0 {
                    probabilities.push((i, prob));
                    total_prob += prob;
                }
            }
        }
        
        if total_prob == 0.0 || probabilities.is_empty() {
            return None;
        }
        
        // Roulette wheel selection
        let random_val = rng.gen::<f64>() * total_prob;
        let mut cumulative = 0.0;
        
        for (food_idx, prob) in &probabilities {
            cumulative += prob;
            if cumulative >= random_val {
                return Some(*food_idx);
            }
        }
        
        probabilities.first().map(|(idx, _)| *idx)
    }
    
    fn get_pheromone(&self, i: usize, j: usize) -> f64 {
        if i < self.pheromone_size && j < self.pheromone_size {
            let idx = i * self.pheromone_size + j;
            if idx < self.pheromone_data.len() {
                return self.pheromone_data[idx];
            }
        }
        1.0
    }
    
    fn set_pheromone(&mut self, i: usize, j: usize, value: f64) {
        if i < self.pheromone_size && j < self.pheromone_size {
            let idx = i * self.pheromone_size + j;
            if idx < self.pheromone_data.len() {
                self.pheromone_data[idx] = value;
            }
        }
    }
    
    fn update_pheromones(&mut self) {
        // Evaporate pheromones
        for i in 0..self.pheromone_size {
            for j in 0..self.pheromone_size {
                if i != j {
                    let current = self.get_pheromone(i, j);
                    let new_value = (current * (1.0 - self.evaporation_rate)).max(0.01);
                    self.set_pheromone(i, j, new_value);
                }
            }
        }
        
        // Collect pheromone deposits first, then apply them
        let mut deposits = Vec::new();
        for ant in &self.ants {
            if ant.7 == 1 && ant.8 > 0.0 { // CarryingFood and has food
                if ant.10 != usize::MAX && ant.10 < self.food_sources.len() {
                    let pheromone_amount = self.q * ant.8;
                    let matrix_idx = ant.10 + 1;
                    if matrix_idx < self.pheromone_size {
                        deposits.push((0, matrix_idx, pheromone_amount));
                    }
                }
            }
        }
        
        // Apply deposits
        for (i, j, amount) in deposits {
            let current = self.get_pheromone(i, j);
            let new_value = (current + amount).min(1000.0);
            self.set_pheromone(i, j, new_value);
            self.set_pheromone(j, i, new_value);
        }
    }
}