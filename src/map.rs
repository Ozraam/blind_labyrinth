pub mod item;
pub mod monster;

use std::collections::HashMap;
use crate::player;
use item::Item;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use self::monster::Monster;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn add_x(&mut self, x: i32) {
        self.x += x;
    }

    pub fn add_y(&mut self, y: i32) {
        self.y += y;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    Empty,
    Monster(Monster),
    Treasure(Item, i32),
    Teleport(Position),
    End,
}

pub struct Map {
    map: HashMap<Position, Event>
}

impl Map {
    pub fn new() -> Map {
        Map {
            map: HashMap::new()
        }
    }

    pub fn get_random_position(&self) -> &Position {
        let mut rng = rand::thread_rng();
        let keys: Vec<&Position> = self.map.keys().collect();
        *keys.choose(&mut rng).unwrap()
    }

    pub fn add_event(&mut self, position: Position, event: Event) {
        self.map.insert(position, event);
    }

    pub fn event_at(&self, position: &Position) -> Option<&Event> {
        self.map.get(position)
    }

    pub fn event_at_mut(&mut self, position: &Position) -> Option<&mut Event> {
        self.map.get_mut(position)
    }

    pub fn remove_event(&mut self, position: &Position) {
        self.map.remove(position);
    }

    pub fn change_event(&mut self, position: &Position, event: Event) {
        self.map.insert(*position, event);
    }

    pub fn do_event(&mut self, position: &Position, player: &mut player::Player) -> bool {
        if let Some(event) = self.event_at_mut(position) {
            match event {
                Event::Empty => {},
                Event::Monster(monster) => {
                    player.take_damage_from(&monster.weapon_mut());
                    if let Some(weapon) = player.weapon() {
                        monster.take_damage_from(weapon);
                    }
                    
                    if monster.life() <= 0 {
                        if let Some((drop, qte)) = monster.drop_mut().take() {
                            for _ in 0..qte {
                                player.add_item(drop.clone());
                            }
                        }
                        self.change_event(position, Event::Empty);
                    }
                },
                Event::Treasure(item, _value) => {
                    player.add_item(item.clone());
                    self.change_event(position, Event::Empty);
                },
                Event::Teleport(new_position) => {
                    player.move_to(*new_position);
                },
                Event::End => {
                    return true;
                },
            }
        }

        false
    }

    pub fn generate_map(&mut self, seed: u64, size: u32) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut grid = HashMap::new();
        let start = Position::new(0, 0);
        // use random walk to generate map
        let mut current = start;
        for _ in 0..size {
            let direction = rng.gen_range(0..4);
            match direction {
                0 => current.add_x(1),
                1 => current.add_x(-1),
                2 => current.add_y(1),
                3 => current.add_y(-1),
                _ => unreachable!(),
            }
            let event: Event = Event::Empty; // Change to random event
            grid.insert(current, event);
        }

        grid.insert(start, Event::Empty);

        self.map = grid;

        let mut end = self.get_random_tile_seeded(&mut rng);
        while end.1 != Event::Empty {
            end = self.get_random_tile_seeded(&mut rng);
        }
        
