use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::{thread_rng, Rng};
use std::sync::Mutex;

mod city;
mod ant;
mod pheromone;
mod renderer;

use city::{Nest, FoodSource, Location};
use ant::{Ant, AntState};
use renderer::Renderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Global state that completely avoids wasm-bindgen Rc management
static GLOBAL_STATE: Mutex<Option<SimulationState>> = Mutex::new(None);

pub struct SimulationState {
    // Core state as simple values (no Rc)
    nest_x: f64,
    nest_y: f64,
    food_sources: Vec<(f64, f64, f64, f64)>, // (x, y, food_amount, max_food)
    ants: Vec<(usize, f64, f64, f64, f64, bool, f64, u8, f64, f64, usize, f64, f64, u32)>, // (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food, direction_angle, move_speed, exploration_timer)
    pheromone_data: Vec<f64>, // Flattened matrix
    pheromone_size: usize,
    
    // 2D pheromone grid for spatial pheromones
    pheromone_grid: Vec<f64>, // Flattened 2D grid
    grid_width: usize,
    grid_height: usize,
    grid_cell_size: f64,
    
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

impl SimulationState {
    fn new(canvas_id: &str) -> Self {
        let grid_cell_size = 10.0; // 10x10 pixel cells
        let grid_width = (800.0 / grid_cell_size) as usize;
        let grid_height = (600.0 / grid_cell_size) as usize;
        
        SimulationState {
            nest_x: 400.0,
            nest_y: 300.0,
            food_sources: Vec::new(),
            ants: Vec::new(),
            pheromone_data: vec![1.0], // Start with single element for nest
            pheromone_size: 1,
            
            // Initialize pheromone grid
            pheromone_grid: vec![0.0; grid_width * grid_height],
            grid_width,
            grid_height,
            grid_cell_size,
            
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
        }
    }
}

// Global functions that work with the global state
#[wasm_bindgen]
pub fn initialize_simulation(canvas_id: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&"Initializing Global Educational Ant Foraging Simulation".into());
    
    let mut state = GLOBAL_STATE.lock().unwrap();
    *state = Some(SimulationState::new(canvas_id));
    
    Ok(())
}

#[wasm_bindgen]
pub fn add_city(x: f64, y: f64) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        // Don't allow food sources too close to nest
        let distance_to_nest = ((x - state.nest_x).powi(2) + (y - state.nest_y).powi(2)).sqrt();
        if distance_to_nest < 30.0 {
            return;
        }
        
        let mut rng = thread_rng();
        let food_amount = 50.0 + rng.gen::<f64>() * 50.0;
        state.food_sources.push((x, y, food_amount, food_amount));
        
        initialize_simulation_internal(state);
    }
}

#[wasm_bindgen]
pub fn remove_city(x: f64, y: f64) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        let click_radius = 15.0;
        let initial_len = state.food_sources.len();
        
        state.food_sources.retain(|(fx, fy, _, _)| {
            let distance = ((fx - x).powi(2) + (fy - y).powi(2)).sqrt();
            distance > click_radius
        });
        
        if state.food_sources.len() != initial_len {
            initialize_simulation_internal(state);
        }
    }
}

#[wasm_bindgen]
pub fn clear_cities() {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.food_sources.clear();
        clear_simulation_internal(state);
    }
}

#[wasm_bindgen]
pub fn start_simulation() {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        if !state.food_sources.is_empty() {
            state.is_running = true;
        }
    }
}

#[wasm_bindgen]
pub fn pause_simulation() {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.is_running = false;
    }
}

#[wasm_bindgen]
pub fn reset_simulation() {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.is_running = false;
        state.total_food_collected = 0.0;
        
        // Reset food sources to full
        for (_, _, food_amount, max_food) in &mut state.food_sources {
            *food_amount = *max_food;
        }
        
        if !state.food_sources.is_empty() {
            initialize_simulation_internal(state);
        }
    }
}

