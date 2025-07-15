use wasm_bindgen_test::*;
use aco_wasm::geometry::city::City;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_city_creation() {
    let city = City::new(0, 100.0, 200.0);
    assert_eq!(city.id(), 0);
    assert_eq!(city.x(), 100.0);
    assert_eq!(city.y(), 200.0);
}

#[wasm_bindgen_test]
fn test_city_distance() {
    let city1 = City::new(0, 0.0, 0.0);
    let city2 = City::new(1, 3.0, 4.0);
    let distance = city1.distance_to(&city2);
    assert_eq!(distance, 5.0);
}

#[wasm_bindgen_test]
fn test_city_equality() {
    let city1 = City::new(0, 100.0, 200.0);
    let city2 = City::new(0, 100.0, 200.0);
    let city3 = City::new(1, 100.0, 200.0);
    
    assert_eq!(city1, city2);
    assert_ne!(city1, city3);
}