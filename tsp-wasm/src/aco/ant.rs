use crate::geometry::city::City;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Ant {
    current_city: usize,
    visited_cities: Vec<bool>,
    route: Vec<usize>,
    total_distance: f64,
}

impl Ant {
    pub fn new(start_city: usize, num_cities: usize) -> Self {
        let mut visited = vec![false; num_cities];
        visited[start_city] = true;
        
        Ant {
            current_city: start_city,
            visited_cities: visited,
            route: vec![start_city],
            total_distance: 0.0,
        }
    }

    pub fn select_next_city(&mut self, cities: &[City], pheromone_matrix: &[Vec<f64>], alpha: f64, beta: f64) -> Option<usize> {
        let unvisited: Vec<usize> = (0..cities.len())
            .filter(|&i| !self.visited_cities[i])
            .collect();

        if unvisited.is_empty() {
            return None;
        }

        let mut probabilities = Vec::new();
        let mut total_prob = 0.0;

        for &city in &unvisited {
            let distance = cities[self.current_city].distance_to(&cities[city]);
            let pheromone = pheromone_matrix[self.current_city][city];
            let probability = pheromone.powf(alpha) * (1.0 / distance).powf(beta);
            probabilities.push(probability);
            total_prob += probability;
        }

        if total_prob == 0.0 {
            return Some(unvisited[0]);
        }

        let mut rng = rand::thread_rng();
        let random_value = rng.gen::<f64>() * total_prob;
        let mut cumulative_prob = 0.0;

        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative_prob += prob;
            if random_value <= cumulative_prob {
                return Some(unvisited[i]);
            }
        }

        Some(unvisited[0])
    }

    pub fn move_to_city(&mut self, city: usize, cities: &[City]) {
        if !self.visited_cities[city] {
            let distance = cities[self.current_city].distance_to(&cities[city]);
            self.total_distance += distance;
            self.visited_cities[city] = true;
            self.route.push(city);
            self.current_city = city;
        }
    }

    pub fn complete_tour(&mut self, cities: &[City]) {
        if !self.route.is_empty() {
            let start_city = self.route[0];
            let distance = cities[self.current_city].distance_to(&cities[start_city]);
            self.total_distance += distance;
            self.route.push(start_city);
        }
    }

    pub fn route(&self) -> &Vec<usize> {
        &self.route
    }

    pub fn total_distance(&self) -> f64 {
        self.total_distance
    }

    pub fn is_tour_complete(&self) -> bool {
        self.route.len() > 1 && self.route.first() == self.route.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ant_creation() {
        let ant = Ant::new(0, 5);
        assert_eq!(ant.current_city, 0);
        assert_eq!(ant.route.len(), 1);
        assert_eq!(ant.route[0], 0);
        assert_eq!(ant.total_distance, 0.0);
        assert!(ant.visited_cities[0]);
        assert!(!ant.visited_cities[1]);
    }

    #[test]
    fn test_ant_move_to_city() {
        let mut ant = Ant::new(0, 3);
        let cities = vec![
            City::new(0, 0.0, 0.0),
            City::new(1, 3.0, 4.0),
            City::new(2, 6.0, 8.0),
        ];
        
        ant.move_to_city(1, &cities);
        assert_eq!(ant.current_city, 1);
        assert_eq!(ant.route.len(), 2);
        assert_eq!(ant.total_distance, 5.0);
        assert!(ant.visited_cities[1]);
    }

    #[test]
    fn test_ant_complete_tour() {
        let mut ant = Ant::new(0, 3);
        let cities = vec![
            City::new(0, 0.0, 0.0),
            City::new(1, 3.0, 4.0),
            City::new(2, 6.0, 8.0),
        ];
        
        ant.move_to_city(1, &cities);
        ant.move_to_city(2, &cities);
        ant.complete_tour(&cities);
        
        assert!(ant.is_tour_complete());
        assert_eq!(ant.route.len(), 4);
        assert_eq!(ant.route[0], ant.route[3]);
    }
}