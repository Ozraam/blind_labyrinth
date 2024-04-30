use std::io::BufRead;

use crate::player;
use super::Monster;
use crate::map::Position;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Effect {
    Heal(i32),
    Damage(i32),
    Teleport(Position),
}

impl Effect {
    pub fn apply(&self, player: &mut player::Player, monster: Option<&mut Monster>) {
        match self {
            Effect::Heal(heal) => {
                player.take_damage(-*heal); 
                ()
            },
            Effect::Damage(damage) => {
                if let Some(monster) = monster {
                    monster.take_damage(*damage);
                }
            },
            Effect::Teleport(position) => player.move_to(*position),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ItemType {
    Weapon(i32),
    Armor(i32),
    Gold(i32),
    Effect(Effect),
    Exp(i32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
    name: String,
    description: String,
    value: i32,
    type_: ItemType,
}

impl Item {
    pub fn new_weapon(name: &str, description: &str, value: i32, damage: i32) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            value,
            type_: ItemType::Weapon(damage),
        }
    }

    pub fn new_armor(name: &str, description: &str, value: i32, defense: i32) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            value,
            type_: ItemType::Armor(defense),
        }
    }

    pub fn new_gold(value: i32) -> Item {
        Item {
            name: "Gold".to_string(),
            description: "A pile of gold".to_string(),
            value,
            type_: ItemType::Gold(value),
        }
    }

    pub fn new_effect(name: &str, description: &str, value: i32, effect: Effect) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            value,
            type_: ItemType::Effect(effect),
        }
    }

    pub fn new_exp(name: &str, description: &str, value: i32) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            value,
            type_: ItemType::Exp(value),
        }
    }

    pub fn use_item(&self) -> Option<&Effect> {
        match self.type_ {
            ItemType::Effect(ref effect) => Some(effect),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn type_(&self) -> &ItemType {
        &self.type_
    }

    pub fn add_value(&mut self, value: i32) {
        self.value += value;
    }
}

#[cfg(test)]
mod tests {
    use crate::map;

    use super::*;

    #[test]
    fn test_item_new_weapon() {
        let item = Item::new_weapon("Sword", "A sharp sword", 10, 5);
        assert_eq!(item.name(), "Sword");
        assert_eq!(item.description(), "A sharp sword");
        assert_eq!(item.value(), 10);
        assert_eq!(item.type_(), &ItemType::Weapon(5));
    }

    #[test]
    fn test_item_new_armor() {
        let item = Item::new_armor("Shield", "A sturdy shield", 10, 5);
        assert_eq!(item.name(), "Shield");
        assert_eq!(item.description(), "A sturdy shield");
        assert_eq!(item.value(), 10);
        assert_eq!(item.type_(), &ItemType::Armor(5));
    }

    #[test]
    fn test_item_new_gold() {
        let item = Item::new_gold(10);
        assert_eq!(item.name(), "Gold");
        assert_eq!(item.description(), "A pile of gold");
        assert_eq!(item.value(), 10);
        assert_eq!(item.type_(), &ItemType::Gold(10));
    }

    #[test]
    fn test_item_new_effect() {
        let effect = Effect::Heal(5);
        let item = Item::new_effect("Potion", "A healing potion", 10, effect.clone());
        assert_eq!(item.name(), "Potion");
        assert_eq!(item.description(), "A healing potion");
        assert_eq!(item.value(), 10);
        assert_eq!(item.type_(), &ItemType::Effect(effect));
    }

    #[test]
    fn test_item_use_item() {
        let effect = Effect::Heal(5);
        let item = Item::new_effect("Potion", "A healing potion", 10, effect.clone());
        assert_eq!(item.use_item(), Some(&effect));
    }

    #[test]
    fn test_effect_apply_heal() {
        let mut player = player::Player::new("Player");
        let monster = Monster::new("Monster", 10, Item::new_weapon("Weapon", "", 0, 10), None, 1);
        let event = map::Event::Monster(monster);
        let mut map = map::Map::new();
        map.add_event(Position::new(0, 0), event);
        map.do_event(&Position::new(0, 0), &mut player);

        let effect = Effect::Heal(5);
        effect.apply(&mut player, None);
        assert_eq!(player.life(), 95);
    }

    #[test]
    fn test_effect_apply_damage() {
        let mut player = player::Player::new("Player");
        let mut monster = Monster::new("Monster", 10, Item::new_weapon("Weapon", "", 0, 5), None, 0);
        let effect = Effect::Damage(5);
        effect.apply(&mut player, Some(&mut monster));
        assert_eq!(monster.life(), 5);
    }

    #[test]
    fn test_effect_apply_teleport() {
        let mut player = player::Player::new("Player");
        let effect = Effect::Teleport(Position::new(1, 1));
        effect.apply(&mut player, None);
        assert_eq!(player.position(), &Position::new(1, 1));
    }
}