#[wasm_bindgen]
pub fn step_simulation() {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        if !state.is_running || state.food_sources.is_empty() {
            return;
        }
        
        // Update each ant using a copy-based approach
        let mut ant_updates = Vec::new();
        
        // First pass: determine updates needed
        for i in 0..state.ants.len() {
            let ant = &state.ants[i];
            let (_id, x, y, _target_x, _target_y, _is_moving, _move_progress, ant_state, carrying_food, _total_food, _current_target_food, direction_angle, move_speed, exploration_timer) = *ant;
            
            match ant_state {
                0 => { // SearchingForFood - Random walk exploration
                    // Check if ant found food at current position
                    if let Some(food_idx) = ant_is_at_food_source_internal(state, i) {
                        // Try to collect food
                        if state.food_sources[food_idx].2 > 0.0 {
                            let food_taken = 1.0_f64.min(state.food_sources[food_idx].2);
                            ant_updates.push((i, "collect_food", food_taken, 0.0, 0.0, food_idx as f64, 1));
                        }
                    } else {
                        // Random walk exploration
                        let (new_x, new_y, new_angle, new_timer) = calculate_random_walk_step(state, i, x, y, direction_angle, move_speed, exploration_timer);
                        ant_updates.push((i, "random_walk", new_x, new_y, new_angle, 0.0, new_timer as i32));
                    }
                }
                1 => { // CarryingFood - Move towards nest with pheromone trail
                    if ant_is_at_nest_internal(state, i) {
                        // Deliver food
                        ant_updates.push((i, "deliver_food", carrying_food, 0.0, 0.0, 0.0, 0));
                    } else {
                        // Move towards nest
                        let (new_x, new_y, new_angle) = calculate_return_to_nest_step(state, i, x, y, move_speed);
                        ant_updates.push((i, "return_to_nest", new_x, new_y, new_angle, 0.0, 0));
                    }
                }
                _ => {}
            }
        }
        
        // Second pass: apply updates
        for (ant_idx, update_type, val1, val2, val3, val4, val5) in ant_updates {
            if ant_idx < state.ants.len() {
                let ant = &mut state.ants[ant_idx];
                match update_type {
                    "random_walk" => {
                        ant.1 = val1; // x
                        ant.2 = val2; // y
                        ant.11 = val3; // direction_angle
                        ant.13 = val5 as u32; // exploration_timer
                        
                        // Deposit small amount of pheromone while exploring
                        deposit_pheromone_at_position(state, val1, val2, 0.5);
                    }
                    "return_to_nest" => {
                        ant.1 = val1; // x
                        ant.2 = val2; // y
                        ant.11 = val3; // direction_angle
                        
                        // Deposit stronger pheromone when carrying food
                        deposit_pheromone_at_position(state, val1, val2, 2.0);
                    }
                    "collect_food" => {
                        let food_idx = val4 as usize;
                        if food_idx < state.food_sources.len() {
                            state.food_sources[food_idx].2 -= val1;
                            ant.8 = val1; // carrying_food
                            ant.9 += val1; // total_food
                            ant.7 = 1; // state = CarryingFood
                            ant.10 = food_idx; // current_target_food
                            console::log_1(&format!("Ant {} collected food", ant.0).into());
                        }
                    }
                    "deliver_food" => {
                        state.total_food_collected += val1;
                        ant.8 = 0.0; // carrying_food
                        ant.7 = 0; // state = SearchingForFood
                        ant.10 = usize::MAX; // clear current_target_food
                        // Reset exploration with random angle
                        let mut rng = thread_rng();
                        ant.11 = rng.gen::<f64>() * 2.0 * std::f64::consts::PI; // new random direction
                        ant.13 = 0; // reset exploration timer
                        console::log_1(&format!("Ant {} delivered {:.1} food. Total: {:.1}", ant.0, val1, state.total_food_collected).into());
                    }
                    _ => {}
                }
            }
        }
        
        // Update pheromones periodically
        let mut rng = thread_rng();
        if rng.gen::<f64>() < 0.1 {
            update_pheromones_internal(state);
            evaporate_spatial_pheromones(state);
        }
    }
}

#[wasm_bindgen]
pub fn render_simulation() {
    let state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_ref() {
        if let Ok(renderer) = Renderer::new(&state.canvas_id) {
            renderer.clear();
            
            // Draw spatial pheromone grid first (as background) if enabled
            if state.show_pheromones {
                renderer.draw_spatial_pheromones(&state.pheromone_grid, state.grid_width, state.grid_height, state.grid_cell_size);
            }
            
            // Draw nest
            let nest = Nest { location: Location::new(state.nest_x, state.nest_y) };
            renderer.draw_nest(&nest);
            
            // Draw food sources
            let food_sources: Vec<FoodSource> = state.food_sources.iter().map(|(x, y, amount, max)| {
                FoodSource {
                    location: Location::new(*x, *y),
                    food_amount: *amount,
                    max_food: *max,
                }
            }).collect();
            renderer.draw_food_sources(&food_sources);
            
            // Draw ants
            let ants: Vec<Ant> = state.ants.iter().map(|(id, x, y, target_x, target_y, is_moving, move_progress, ant_state, carrying_food, total_food, current_target_food, _direction_angle, _move_speed, _exploration_timer)| {
                Ant {
                    id: *id,
                    x: *x,
                    y: *y,
                    target_x: *target_x,
                    target_y: *target_y,
                    path: vec![], // Empty path for performance
                    is_moving: *is_moving,
                    move_progress: *move_progress,
                    state: if *ant_state == 0 { AntState::SearchingForFood } else { AntState::CarryingFood },
                    carrying_food: *carrying_food,
                    total_food_collected: *total_food,
                    nest_location: Location::new(state.nest_x, state.nest_y),
                    current_target_food: if *current_target_food == usize::MAX { None } else { Some(*current_target_food) },
                }
            }).collect();
            renderer.draw_ants(&ants, state.show_trails);
        }
    }
}

