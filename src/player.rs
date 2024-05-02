use crate::map::{item::{Item, ItemType}, Position};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
    life: i32,
    max_life: i32,
    position: Position,
    weapon: Option<Item>,
    armor: Option<Item>,
    inventory: Vec<Item>,
    experience: i32,
    level: i32,
    next_level: i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            life: 100,
            max_life: 100,
            position: Position::new(0, 0),
            weapon: Some(Item::new_weapon("Stick", "A stick", 0, 1)),
            armor: None,
            inventory: Vec::new(),
            experience: 0,
            level: 1,
            next_level: 100,
        }
    }

    pub fn new_all(name: &str, life: i32, max_life: i32, position: Position, weapon: Option<Item>, armor: Option<Item>, inventory: Vec<Item>, experience: i32, level: i32, next_level: i32) -> Player {
        Player {
            name: name.to_string(),
            life,
            max_life,
            position,
            weapon,
            armor,
            inventory,
            experience,
            level,
            next_level,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn life(&self) -> i32 {
        self.life
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    pub fn weapon(&self) -> Option<&Item> {
        self.weapon.as_ref()
    }

    pub fn equip_weapon(&mut self, weapon: Item) {
        self.weapon = Some(weapon);
    }

    pub fn armor(&self) -> Option<&Item> {
        self.armor.as_ref()
    }

    pub fn equip_armor(&mut self, armor: Item) {
        self.armor = Some(armor);
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    pub fn move_to_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.add_y(-1),
            Direction::Down => self.position.add_y(1),
            Direction::Left => self.position.add_x(-1),
            Direction::Right => self.position.add_x(1),
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.life -= damage - self.armor.as_ref().map_or(0, |armor| armor.value());
        self.life = self.life.min(self.max_life);
        self.life <= 0
    }

    pub fn is_dead(&self) -> bool {
        self.life <= 0
    }

    pub fn take_damage_from(&mut self, weapon: &Item) -> bool {
        match weapon.type_() {
            ItemType::Weapon(damage) => self.take_damage(*damage),
            _ => false,
        }
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn experience(&self) -> i32 {
        self.experience
    }

    pub fn next_level(&self) -> i32 {
        self.next_level
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.next_level = self.next_level * 10 * 15 / 10;
        self.max_life = self.max_life * 10 * 11 / 10;
        self.life = self.max_life;
    }

    pub fn get_gold(&self) -> Option<&Item> {
        self.inventory.iter().find(|item| {
            if let ItemType::Gold(_) = item.type_() {
                true
            } else {
                false
            }
        })
    }

    pub fn get_gold_mut(&mut self) -> Option<&mut Item> {
        self.inventory.iter_mut().find(|item| {
            if let ItemType::Gold(_) = item.type_() {
                true
            } else {
                false
            }
        })
    }

    pub fn add_item(&mut self, item: Item) {
        match item.type_() {
            ItemType::Exp(exp) => {
                self.experience += exp;
                if self.experience >= self.next_level {
                    self.level_up();
                }
            },
            ItemType::Gold(_) => {
                if let Some(gold) = self.get_gold_mut() {
                    gold.add_value(item.value());
                } else {
                    self.inventory.push(item);
                }
            },
            _ => self.inventory.push(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_take_damage() {
        let mut player = Player::new("Player 1");
        player.take_damage(10);
        assert_eq!(player.life(), 90);
    }

    #[test]
    fn test_player_move_to() {
        let mut player = Player::new("Player 1");
        player.move_to(Position::new(1, 1));
        assert_eq!(player.position(), &Position::new(1, 1));
    }

    #[test]
    fn test_player_move_to_direction() {
        let mut player = Player::new("Player 1");
        player.move_to_direction(Direction::Up);
        assert_eq!(player.position(), &Position::new(0, -1));
        player.move_to_direction(Direction::Down);
        assert_eq!(player.position(), &Position::new(0, 0));
        player.move_to_direction(Direction::Left);
        assert_eq!(player.position(), &Position::new(-1, 0));
        player.move_to_direction(Direction::Right);
        assert_eq!(player.position(), &Position::new(0, 0));
    }

    #[test]
    fn test_player_take_damage_from() {
        let mut player = Player::new("Player 1");
        let weapon = Item::new_weapon("Sword", "A sword", 10, 10);
        player.take_damage_from(&weapon);
        assert_eq!(player.life(), 90);
    }

    #[test]
    fn test_player_take_damage_with_armot() {
        let mut player = Player::new("Player 1");
        player.equip_armor(Item::new_armor("Armor", "An armor", 5, 5));
        player.take_damage(10);
        assert_eq!(player.life(), 95);
    }

    #[test]
    fn test_player_level_up() {
        let mut player = Player::new("Player 1");
        player.add_item(Item::new_exp("Exp", "Experience", 100));
        assert_eq!(player.level(), 2);
    }

    #[test]
    fn test_player_add_item_gold() {
        let mut player = Player::new("Player 1");
        player.add_item(Item::new_gold(10));
        player.add_item(Item::new_gold(20));
        assert_eq!(player.get_gold().unwrap().value(), 30);
    }

    #[test]
    fn test_player_add_item_exp() {
        let mut player = Player::new("Player 1");
        player.add_item(Item::new_exp("Exp", "Experience", 100));
        assert_eq!(player.experience(), 100);
    }
}