use super::item::{Item, ItemType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monster {
    name: String,
    life: i32,
    weapon: Item,
    drop: Option<(Item, u32)>,
    rareness: u32,
}

impl Monster {
    pub fn new(name: &str, life: i32, weapon: Item, drop: Option<Item>, dropqte: u32, rareness: u32) -> Monster {
        Monster {
            name: name.to_string(),
            life,
            weapon,
            drop: drop.map(|item| (item, dropqte)),
            rareness,
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

    pub fn drop(&self) -> &Option<(Item, u32)> {
        &self.drop
    }

    pub fn weapon(&self) -> &Item {
        &self.weapon
    }

    pub fn drop_mut(&mut self) -> &mut Option<(Item, u32)> {
        &mut self.drop
    }

    pub fn weapon_mut(&mut self) -> &mut Item {
        &mut self.weapon
    }
}