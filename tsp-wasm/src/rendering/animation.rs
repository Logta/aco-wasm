use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct AntAnimation {
    id: u32,
    current_x: f64,
    current_y: f64,
    target_x: f64,
    target_y: f64,
    progress: f64,
    speed: f64,
}

#[wasm_bindgen]
impl AntAnimation {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, start_x: f64, start_y: f64, speed: f64) -> AntAnimation {
        AntAnimation {
            id,
            current_x: start_x,
            current_y: start_y,
            target_x: start_x,
            target_y: start_y,
            progress: 1.0,
            speed,
        }
    }

    #[wasm_bindgen]
    pub fn set_target(&mut self, target_x: f64, target_y: f64) {
        if self.progress >= 1.0 {
            self.target_x = target_x;
            self.target_y = target_y;
            self.progress = 0.0;
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f64) -> bool {
        if self.progress >= 1.0 {
            return false;
        }

        self.progress += self.speed * delta_time;
        if self.progress > 1.0 {
            self.progress = 1.0;
        }

        // Smooth interpolation
        let t = self.ease_in_out_cubic(self.progress);
        self.current_x = self.lerp(self.current_x, self.target_x, t);
        self.current_y = self.lerp(self.current_y, self.target_y, t);

        true
    }

    #[wasm_bindgen]
    pub fn get_position(&self) -> Vec<f64> {
        vec![self.current_x, self.current_y]
    }

    #[wasm_bindgen]
    pub fn get_angle(&self) -> f64 {
        let dx = self.target_x - self.current_x;
        let dy = self.target_y - self.current_y;
        dy.atan2(dx)
    }

    #[wasm_bindgen]
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    fn lerp(&self, start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t
    }

    fn ease_in_out_cubic(&self, t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

#[wasm_bindgen]
pub struct AnimationManager {
    ant_animations: HashMap<u32, AntAnimation>,
    animation_speed: f64,
    last_timestamp: f64,
}

#[wasm_bindgen]
impl AnimationManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnimationManager {
        AnimationManager {
            ant_animations: HashMap::new(),
            animation_speed: 1.0,
            last_timestamp: 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn add_ant(&mut self, id: u32, x: f64, y: f64) {
        let animation = AntAnimation::new(id, x, y, self.animation_speed);
        self.ant_animations.insert(id, animation);
    }

    #[wasm_bindgen]
    pub fn move_ant(&mut self, id: u32, target_x: f64, target_y: f64) {
        if let Some(animation) = self.ant_animations.get_mut(&id) {
            animation.set_target(target_x, target_y);
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, timestamp: f64) -> bool {
        let delta_time = if self.last_timestamp > 0.0 {
            (timestamp - self.last_timestamp) / 1000.0 // Convert to seconds
        } else {
            0.0
        };
        self.last_timestamp = timestamp;

        let mut any_active = false;
        for animation in self.ant_animations.values_mut() {
            if animation.update(delta_time) {
                any_active = true;
            }
        }

        any_active
    }

    #[wasm_bindgen]
    pub fn get_ant_position(&self, id: u32) -> Vec<f64> {
        // Check if the ID is valid and exists in the map
        if let Some(animation) = self.ant_animations.get(&id) {
            animation.get_position()
        } else {
            // Return empty vector if ID doesn't exist
            vec![]
        }
    }

    #[wasm_bindgen]
    pub fn get_ant_angle(&self, id: u32) -> f64 {
        if let Some(animation) = self.ant_animations.get(&id) {
            animation.get_angle()
        } else {
            0.0
        }
    }

    #[wasm_bindgen]
    pub fn set_animation_speed(&mut self, speed: f64) {
        self.animation_speed = speed.max(0.1).min(10.0);
        for animation in self.ant_animations.values_mut() {
            animation.speed = self.animation_speed;
        }
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.ant_animations.clear();
        self.last_timestamp = 0.0;
    }

    #[wasm_bindgen]
    pub fn get_active_ant_count(&self) -> u32 {
        self.ant_animations.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ant_animation_creation() {
        let animation = AntAnimation::new(0, 10.0, 20.0, 1.0);
        assert_eq!(animation.current_x, 10.0);
        assert_eq!(animation.current_y, 20.0);
        assert!(animation.is_complete());
    }

    #[test]
    fn test_ant_animation_movement() {
        let mut animation = AntAnimation::new(0, 0.0, 0.0, 2.0);
        animation.set_target(10.0, 10.0);
        assert!(!animation.is_complete());
        
        animation.update(0.5); // 0.5 seconds
        assert!(!animation.is_complete());
        
        animation.update(0.5); // Another 0.5 seconds (total 1.0s at speed 2.0)
        assert!(animation.is_complete());
    }

    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new();
        manager.add_ant(0, 0.0, 0.0);
        manager.move_ant(0, 10.0, 10.0);
        
        assert_eq!(manager.get_active_ant_count(), 1);
        
        let position = manager.get_ant_position(0);
        assert_eq!(position.len(), 2);
    }
}