use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct Game {
    pub bird_y: f64,
    velocity: f64,
    gravity: f64,
    obstacles: Vec<Obstacle>,
    frames_since_last_obstacle: u32,
    state: GameState,
    canvas_height: f64, 
    score: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            bird_y: 50.0,
            velocity: 0.0,
            gravity: 0.1,
            obstacles: Vec::new(),
            frames_since_last_obstacle: 0, // Initialize the counter
            state: GameState::Playing,
            canvas_height:600.0,
            score:0,
        }
    }
    // Add a method to restart the game
    pub fn restart(&mut self) {
        self.bird_y = 50.0;
        self.velocity = 0.0;
        self.obstacles.clear();
        self.frames_since_last_obstacle = 0;
        self.state = GameState::Playing;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
    
    // Getter for the game state
    pub fn get_state(&self) -> String {
        match self.state {
            GameState::Playing => "Playing".to_string(),
            GameState::GameOver => "GameOver".to_string(),
        }
    }

    pub fn add_obstacle(&mut self, x: f64, gap_y: f64, gap_size: f64) {
        let obstacle = Obstacle::new(x, gap_y, gap_size);
        self.obstacles.push(obstacle);
    }

    pub fn get_obstacle_count(&self) -> usize {
        self.obstacles.len()
    }

    // Rust: Confirming to_value usage
    pub fn get_obstacle_data(&self, index: usize) -> JsValue {
        if let Some(obstacle) = self.obstacles.get(index) {
            return serde_wasm_bindgen::to_value(obstacle).unwrap_or(JsValue::UNDEFINED);
        }
        JsValue::UNDEFINED // Use UNDEFINED for better clarity in error handling
    }

    pub fn update(&mut self) -> bool {
        if self.state == GameState::GameOver {
            return false;
        }
        
        self.velocity += self.gravity;
        self.bird_y += self.velocity;
        
        // Update obstacles and increment score if an obstacle is passed
        let bird_x = 100.0; // Assuming bird's constant X position
        self.obstacles.iter_mut().for_each(|obstacle| {
            obstacle.update();
            if obstacle.x + 50.0 < bird_x && !obstacle.passed {
                obstacle.passed = true;
                self.score += 1;
            }
        });

        // Remove obstacles that are off-screen
        self.obstacles.retain(|obstacle| !obstacle.is_off_screen());

        // Increment the counter
        self.frames_since_last_obstacle += 1;

        // Add a new obstacle every 120 frames (adjust based on your game's speed and difficulty)
        if self.frames_since_last_obstacle >= 120 {
            self.add_obstacle(800.0, 200.0, 150.0); // Example values; adjust as needed
            self.frames_since_last_obstacle = 0;
        }
        // Check for collisions with obstacles
        for obstacle in &self.obstacles {
            if self.check_collision(obstacle) {
                self.state = GameState::GameOver;
                return true; // Collision detected
            }
        }
        // Check for collision with the ground
        if self.bird_y >= 600.0 - 20.0 { // Assuming canvas height is 600 and bird height is 20
            self.state = GameState::GameOver;
            return true; // Collision with the ground
        }

        false // No collision
    }

    fn check_collision(&self, obstacle: &Obstacle) -> bool {
        // Define the bird's hitbox
        let bird_left = 100.0; // Assuming the bird's X position is constant
        let bird_right = bird_left + 20.0; // Assuming the bird's width is 20
        let bird_top = self.bird_y;
        let bird_bottom = bird_top + 20.0; // Assuming the bird's height is 20
    
        // Define the obstacle's hitboxes
        let obstacle_left = obstacle.x;
        let obstacle_right = obstacle.x + 50.0; // Assuming obstacle width is 50
        let gap_top = obstacle.gap_y;
        let gap_bottom = obstacle.gap_y + obstacle.gap_size;
    
        // Check horizontal overlap
        let horizontal_overlap = bird_right > obstacle_left && bird_left < obstacle_right;
    
        // Check vertical overlap with the top and bottom parts of the obstacle
        let vertical_overlap_top = bird_bottom > 0.0 && bird_top < gap_top;
        let vertical_overlap_bottom = bird_top < self.canvas_height && bird_bottom > gap_bottom; // Use self.canvas_height
    
        horizontal_overlap && (vertical_overlap_top || vertical_overlap_bottom)
    }
    
    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }

}


#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Obstacle {
    pub x: f64,
    pub gap_y: f64,
    pub gap_size: f64,
    pub passed: bool,
}

#[wasm_bindgen]
impl Obstacle {
    pub fn new(x: f64, gap_y: f64, gap_size: f64) -> Obstacle {
        Obstacle { x, gap_y, gap_size, passed: false }
    }

    pub fn update(&mut self) {
        self.x -= 2.0; // Move the obstacle left each frame
    }

    pub fn is_off_screen(&self) -> bool {
        self.x < -50.0 // Determines if the obstacle is off the visible screen
    }
}

#[derive(PartialEq, Eq)]
#[wasm_bindgen]
pub enum GameState {
    Playing,
    GameOver,
}