        self.change_event(&end.0, Event::End);
    }

    fn get_random_tile_seeded(&self, rng: &mut StdRng) -> (Position, Event) {
        let index = rng.gen_range(0..self.map.len());
        // sort keys to get deterministic result
        let mut keys: Vec<Position> = self.map.keys().cloned().collect();
        keys.sort();
        let key = keys[index];
        (key, self.map.get(&key).unwrap().clone())
    }

    pub fn print_map(&self, player: &player::Player) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for position in self.map.keys() {
            if position.x < min_x {
                min_x = position.x;
            }
            if position.x > max_x {
                max_x = position.x;
            }
            if position.y < min_y {
                min_y = position.y;
            }
            if position.y > max_y {
                max_y = position.y;
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = Position::new(x, y);
                if &position == player.position() {
                    print!("P");
                } else {
                    match self.event_at(&position) {
                        Some(Event::Empty) => print!("."),
                        Some(Event::Monster(_)) => print!("M"),
                        Some(Event::Treasure(_, _)) => print!("T"),
                        Some(Event::Teleport(_)) => print!("X"),
                        Some(Event::End) => print!("E"),
                        None => print!(" "),
                    }
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;

    use super::*;

    #[test]
    fn test_map_add_event() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Empty;
        map.add_event(position, event);
        assert_eq!(map.event_at(&Position::new(0, 0)), Some(&Event::Empty));
    }

    #[test]
    fn test_map_remove_event() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Empty;
        map.add_event(position, event);
        map.remove_event(&Position::new(0, 0));
        assert_eq!(map.event_at(&Position::new(0, 0)), None);
    }

    #[test]
    fn test_map_do_event_empty() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Empty;
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.life(), 100);
    }

    #[test]
    fn test_map_do_event_monster() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let weapon = Item::new_weapon("Weapon 1", "",10, 10);
        let event = Event::Monster(Monster::new("Monster 1", 100,  weapon, None, 0, 1));
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.life(), 90);
    }

    #[test]
    fn test_map_do_event_monster_with_weapon() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let weapon = Item::new_weapon("Weapon 1","", 10, 10);
        let event = Event::Monster(Monster::new("Monster 1", 100, weapon.clone(), None, 0, 1));
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        player.equip_weapon(Item::new_weapon("Weapon 2", "", 20, 20));
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.life(), 90);
        assert_eq!(map.event_at(&Position::new(0, 0)), Some(&Event::Monster(Monster::new("Monster 1", 80, weapon, None, 0, 1))));
    }

    #[test]
    fn test_map_do_event_monster_die() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Monster(Monster::new("Monster 1", 10,  Item::new_weapon("Weapon 1", "", 10, 10), None, 0, 1));
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        player.equip_weapon(Item::new_weapon("Weapon 2","", 20, 20));
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.life(), 90);
        assert_eq!(map.event_at(&Position::new(0, 0)), Some(&Event::Empty));
    }

    #[test]
    fn test_map_do_event_treasure() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let item = Item::new_weapon("Weapon 1","", 10, 10);
        let event = Event::Treasure(item.clone(), 10);
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.inventory()[0], item);
        assert_eq!(map.event_at(&Position::new(0, 0)), Some(&Event::Empty));
    }

    #[test]
    fn test_map_do_event_teleport() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Teleport(Position::new(1, 1));
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.position(), &Position::new(1, 1));
    }

    #[test]
    fn test_map_do_event_end() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::End;
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        assert_eq!(map.do_event(&Position::new(0, 0), &mut player), true);
    }

    #[test]
    fn test_monster_take_damage() {
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), None, 0, 1);
        monster.take_damage(10);
        assert_eq!(monster.life(), 90);
    }

    #[test]
    fn test_monster_take_damage_from() {
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), None, 0, 1);
        let weapon = Item::new_weapon("Weapon 2", "", 20, 20);
        monster.take_damage_from(&weapon);
        assert_eq!(monster.life(), 80);
    }

    #[test]
    fn test_monster_drop() {
        let monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), Some(Item::new_weapon("Weapon 2", "", 20, 20)), 1, 1);
        assert_eq!(monster.drop(), &Some((Item::new_weapon("Weapon 2", "", 20, 20), 1)));
    }

    #[test]
    fn test_monster_drop_to_player() {
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), Some(Item::new_weapon("Weapon 2", "", 20, 20)), 1, 1);
        let mut player = Player::new("Player 1");
        monster.take_damage(100);
        if let Some((drop, _)) = monster.drop_mut().take() {
            player.add_item(drop);
        }
        assert_eq!(player.inventory()[0], Item::new_weapon("Weapon 2", "", 20, 20));
    }
}