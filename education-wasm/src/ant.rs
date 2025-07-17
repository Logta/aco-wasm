use crate::city::{Location, Nest, FoodSource};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum AntState {
    SearchingForFood,
    CarryingFood,
}

#[derive(Clone, Debug)]
pub struct Ant {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub path: Vec<(f64, f64)>,
    pub is_moving: bool,
    pub move_progress: f64,
    pub state: AntState,
    pub carrying_food: f64,
    pub total_food_collected: f64,
    pub nest_location: Location,
    pub current_target_food: Option<usize>,
}

impl Ant {
    pub fn new(id: usize, nest: &Nest) -> Self {
        Ant {
            id,
            x: nest.location.x,
            y: nest.location.y,
            target_x: nest.location.x,
            target_y: nest.location.y,
            path: vec![(nest.location.x, nest.location.y)],
            is_moving: false,
            move_progress: 0.0,
            state: AntState::SearchingForFood,
            carrying_food: 0.0,
            total_food_collected: 0.0,
            nest_location: nest.location.clone(),
            current_target_food: None,
        }
    }
    
    pub fn select_food_source(&self, food_sources: &[FoodSource], pheromones: &[Vec<f64>], alpha: f64, beta: f64, rng: &mut impl Rng) -> Option<usize> {
        if self.state != AntState::SearchingForFood {
            return None;
        }
        
        let mut probabilities = Vec::with_capacity(food_sources.len());
        let mut total_prob = 0.0;
        
        for (i, food_source) in food_sources.iter().enumerate() {
            if !food_source.is_depleted() {
                let distance = Location::new(self.x, self.y).distance_to(&food_source.location);
                
                // Use food amount as attractiveness factor
                let food_attractiveness = food_source.food_ratio();
                
                // Get pheromone level (from nest to food source) with bounds check
                let pheromone = if pheromones.len() > 0 && i + 1 < pheromones[0].len() {
                    pheromones[0][i + 1] // Index 0 represents nest, i+1 represents food source i
                } else {
                    1.0
                };
                
                // ACO probability calculation: τ^α * η^β * food_factor
                let distance_attractiveness = if distance > 0.0 { 1.0 / distance } else { 1.0 };
                let prob = pheromone.powf(alpha.max(0.0).min(10.0)) 
                         * distance_attractiveness.powf(beta.max(0.0).min(10.0)) 
                         * (1.0 + food_attractiveness * 2.0); // Boost probability for more food
                
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
        
        // Fallback to first available food source
        probabilities.first().map(|(idx, _)| *idx)
    }
    
    pub fn start_move_to_food(&mut self, food_idx: usize, food_sources: &[FoodSource]) {
        if food_idx >= food_sources.len() || food_sources[food_idx].is_depleted() {
            return;
        }
        
        let food_source = &food_sources[food_idx];
        self.target_x = food_source.location.x;
        self.target_y = food_source.location.y;
        self.is_moving = true;
        self.move_progress = 0.0;
        self.current_target_food = Some(food_idx);
        
        // Clear excessive path history when starting new movement
        if self.path.len() > 10 {
            let keep_last = 5;
            let new_start = self.path.len() - keep_last;
            self.path = self.path[new_start..].to_vec();
        }
    }
    
    pub fn start_return_to_nest(&mut self) {
        self.target_x = self.nest_location.x;
        self.target_y = self.nest_location.y;
        self.is_moving = true;
        self.move_progress = 0.0;
        self.state = AntState::CarryingFood;
        self.current_target_food = None;
    }
    
    pub fn update_movement(&mut self, speed: f64) {
        if !self.is_moving {
            return;
        }
        
        self.move_progress += speed;
        
        if self.move_progress >= 1.0 {
            // Reached target
            self.move_progress = 1.0;
            let start_x = self.x;
            let start_y = self.y;
            self.x = self.target_x;
            self.y = self.target_y;
            
            // Limit path length to prevent memory growth
            const MAX_PATH_LENGTH: usize = 50;
            if self.path.len() >= MAX_PATH_LENGTH {
                self.path.remove(0); // Remove oldest point
            }
            self.path.push((self.x, self.y));
            self.is_moving = false;
        } else {
            // Interpolate position
            let start_x = if self.path.is_empty() { self.x } else { self.path.last().unwrap().0 };
            let start_y = if self.path.is_empty() { self.y } else { self.path.last().unwrap().1 };
            
            self.x = start_x + (self.target_x - start_x) * self.move_progress;
            self.y = start_y + (self.target_y - start_y) * self.move_progress;
        }
    }
    
    pub fn collect_food(&mut self, food_sources: &mut [FoodSource]) -> bool {
        if let Some(food_idx) = self.current_target_food {
            if food_idx < food_sources.len() {
                let food_taken = food_sources[food_idx].take_food(1.0); // Take 1 unit of food
                if food_taken > 0.0 {
                    self.carrying_food = food_taken;
                    self.total_food_collected += food_taken;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn deliver_food(&mut self) -> f64 {
        let delivered = self.carrying_food;
        self.carrying_food = 0.0;
        self.state = AntState::SearchingForFood;
        delivered
    }
    
    pub fn is_at_nest(&self) -> bool {
        let distance = Location::new(self.x, self.y).distance_to(&self.nest_location);
        distance < 10.0 // Within 10 pixels of nest
    }
    
    pub fn is_at_food_source(&self, food_sources: &[FoodSource]) -> bool {
        if let Some(food_idx) = self.current_target_food {
            if food_idx < food_sources.len() {
                let distance = Location::new(self.x, self.y).distance_to(&food_sources[food_idx].location);
                return distance < 10.0; // Within 10 pixels of food source
            }
        }
        false
    }
    
    pub fn reset(&mut self) {
        self.x = self.nest_location.x;
        self.y = self.nest_location.y;
        self.target_x = self.nest_location.x;
        self.target_y = self.nest_location.y;
        self.path = vec![(self.nest_location.x, self.nest_location.y)];
        self.is_moving = false;
        self.move_progress = 0.0;
        self.state = AntState::SearchingForFood;
        self.carrying_food = 0.0;
        self.current_target_food = None;
    }
}