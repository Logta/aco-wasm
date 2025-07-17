#[derive(Clone, Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self {
        Location { x, y }
    }
    
    pub fn distance_to(&self, other: &Location) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct Nest {
    pub location: Location,
}

impl Nest {
    pub fn new(x: f64, y: f64) -> Self {
        Nest {
            location: Location::new(x, y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FoodSource {
    pub location: Location,
    pub food_amount: f64,
    pub max_food: f64,
}

impl FoodSource {
    pub fn new(x: f64, y: f64, food_amount: f64) -> Self {
        FoodSource {
            location: Location::new(x, y),
            food_amount,
            max_food: food_amount,
        }
    }
    
    pub fn take_food(&mut self, amount: f64) -> f64 {
        let taken = amount.min(self.food_amount);
        self.food_amount -= taken;
        taken
    }
    
    pub fn is_depleted(&self) -> bool {
        self.food_amount <= 0.0
    }
    
    pub fn food_ratio(&self) -> f64 {
        if self.max_food > 0.0 {
            self.food_amount / self.max_food
        } else {
            0.0
        }
    }
}