#[wasm_bindgen]
pub fn get_stats() -> JsValue {
    let state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_ref() {
        let stats = js_sys::Object::new();
        
        js_sys::Reflect::set(&stats, &"iteration".into(), &0.into()).unwrap();
        js_sys::Reflect::set(&stats, &"cities_count".into(), &state.food_sources.len().into()).unwrap();
        js_sys::Reflect::set(&stats, &"ants_count".into(), &state.ants.len().into()).unwrap();
        js_sys::Reflect::set(&stats, &"best_distance".into(), &state.total_food_collected.into()).unwrap();
        
        let state_str = if state.is_running { "running" } else { "idle" };
        js_sys::Reflect::set(&stats, &"state".into(), &state_str.into()).unwrap();
        
        stats.into()
    } else {
        js_sys::Object::new().into()
    }
}

// Parameter setters
#[wasm_bindgen]
pub fn set_animation_speed(speed: f64) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.animation_speed = speed.max(0.1).min(5.0);
    }
}

#[wasm_bindgen]
pub fn set_show_ant_trails(show: bool) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.show_trails = show;
    }
}

#[wasm_bindgen]
pub fn set_show_pheromone_levels(show: bool) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        state.show_pheromones = show;
    }
}

#[wasm_bindgen]
pub fn set_aco_param(param: &str, value: f64) {
    let mut state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_mut() {
        match param {
            "alpha" => state.alpha = value.max(0.1).min(5.0),
            "beta" => state.beta = value.max(0.1).min(5.0),
            "evaporation" => state.evaporation_rate = value.max(0.01).min(0.5),
            "num_ants" => {
                state.num_ants = (value as usize).max(5).min(50);
                if !state.food_sources.is_empty() {
                    initialize_simulation_internal(state);
                }
            },
            _ => {}
        }
    }
}

// Helper function to deposit pheromone at a position
fn deposit_pheromone_at_position(state: &mut SimulationState, x: f64, y: f64, amount: f64) {
    let grid_x = (x / state.grid_cell_size) as usize;
    let grid_y = (y / state.grid_cell_size) as usize;
    
    if grid_x < state.grid_width && grid_y < state.grid_height {
        let idx = grid_y * state.grid_width + grid_x;
        if idx < state.pheromone_grid.len() {
            state.pheromone_grid[idx] = (state.pheromone_grid[idx] + amount).min(1000.0);
            
            // Also deposit in neighboring cells for smoother trails
            let neighbors = [
                (grid_x.wrapping_sub(1), grid_y),
                (grid_x + 1, grid_y),
                (grid_x, grid_y.wrapping_sub(1)),
                (grid_x, grid_y + 1),
            ];
            
            for (nx, ny) in neighbors {
                if nx < state.grid_width && ny < state.grid_height {
                    let nidx = ny * state.grid_width + nx;
                    if nidx < state.pheromone_grid.len() {
                        state.pheromone_grid[nidx] = (state.pheromone_grid[nidx] + amount * 0.5).min(1000.0);
                    }
                }
            }
        }
    }
}

// Helper function to get pheromone level at a position
fn get_pheromone_at_position(state: &SimulationState, x: f64, y: f64) -> f64 {
    let grid_x = (x / state.grid_cell_size) as usize;
    let grid_y = (y / state.grid_cell_size) as usize;
    
    if grid_x < state.grid_width && grid_y < state.grid_height {
        let idx = grid_y * state.grid_width + grid_x;
        if idx < state.pheromone_grid.len() {
            return state.pheromone_grid[idx];
        }
    }
    0.0
}

