use super::city::City;

pub fn calculate_distance(city1: &City, city2: &City) -> f64 {
    city1.distance_to(city2)
}

pub fn calculate_total_distance(cities: &[City]) -> f64 {
    if cities.len() < 2 {
        return 0.0;
    }
    
    let mut total = 0.0;
    for i in 0..cities.len() - 1 {
        total += calculate_distance(&cities[i], &cities[i + 1]);
    }
    // Return to starting city
    total += calculate_distance(&cities[cities.len() - 1], &cities[0]);
    total
}