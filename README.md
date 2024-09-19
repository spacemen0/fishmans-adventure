# Bevy 2D Shooter
This is a 2d top-down shooter written in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine. It's capable of handling 100K enemies and uses a kd-tree to efficiently handle the collisions.


## Usage
- Clone the repo
```bash
git clone git@github.com:spacemen0/bevy-2d-shooter.git
cd bevy-2d-shooter
```
- Run
```bash
cargo run
```

## Configurations
- The project config file is located at `src/configs.rs`

## Credits
- Game assets - [https://0x72.itch.io/dungeontileset-ii](https://0x72.itch.io/dungeontileset-ii)
- Monogram Font - [https://datagoblin.itch.io/monogram](https://datagoblin.itch.io/monogram)

## Controls
- `WASD` for movement
- Mouse wheel to change camera zoom

## Todo

### Gameplay Mechanics

- [x] **Automatic Player Fire**
  - [ ] Implement automatic firing mechanism for the player.
  - [x] Define fire rate and control options for automatic firing.

- [ ] **Re-engineer Enemy Spawning Logic**
  - [x] Implement wave-based spawning system.
  - [ ] Define wave progression and difficulty scaling.
  - [ ] Add enemy spawn patterns for different waves.

- [ ] **Enemy Projectiles and Collision**
  - [ ] Add functionality for enemies to fire projectiles.
  - [ ] Implement collision detection between player and enemy projectiles.
  - [ ] Prevent overlap between sprites of specific entities
- [x] Implement invincible time
  
- [ ] **Enhance Enemy Diversity**
  - [ ] Create new enemy types with varying behaviors.
  - [ ] Define attributes and projectiles firing abilities for each new enemy type.


- [ ] **Loot System**
  - [ ] Design loot drop mechanics (e.g., probability, loot pools).
  - [ ] Define loot types (potions, weapons, armors, etc.).
  - [ ] Implement loot drop logic for enemies and events.
  - [ ] Implement potion usage mechanics and effects.

### User Interface
- [ ] **On-Screen Information**
  - [x] Display wave counts on screen.
  - [ ] Implement switch to toggle on/off on-screen debug messages.

- [ ] **Loot Display GUI**
  - [ ] Design and implement GUI elements for displaying loot information.
  - [ ] Add functionality for showing item details and usage options.

- [x] **Pause Game and Settings**
  - [x] Implement pause game functionality and render pause game screen
### World and NPC Features
- [ ] **Portal to NPC Town**
  - [ ] Design and implement a portal or transition system to the NPC town.

- [ ] **NPC Town Map**
  - [ ] Create a visual map for the NPC town.
  - [ ] Implement navigation and interaction within the town.

- [ ] **Trade System**
  - [ ] Develop a trading system for buying and selling loot.
  - [ ] Implement NPC interactions for trade.
  - [ ] Define tradeable items and pricing mechanisms.

### Bug Fix
- [ ] Health should reset when starting new game

