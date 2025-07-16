use crate::aco::{ant::Ant, pheromone::PheromoneMatrix};
use crate::geometry::city::City;

#[derive(Debug, Clone)]
pub struct ACOParameters {
    pub num_ants: usize,
    pub max_generations: usize,
    pub evaporation_rate: f64,
    pub alpha: f64,
    pub beta: f64,
    pub initial_pheromone: f64,
}

impl Default for ACOParameters {
    fn default() -> Self {
        ACOParameters {
            num_ants: 50,
            max_generations: 100,
            evaporation_rate: 0.1,
            alpha: 1.0,
            beta: 2.0,
            initial_pheromone: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct Colony {
    cities: Vec<City>,
    pheromone_matrix: PheromoneMatrix,
    parameters: ACOParameters,
    best_route: Option<Vec<usize>>,
    best_distance: f64,
    generation: usize,
}

impl Colony {
    pub fn new(cities: Vec<City>, parameters: ACOParameters) -> Self {
        let num_cities = cities.len();
        let pheromone_matrix = PheromoneMatrix::new(num_cities, parameters.initial_pheromone);
        
        Colony {
            cities,
            pheromone_matrix,
            parameters,
            best_route: None,
            best_distance: f64::INFINITY,
            generation: 0,
        }
    }

    pub fn run_iteration(&mut self) -> bool {
        if self.generation >= self.parameters.max_generations {
            return false;
        }

        let mut ants = Vec::new();
        
        for i in 0..self.parameters.num_ants {
            let start_city = i % self.cities.len();
            let mut ant = Ant::new(start_city, self.cities.len());
            
            while !ant.is_tour_complete() {
                if let Some(next_city) = ant.select_next_city(
                    &self.cities,
                    self.pheromone_matrix.get_matrix(),
                    self.parameters.alpha,
                    self.parameters.beta,
                ) {
                    ant.move_to_city(next_city, &self.cities);
                } else {
                    ant.complete_tour(&self.cities);
                }
            }
            
            if !ant.is_tour_complete() {
                ant.complete_tour(&self.cities);
            }
            
            ants.push(ant);
        }

        self.pheromone_matrix.evaporate(self.parameters.evaporation_rate);

        for ant in &ants {
            if ant.total_distance() < self.best_distance {
                self.best_distance = ant.total_distance();
                self.best_route = Some(ant.route().clone());
            }
            
            self.pheromone_matrix.deposit(ant.route(), ant.total_distance());
        }

        self.generation += 1;
        true
    }

    pub fn best_route(&self) -> Option<&Vec<usize>> {
        self.best_route.as_ref()
    }

    pub fn best_distance(&self) -> f64 {
        self.best_distance
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn cities(&self) -> &Vec<City> {
        &self.cities
    }

    pub fn is_complete(&self) -> bool {
        self.generation >= self.parameters.max_generations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aco_parameters_default() {
        let params = ACOParameters::default();
        assert_eq!(params.num_ants, 50);
        assert_eq!(params.max_generations, 100);
        assert_eq!(params.evaporation_rate, 0.1);
        assert_eq!(params.alpha, 1.0);
        assert_eq!(params.beta, 2.0);
        assert_eq!(params.initial_pheromone, 1.0);
    }

    #[test]
    fn test_colony_creation() {
        let cities = vec![
            City::new(0, 0.0, 0.0),
            City::new(1, 1.0, 1.0),
            City::new(2, 2.0, 2.0),
        ];
        let params = ACOParameters::default();
        let colony = Colony::new(cities, params);
        
        assert_eq!(colony.cities().len(), 3);
        assert_eq!(colony.generation(), 0);
        assert_eq!(colony.best_distance(), f64::INFINITY);
        assert!(colony.best_route().is_none());
    }

    #[test]
    fn test_colony_run_iteration() {
        let cities = vec![
            City::new(0, 0.0, 0.0),
            City::new(1, 3.0, 4.0),
            City::new(2, 6.0, 8.0),
        ];
        let mut params = ACOParameters::default();
        params.num_ants = 5;
        params.max_generations = 2;
        
        let mut colony = Colony::new(cities, params);
        
        assert!(colony.run_iteration());
        assert_eq!(colony.generation(), 1);
        assert!(colony.best_distance() < f64::INFINITY);
        assert!(colony.best_route().is_some());
        
        assert!(colony.run_iteration());
        assert_eq!(colony.generation(), 2);
        
        assert!(!colony.run_iteration());
        assert!(colony.is_complete());
    }
}