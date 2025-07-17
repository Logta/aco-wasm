#[derive(Clone, Debug)]
pub struct PheromoneMatrix {
    matrix: Vec<Vec<f64>>,
    size: usize,
    initial_pheromone: f64,
}

impl PheromoneMatrix {
    pub fn new(size: usize, initial_pheromone: f64) -> Self {
        let matrix = vec![vec![initial_pheromone; size]; size];
        PheromoneMatrix {
            matrix,
            size,
            initial_pheromone,
        }
    }
    
    pub fn get(&self, i: usize, j: usize) -> f64 {
        if i < self.size && j < self.size && i < self.matrix.len() && j < self.matrix[i].len() {
            self.matrix[i][j]
        } else {
            self.initial_pheromone
        }
    }
    
    pub fn deposit(&mut self, i: usize, j: usize, amount: f64) {
        if i < self.size && j < self.size && i < self.matrix.len() && j < self.matrix[i].len() 
           && j < self.matrix.len() && i < self.matrix[j].len() && amount.is_finite() && amount >= 0.0 {
            self.matrix[i][j] += amount;
            self.matrix[j][i] = self.matrix[i][j]; // Symmetric
            
            // Prevent excessive pheromone accumulation
            const MAX_PHEROMONE: f64 = 1000.0;
            if self.matrix[i][j] > MAX_PHEROMONE {
                self.matrix[i][j] = MAX_PHEROMONE;
                self.matrix[j][i] = MAX_PHEROMONE;
            }
        }
    }
    
    pub fn evaporate(&mut self, evaporation_rate: f64) {
        let safe_rate = evaporation_rate.max(0.0).min(1.0);
        let actual_size = self.matrix.len().min(self.size);
        
        for i in 0..actual_size {
            if i < self.matrix.len() {
                let row_size = self.matrix[i].len().min(self.size);
                for j in 0..row_size {
                    if i != j {
                        self.matrix[i][j] *= 1.0 - safe_rate;
                        if self.matrix[i][j] < 0.01 {
                            self.matrix[i][j] = 0.01; // Minimum pheromone level
                        }
                    }
                }
            }
        }
    }
    
    pub fn deposit_tour(&mut self, tour: &[usize], tour_length: f64, q: f64) {
        if tour_length > 0.0 {
            let delta = q / tour_length;
            
            for window in tour.windows(2) {
                if let [i, j] = window {
                    self.deposit(*i, *j, delta);
                }
            }
            
            // Close the tour (return to start)
            if let (Some(&first), Some(&last)) = (tour.first(), tour.last()) {
                self.deposit(last, first, delta);
            }
        }
    }
    
    pub fn get_matrix(&self) -> &Vec<Vec<f64>> {
        &self.matrix
    }
    
    pub fn reset(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.matrix[i][j] = self.initial_pheromone;
            }
        }
    }
    
    pub fn get_max_pheromone(&self) -> f64 {
        let mut max_val = 0.0;
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self.matrix[i][j] > max_val {
                    max_val = self.matrix[i][j];
                }
            }
        }
        max_val
    }
}