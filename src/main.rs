pub mod asciicontext;
pub mod asteroid;
pub mod bullet;
pub mod drawables;
pub mod drawingcontext;
pub mod enemy;
pub mod game;
pub mod leaderboard;
pub mod logo;
pub mod menu;
pub mod menu_help;
pub mod menu_objects;
pub mod particle;
pub mod plasma;
pub mod powerup;
pub mod ship;
pub mod sprite;
pub mod starship;
pub mod terminaldrawable;

use crate::menu::*;

fn main() {
    menu();
}