// Internal helper functions
fn initialize_simulation_internal(state: &mut SimulationState) {
    // Limit food sources 
    const MAX_FOOD_SOURCES: usize = 20;
    if state.food_sources.len() > MAX_FOOD_SOURCES {
        state.food_sources.truncate(MAX_FOOD_SOURCES);
    }
    
    // Initialize pheromone matrix
    let matrix_size = 1 + state.food_sources.len();
    state.pheromone_size = matrix_size;
    state.pheromone_data = vec![1.0; matrix_size * matrix_size];
    
    // Create ants at nest
    state.ants.clear();
    let mut rng = thread_rng();
    for i in 0..state.num_ants {
        let initial_angle = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        let initial_speed = 1.0 + rng.gen::<f64>() * 0.5; // Speed variation 1.0-1.5
        // (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food, direction_angle, move_speed, exploration_timer)
        state.ants.push((i, state.nest_x, state.nest_y, state.nest_x, state.nest_y, false, 0.0, 0, 0.0, 0.0, usize::MAX, initial_angle, initial_speed, 0));
    }
}

fn clear_simulation_internal(state: &mut SimulationState) {
    state.ants.clear();
    state.pheromone_size = 1;
    state.pheromone_data = vec![1.0];
    state.is_running = false;
    state.total_food_collected = 0.0;
    
    // Clear spatial pheromone grid
    state.pheromone_grid.fill(0.0);
}

fn ant_is_at_food_source_internal(state: &SimulationState, ant_idx: usize) -> Option<usize> {
    if ant_idx >= state.ants.len() {
        return None;
    }
    
    let ant = &state.ants[ant_idx];
    for (i, (fx, fy, amount, _)) in state.food_sources.iter().enumerate() {
        if *amount > 0.0 {
            let distance = ((ant.1 - fx).powi(2) + (ant.2 - fy).powi(2)).sqrt();
            if distance < 10.0 {
                return Some(i);
            }
        }
    }
    None
}

fn ant_is_at_nest_internal(state: &SimulationState, ant_idx: usize) -> bool {
    if ant_idx >= state.ants.len() {
        return false;
    }
    
    let ant = &state.ants[ant_idx];
    let distance = ((ant.1 - state.nest_x).powi(2) + (ant.2 - state.nest_y).powi(2)).sqrt();
    distance < 10.0
}

// This function is no longer used since we switched to random walk exploration

fn get_pheromone_internal(state: &SimulationState, i: usize, j: usize) -> f64 {
    if i < state.pheromone_size && j < state.pheromone_size {
        let idx = i * state.pheromone_size + j;
        if idx < state.pheromone_data.len() {
            return state.pheromone_data[idx];
        }
    }
    1.0
}

fn set_pheromone_internal(state: &mut SimulationState, i: usize, j: usize, value: f64) {
    if i < state.pheromone_size && j < state.pheromone_size {
        let idx = i * state.pheromone_size + j;
        if idx < state.pheromone_data.len() {
            state.pheromone_data[idx] = value;
        }
    }
}

