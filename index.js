import init, { Game } from './pkg/rust_flappy_bird.js';

document.addEventListener('DOMContentLoaded', async () => {
    await init(); // Initialize the WASM module

    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');
    let game; // Define game here, but initialize it in runGame function

    const gameOverScreen = document.getElementById('gameOverScreen');
    const tryAgainButton = document.getElementById('tryAgainButton');

    const scoreDisplay = document.getElementById('scoreDisplay');

    function updateScoreDisplay() {
        const score = game.get_score();
        scoreDisplay.textContent = `Score: ${score}`;
    }

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height); // Clear the canvas
        
        // Draw the bird
        ctx.fillStyle = 'red';
        ctx.beginPath();
        ctx.ellipse(100, game.bird_y, 20, 20, 0, 0, 2 * Math.PI); // Adjusted the bird size for visibility
        ctx.fill();

        // Draw obstacles
        const obstacleCount = game.get_obstacle_count();
        for (let i = 0; i < obstacleCount; i++) {
            const obstacle = game.get_obstacle_data(i);
            if (!obstacle) continue; // Check if obstacle is correctly fetched

            // Assuming obstacleData is now correctly structured as an object
            ctx.fillStyle = 'green';
            // Top part
            ctx.fillRect(obstacle.x, 0, 50, obstacle.gap_y);
            // Bottom part
            ctx.fillRect(obstacle.x, obstacle.gap_y + obstacle.gap_size, 50, canvas.height - (obstacle.gap_y + obstacle.gap_size));
        }
    }

    function update() {
        if (game.get_state() === "GameOver") {
            gameOverScreen.style.display = "block"; // Show the game over screen
            return; // Exit the update loop
        }
    
        game.update(); // Update game state before drawing
        draw(); // Draw the game state
        updateScoreDisplay();
        requestAnimationFrame(update); // Continue the game loop
    }

    function runGame() {
        game = Game.new(); // Initialize the game
        gameOverScreen.style.display = "none"; // Ensure game over screen is hidden
        updateScoreDisplay(); // Initialize score display with the starting score
        requestAnimationFrame(update); // Start the game loop
    }

    document.addEventListener('keydown', (event) => {
        if (event.code === 'Space' && game.get_state() !== "GameOver") {
            game.flap();
        }
    });

    tryAgainButton.addEventListener('click', runGame); // Attach the runGame function to the button

    runGame(); // Initialize and start the game
});
