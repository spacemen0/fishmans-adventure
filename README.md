# Fishmans Adventure
This is a 2d top-down shooter written in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine. It's currently under development


## Usage
- Clone the repo
```bash
git clone git@github.com:spacemen0/fishmans-adventure.git
cd fishmans-adventure
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
  - [x] Implement automatic firing mechanism for the player.
  - [x] Define fire rate and control options for automatic firing.

- [ ] **Re-engineer Enemy Spawning Logic**
  - [x] Implement wave-based spawning system.
  - [x] Define wave progression and difficulty scaling.
  - [ ] Add enemy spawn patterns for different waves.

- [x] **Enemy Projectiles and Collision**
  - [x] Add functionality for enemies to fire projectiles.
  - [x] Implement collision detection between player and enemy projectiles.
  - [x] Prevent overlap between sprites of specific entities

- [x] **Implement invincible time**
  
- [x] **Enhance Enemy Diversity**
  - [x] Create new enemy types with varying behaviors.
  - [x] Define attributes and projectiles firing abilities for each new enemy type.


- [ ] **Loot System**
  - [x] Design loot drop mechanics (e.g., probability, loot pools).
  - [x] Define loot types (potions, weapons, armors, etc.).
  - [ ] Implement loot drop logic for enemies and events.
  - [x] Implement potion usage mechanics and effects.

### User Interface
- [ ] **On-Screen Information**
  - [x] Display wave counts on screen.
  - [ ] Implement switch to toggle on/off on-screen debug messages.

- [ ] **Loot Display GUI**
  - [ ] Design and implement GUI elements for displaying loot information.
  - [ ] Add functionality for showing item details and usage options.

- [x] **Pause Game and Settings**
  - [x] Implement pause game functionality and render pause game screen
  - [ ] Spawn setting screen when pausing game
### World and NPC Features
- [x] **Portal to NPC Town**
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

### Balance Game

