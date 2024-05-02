mod player;
mod map;
mod data_loader;

use std::{fs::{self, DirEntry}, io};

pub fn run() {
    let items_dir : Vec<io::Result<DirEntry>> = fs::read_dir(".\\data\\items").unwrap().collect();
    let items_length = items_dir.len();
    let mut items = vec![];

    let monsters_dir : Vec<io::Result<DirEntry>> = fs::read_dir(".\\data\\monsters").unwrap().collect();
    let monsters_length = monsters_dir.len();
    let mut monsters = vec![];

    let player_path = ".\\data\\player.blind";
    println!("Loading items");
    for (i, path) in items_dir.iter().enumerate() {
        let path = path.as_ref().unwrap().path();
        let item_string = fs::read_to_string(path).unwrap();
        let item: Option<map::item::Item> = data_loader::Parser::new(data_loader::Lexer::new(item_string).lex()).parse_item();
        println!("{}/{} : {}", i + 1, items_length, item.as_ref().unwrap().name());
        match item {
            Some(item) => items.push(item),
            None => println!("Error parsing item {}", i + 1),
        }
    }

    println!("Loading monsters");
    for (i, path) in monsters_dir.iter().enumerate() {
        let path = path.as_ref().unwrap().path();
        let monster_string = fs::read_to_string(path).unwrap();
        let monster: Option<map::monster::Monster> = data_loader::Parser::new(data_loader::Lexer::new(monster_string).lex()).load_monster(&items);
        println!("{}/{} : {}", i + 1, monsters_length, monster.as_ref().unwrap().name());
        match monster {
            Some(monster) => monsters.push(monster),
            None => println!("Error parsing monster {}", i + 1),
        }
    }
    println!("Loading player");
    let player_string = fs::read_to_string(player_path).unwrap();
    let player = data_loader::Parser::new(data_loader::Lexer::new(player_string).lex()).parse_player(&items);
    println!("1/1 : {:?}", player);

    let mut map = map::Map::new();
    map.generate_map(4747515738017, 1000);
    map.print_map(&player);
}