// Calculate random walk step for exploring ants
fn calculate_random_walk_step(state: &SimulationState, ant_idx: usize, x: f64, y: f64, current_angle: f64, move_speed: f64, exploration_timer: u32) -> (f64, f64, f64, u32) {
    let mut rng = thread_rng();
    
    // Random direction change every 20-60 steps
    let direction_change_interval = 20 + (exploration_timer % 40);
    let mut new_angle = current_angle;
    let mut new_timer = exploration_timer + 1;
    
    if exploration_timer >= direction_change_interval {
        // Sample pheromone levels in different directions
        let num_samples = 8;
        let mut best_angle = current_angle;
        let mut best_pheromone = 0.0;
        
        for i in 0..num_samples {
            let sample_angle = (i as f64 / num_samples as f64) * 2.0 * std::f64::consts::PI;
            let sample_distance = 20.0;
            let sample_x = x + sample_angle.cos() * sample_distance;
            let sample_y = y + sample_angle.sin() * sample_distance;
            
            let pheromone_level = get_pheromone_at_position(state, sample_x, sample_y);
            
            if pheromone_level > best_pheromone {
                best_pheromone = pheromone_level;
                best_angle = sample_angle;
            }
        }
        
        // Also check for nearby food sources
        let mut best_food_angle = None;
        let mut best_food_distance = f64::MAX;
        
        for (fx, fy, amount, _) in &state.food_sources {
            if *amount > 0.0 {
                let distance = ((x - fx).powi(2) + (y - fy).powi(2)).sqrt();
                if distance < 100.0 && distance < best_food_distance { // Within sensing range
                    best_food_distance = distance;
                    best_food_angle = Some((fy - y).atan2(fx - x));
                }
            }
        }
        
        // Decide direction based on pheromone and food sources
        if let Some(food_angle) = best_food_angle {
            // Strong bias towards visible food
            new_angle = food_angle + (rng.gen::<f64>() - 0.5) * 0.5;
        } else if best_pheromone > 0.1 {
            // Follow pheromone trail with some randomness
            let angle_diff = best_angle - current_angle;
            let normalized_diff = ((angle_diff + std::f64::consts::PI) % (2.0 * std::f64::consts::PI)) - std::f64::consts::PI;
            new_angle = current_angle + normalized_diff * 0.5 + (rng.gen::<f64>() - 0.5) * std::f64::consts::PI * 0.3;
        } else {
            // Random exploration
            new_angle = current_angle + (rng.gen::<f64>() - 0.5) * std::f64::consts::PI;
        }
        
        new_timer = 0;
    }
    
    // Small random deviation each step
    let angle_noise = (rng.gen::<f64>() - 0.5) * 0.2;
    new_angle += angle_noise;
    
    // Calculate movement
    let step_size = move_speed * 2.0;
    let mut new_x = x + new_angle.cos() * step_size;
    let mut new_y = y + new_angle.sin() * step_size;
    
    // Boundary checking and bouncing
    const CANVAS_WIDTH: f64 = 800.0;
    const CANVAS_HEIGHT: f64 = 600.0;
    const MARGIN: f64 = 20.0;
    
    if new_x < MARGIN {
        new_x = MARGIN;
        new_angle = std::f64::consts::PI - new_angle;
    } else if new_x > CANVAS_WIDTH - MARGIN {
        new_x = CANVAS_WIDTH - MARGIN;
        new_angle = std::f64::consts::PI - new_angle;
    }
    
    if new_y < MARGIN {
        new_y = MARGIN;
        new_angle = -new_angle;
    } else if new_y > CANVAS_HEIGHT - MARGIN {
        new_y = CANVAS_HEIGHT - MARGIN;
        new_angle = -new_angle;
    }
    
    (new_x, new_y, new_angle, new_timer)
}

// Calculate return to nest step for ants carrying food
fn calculate_return_to_nest_step(state: &SimulationState, _ant_idx: usize, x: f64, y: f64, move_speed: f64) -> (f64, f64, f64) {
    let mut rng = thread_rng();
    
    // Direction towards nest
    let nest_angle = (state.nest_y - y).atan2(state.nest_x - x);
    
    // Add small random deviation to make path more natural
    let angle_noise = (rng.gen::<f64>() - 0.5) * 0.3;
    let actual_angle = nest_angle + angle_noise;
    
    // Move towards nest
    let step_size = move_speed * 1.5; // Slightly faster when returning
    let new_x = x + actual_angle.cos() * step_size;
    let new_y = y + actual_angle.sin() * step_size;
    
    (new_x, new_y, actual_angle)
}

// Evaporate spatial pheromones
fn evaporate_spatial_pheromones(state: &mut SimulationState) {
    for i in 0..state.pheromone_grid.len() {
        state.pheromone_grid[i] *= 1.0 - state.evaporation_rate * 0.5; // Slower evaporation for spatial pheromones
        if state.pheromone_grid[i] < 0.01 {
            state.pheromone_grid[i] = 0.0;
        }
    }
}

fn update_pheromones_internal(state: &mut SimulationState) {
    // Evaporate pheromones
    for i in 0..state.pheromone_size {
        for j in 0..state.pheromone_size {
            if i != j {
                let current = get_pheromone_internal(state, i, j);
                let new_value = (current * (1.0 - state.evaporation_rate)).max(0.01);
                set_pheromone_internal(state, i, j, new_value);
            }
        }
    }
    
    // Collect pheromone deposits first, then apply them
    let mut deposits = Vec::new();
    for ant in &state.ants {
        if ant.7 == 1 && ant.8 > 0.0 { // CarryingFood and has food
            if ant.10 != usize::MAX && ant.10 < state.food_sources.len() {
                let pheromone_amount = state.q * ant.8;
                let matrix_idx = ant.10 + 1;
                if matrix_idx < state.pheromone_size {
                    deposits.push((0, matrix_idx, pheromone_amount));
                }
            }
        }
    }
    
    // Apply deposits
    for (i, j, amount) in deposits {
        let current = get_pheromone_internal(state, i, j);
        let new_value = (current + amount).min(1000.0);
        set_pheromone_internal(state, i, j, new_value);
        set_pheromone_internal(state, j, i, new_value);
    }
}