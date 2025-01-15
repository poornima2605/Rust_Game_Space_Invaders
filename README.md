# Space Invaders Game

A classic **Space Invaders** arcade-style game implemented in **Rust**. This project showcases the use of Rust for game development and demonstrates efficient rendering, gameplay mechanics, and user interactions.

---

## 🎮 Features

- **Classic Gameplay:** Replicates the nostalgic feel of the original Space Invaders.
- **Dynamic Difficulty:** Enemies move faster as the game progresses.
- **Player Controls:** Intuitive controls for smooth gameplay.
- **Sound Effects:** Optional sound effects for an immersive experience (if implemented).
- **Game Over and Restart Options:** Seamless restart functionality.

---

## 🛠️ Built With

- **Rust:** A systems programming language for high performance and safety.
- **Game Engine/Framework:** [Bevy](https://bevyengine.org/) (or another Rust game framework, specify if used).
- **Graphics and Rendering:** Implemented using Rust’s powerful libraries.

---

## 🚀 Getting Started

Follow these instructions to get a copy of the project up and running on your local machine.

### Prerequisites

Make sure you have the following installed:

- **Rust:** Install Rust via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo:** Comes bundled with Rust for managing dependencies and building the project.


## 🎮 How to Play

- **Move Left/Right:** Use the arrow keys (`←`, `→`) or `A`/`D` keys.
- **Shoot:** Press the `Space` key to fire at the enemies.
- **Objective:** Destroy all enemies while avoiding their attacks.
- **Game Over:** The game ends if enemies reach the bottom of the screen or your ship is hit.

---

## 🧩 File Structure

```plaintext
src/
├── main.rs         # Entry point of the game
├── player.rs       # Player logic and controls
├── enemy.rs        # Enemy behavior and mechanics
├── bullet.rs       # Bullet physics and interactions
├── game_state.rs   # Game state management
└── utils.rs        # Helper functions and utilities
assets/             # Assets like images, sounds, and fonts
Cargo.toml          # Rust project dependencies and configuration
```

