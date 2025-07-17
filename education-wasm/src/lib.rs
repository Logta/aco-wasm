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

impl SimulationState {
    fn new(canvas_id: &str) -> Self {
        SimulationState {
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
            let (_id, x, y, target_x, target_y, is_moving, move_progress, ant_state, carrying_food, _total_food, _current_target_food) = *ant;
            
            if is_moving {
                // Continue moving towards target
                let new_progress = move_progress + state.animation_speed * 0.02;
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
                match ant_state {
                    0 => { // SearchingForFood
                        if let Some(food_idx) = ant_is_at_food_source_internal(state, i) {
                            // Try to collect food
                            if state.food_sources[food_idx].2 > 0.0 {
                                let food_taken = 1.0_f64.min(state.food_sources[food_idx].2);
                                ant_updates.push((i, "collect_food", food_taken, 0.0, 0.0, food_idx as f64, 1));
                            }
                        } else {
                            // Look for a food source to visit
                            if let Some(food_idx) = select_food_source_for_ant_internal(state, i) {
                                let target_x = state.food_sources[food_idx].0;
                                let target_y = state.food_sources[food_idx].1;
                                ant_updates.push((i, "move_to_food", target_x, target_y, 0.0, food_idx as f64, 0));
                            }
                        }
                    }
                    1 => { // CarryingFood
                        if ant_is_at_nest_internal(state, i) {
                            // Deliver food
                            ant_updates.push((i, "deliver_food", carrying_food, 0.0, 0.0, 0.0, 0));
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Second pass: apply updates
        for (ant_idx, update_type, val1, val2, val3, val4, _val5) in ant_updates {
            if ant_idx < state.ants.len() {
                let ant = &mut state.ants[ant_idx];
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
                        if food_idx < state.food_sources.len() {
                            state.food_sources[food_idx].2 -= val1;
                            ant.8 = val1; // carrying_food
                            ant.9 += val1; // total_food
                            ant.3 = state.nest_x; // target_x
                            ant.4 = state.nest_y; // target_y
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
                        state.total_food_collected += val1;
                        ant.8 = 0.0; // carrying_food
                        ant.7 = 0; // state = SearchingForFood
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
        }
    }
}

#[wasm_bindgen]
pub fn render_simulation() {
    let state_guard = GLOBAL_STATE.lock().unwrap();
    if let Some(state) = state_guard.as_ref() {
        if let Ok(renderer) = Renderer::new(&state.canvas_id) {
            renderer.clear();
            
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
            let ants: Vec<Ant> = state.ants.iter().map(|(id, x, y, target_x, target_y, is_moving, move_progress, ant_state, carrying_food, total_food, current_target_food)| {
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
    for i in 0..state.num_ants {
        // (id, x, y, target_x, target_y, is_moving, move_progress, state, carrying_food, total_food, current_target_food)
        state.ants.push((i, state.nest_x, state.nest_y, state.nest_x, state.nest_y, false, 0.0, 0, 0.0, 0.0, usize::MAX));
    }
}

fn clear_simulation_internal(state: &mut SimulationState) {
    state.ants.clear();
    state.pheromone_size = 1;
    state.pheromone_data = vec![1.0];
    state.is_running = false;
    state.total_food_collected = 0.0;
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

fn select_food_source_for_ant_internal(state: &SimulationState, ant_idx: usize) -> Option<usize> {
    if ant_idx >= state.ants.len() || state.ants[ant_idx].7 != 0 {
        return None;
    }
    
    let ant = &state.ants[ant_idx];
    let mut rng = thread_rng();
    let mut probabilities = Vec::new();
    let mut total_prob = 0.0;
    
    for (i, (fx, fy, amount, max_amount)) in state.food_sources.iter().enumerate() {
        if *amount > 0.0 {
            let distance = ((ant.1 - fx).powi(2) + (ant.2 - fy).powi(2)).sqrt();
            let food_ratio = if *max_amount > 0.0 { amount / max_amount } else { 0.0 };
            
            // Get pheromone level
            let pheromone = get_pheromone_internal(state, 0, i + 1);
            
            let distance_attractiveness = if distance > 0.0 { 1.0 / distance } else { 1.0 };
            let prob = pheromone.powf(state.alpha) * distance_attractiveness.powf(state.beta) * (1.0 + food_ratio * 2.0);
            
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