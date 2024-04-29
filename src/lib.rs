mod player;
mod map;

use player::Player;
use map::Map;

pub fn run() {
    let player = Player::new("Player 1");
    println!("{} has {} life", player.name(), player.life());
}
