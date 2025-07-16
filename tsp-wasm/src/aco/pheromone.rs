
#[derive(Debug, Clone)]
pub struct PheromoneMatrix {
    matrix: Vec<Vec<f64>>,
    size: usize,
}

impl PheromoneMatrix {
    pub fn new(size: usize, initial_pheromone: f64) -> Self {
        let matrix = vec![vec![initial_pheromone; size]; size];
        PheromoneMatrix { matrix, size }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        if i < self.size && j < self.size {
            self.matrix[i][j]
        } else {
            0.0
        }
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        if i < self.size && j < self.size {
            self.matrix[i][j] = value;
            self.matrix[j][i] = value;
        }
    }

    pub fn evaporate(&mut self, evaporation_rate: f64) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.matrix[i][j] *= 1.0 - evaporation_rate;
            }
        }
    }

    pub fn deposit(&mut self, route: &[usize], total_distance: f64) {
        if route.len() < 2 || total_distance <= 0.0 {
            return;
        }

        let pheromone_deposit = 1.0 / total_distance;
        
        for i in 0..route.len() - 1 {
            let city1 = route[i];
            let city2 = route[i + 1];
            self.matrix[city1][city2] += pheromone_deposit;
            self.matrix[city2][city1] += pheromone_deposit;
        }
    }

    pub fn get_matrix(&self) -> &Vec<Vec<f64>> {
        &self.matrix
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pheromone_matrix_creation() {
        let matrix = PheromoneMatrix::new(3, 1.0);
        assert_eq!(matrix.size(), 3);
        assert_eq!(matrix.get(0, 1), 1.0);
        assert_eq!(matrix.get(1, 2), 1.0);
    }

    #[test]
    fn test_pheromone_matrix_set_get() {
        let mut matrix = PheromoneMatrix::new(3, 1.0);
        matrix.set(0, 1, 2.0);
        assert_eq!(matrix.get(0, 1), 2.0);
        assert_eq!(matrix.get(1, 0), 2.0);
    }

    #[test]
    fn test_pheromone_evaporation() {
        let mut matrix = PheromoneMatrix::new(3, 1.0);
        matrix.evaporate(0.1);
        assert_eq!(matrix.get(0, 1), 0.9);
        assert_eq!(matrix.get(1, 2), 0.9);
    }

    #[test]
    fn test_pheromone_deposit() {
        let mut matrix = PheromoneMatrix::new(3, 1.0);
        let route = vec![0, 1, 2, 0];
        matrix.deposit(&route, 10.0);
        
        assert_eq!(matrix.get(0, 1), 1.1);
        assert_eq!(matrix.get(1, 2), 1.1);
        assert_eq!(matrix.get(2, 0), 1.1);
    }
}