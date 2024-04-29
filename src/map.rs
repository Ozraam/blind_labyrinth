pub mod item;
use std::collections::HashMap;
use crate::player;
use item::{Item, ItemType};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Monster {
    name: String,
    life: i32,
    weapon: Item,
    drop: Option<(Item, u32)>,
}

impl Monster {
    pub fn new(name: &str, life: i32, weapon: Item, drop: Option<Item>, dropqte: u32) -> Monster {
        Monster {
            name: name.to_string(),
            life,
            weapon,
            drop: drop.map(|item| (item, dropqte)),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn life(&self) -> i32 {
        self.life
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.life -= damage;
    }

    pub fn take_damage_from(&mut self, weapon: &Item) {
        match weapon.type_() {
            ItemType::Weapon(damage) => self.take_damage(*damage),
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
                    player.take_damage_from(&monster.weapon);
                    if let Some(weapon) = player.weapon() {
                        monster.take_damage_from(weapon);
                    }
                    
                    if monster.life() <= 0 {
                        if let Some((drop, qte)) = monster.drop.take() {
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
        let event = Event::Monster(Monster::new("Monster 1", 100,  weapon, None, 0));
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
        let event = Event::Monster(Monster::new("Monster 1", 100, weapon.clone(), None, 0));
        map.add_event(position, event);
        let mut player = Player::new("Player 1");
        player.equip_weapon(Item::new_weapon("Weapon 2", "", 20, 20));
        map.do_event(&Position::new(0, 0), &mut player);
        assert_eq!(player.life(), 90);
        assert_eq!(map.event_at(&Position::new(0, 0)), Some(&Event::Monster(Monster::new("Monster 1", 80, weapon, None, 0))));
    }

    #[test]
    fn test_map_do_event_monster_die() {
        let mut map = Map::new();
        let position = Position::new(0, 0);
        let event = Event::Monster(Monster::new("Monster 1", 10,  Item::new_weapon("Weapon 1", "", 10, 10), None, 0));
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
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), None, 0);
        monster.take_damage(10);
        assert_eq!(monster.life(), 90);
    }

    #[test]
    fn test_monster_take_damage_from() {
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), None, 0);
        let weapon = Item::new_weapon("Weapon 2", "", 20, 20);
        monster.take_damage_from(&weapon);
        assert_eq!(monster.life(), 80);
    }

    #[test]
    fn test_monster_drop() {
        let monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), Some(Item::new_weapon("Weapon 2", "", 20, 20)), 1);
        assert_eq!(monster.drop, Some((Item::new_weapon("Weapon 2", "", 20, 20), 1)));
    }

    #[test]
    fn test_monster_drop_to_player() {
        let mut monster = Monster::new("Monster 1", 100, Item::new_weapon("Weapon 1", "", 10, 10), Some(Item::new_weapon("Weapon 2", "", 20, 20)), 1);
        let mut player = Player::new("Player 1");
        monster.take_damage(100);
        if let Some((drop, _)) = monster.drop.take() {
            player.add_item(drop);
        }
        assert_eq!(player.inventory()[0], Item::new_weapon("Weapon 2", "", 20, 20));
    }
}