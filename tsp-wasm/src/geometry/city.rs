use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct City {
    id: u32,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl City {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, x: f64, y: f64) -> City {
        City { id, x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen]
    pub fn distance_to(&self, other: &City) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_creation() {
        let city = City::new(0, 100.0, 200.0);
        assert_eq!(city.id(), 0);
        assert_eq!(city.x(), 100.0);
        assert_eq!(city.y(), 200.0);
    }

    #[test]
    fn test_city_distance() {
        let city1 = City::new(0, 0.0, 0.0);
        let city2 = City::new(1, 3.0, 4.0);
        let distance = city1.distance_to(&city2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_city_equality() {
        let city1 = City::new(0, 100.0, 200.0);
        let city2 = City::new(0, 100.0, 200.0);
        let city3 = City::new(1, 100.0, 200.0);
        
        assert_eq!(city1, city2);
        assert_ne!(city1, city3);
    }
}