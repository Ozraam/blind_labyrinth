use super::lexer::{Token, Lexer};
use crate::map::Position;
use crate::map::{item::{Item, Effect}, monster::Monster};
use crate::player::Player;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn next_token(&mut self) -> &Token {
        self.position += 1;
        &self.tokens[self.position - 1]
    }

    fn at(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn fetch_weapon(&mut self) -> Option<Item> {
        let mut name = String::new();
        let mut atk = 0;
        let mut description = String::new();
        let mut value = 0;

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Atk => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            atk = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Description => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            description = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Value => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            value = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        Some(Item::new_weapon(&name, &description, value, atk))
    }

    fn fetch_armor(&mut self) -> Option<Item> {
        let mut name = String::new();
        let mut def = 0;
        let mut description = String::new();
        let mut value = 0;

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Def => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            def = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Description => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            description = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Value => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            value = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        Some(Item::new_armor(&name, &description, value, def))
    }

    fn fetch_effect(&mut self) -> Option<Item> {
        let mut name = String::new();
        let mut description = String::new();
        let mut value = 0;
        let mut effect = Effect::None;

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Description => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            description = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Value => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            value = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Effect => {
                    self.next_token();
                    match self.next_token() {
                        Token::Heal => {
                            self.next_token();
                            if let Token::Int(i) = self.next_token() {
                                effect = Effect::Heal(*i);
                            }
                        },
                        Token::Damage => {
                            self.next_token();
                            if let Token::Int(i) = self.next_token() {
                                effect = Effect::Damage(*i);
                            }
                        },
                        Token::Teleport => {
                            self.next_token();
                            match self.next_token() {
                                Token::Start => {
                                    effect = Effect::Teleport(Position::new(0, 0));
                                },
                                Token::Random => {
                                    effect = Effect::RandomTeleport;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        Some(Item::new_effect(&name, &description, value, effect))
    }

    fn fetch_gold(&mut self) -> Option<Item> {
        let mut value = 0;

        match self.next_token() {
            Token::Value => {
                self.next_token();
                match self.next_token() {
                    Token::Int(i) => {
                        value = *i;
                    },
                    _ => {
                        return None;
                    }
                }
            },
            _ => {
                return None;
            }
        }

        Some(Item::new_gold(value))
    }

    fn fetch_exp(&mut self) -> Option<Item> {
        let mut name = "".to_string();
        let mut value = 0;
        let mut description = "".to_string();

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Value => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            value = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Description => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            description = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        Some(Item::new_exp(&name, &description, value))
    }

    pub fn parse_item(&mut self) -> Option<Item> {
        match self.next_token() {
            Token::Item => {
                match self.next_token() {
                    Token::Weapon => {
                        self.fetch_weapon()
                    },
                    Token::Armor => {
                        self.fetch_armor()
                    },
                    Token::Effect => {
                        self.fetch_effect()
                    },
                    Token::Gold => {
                        self.fetch_gold()
                    },
                    Token::Exp => {
                        self.fetch_exp()
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn load_monster(&mut self, items: &Vec<Item>) -> Option<Monster> {
        let mut name = "".to_string();
        let mut life = 0;
        let mut weapon = Item::new_weapon("", "", 0, 0);
        let mut drop = None;
        let mut rareness = 0;

        let is_monster = match self.next_token() {
            Token::Monster => true,
            _ => false,
        };

        if !is_monster {
            println!("Not a monster");
            return None;
        }

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Life => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            life = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Weapon => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            let item = items.iter().find(|item| item.name() == s);
                            if let Some(item) = item {
                                weapon = item.clone();
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Drop => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            let item = items.iter().find(|item| item.name() == s);
                            if let Some(item) = item {
                                if let Token::Int(qte) = self.next_token() {
                                    drop = Some((item.clone(), *qte as u32));
                                }
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Rareness => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            rareness = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        let (item, qte) = drop.unwrap_or((Item::new_gold(0), 0));
        Some(Monster::new(&name, life, weapon, Some(item), qte, rareness as u32))
    }

    fn parse_player_inventory(&mut self, items: &Vec<Item>) -> Vec<Item> {
        let mut inventory = Vec::new();

        // Inventory look like this:
        // Token::Str("Gold") Token::Int(10) the int is the quantity

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Str(s) => {
                    let item = items.iter().find(|item| item.name() == s);
                    if let Some(item) = item {
                        if let Token::Int(qte) = self.next_token() {
                            for _ in 0..*qte {
                                inventory.push(item.clone());
                            }
                        }
                    }
                },
                _ => {
                    break;
                },
            }
        }

        inventory
    }

    pub fn parse_player(&mut self, item: &Vec<Item>) -> Player {
        self.next_token();
        let mut name = "".to_string();
        let mut life = 0;
        let mut max_life = 0;
        let mut weapon = None;
        let mut armor = None;
        let mut exp = 0;
        let mut level = 0;
        let mut next_level = 0;
        let mut inventory = Vec::new();
        let position = Position::new(0, 0);

        while self.at() != &Token::Eof {
            match self.next_token() {
                Token::Name => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            name = s.to_string();
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Life => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            life = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::MaxLife => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            max_life = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Weapon => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            let item = item.iter().find(|item| item.name() == s);
                            if let Some(item) = item {
                                weapon = Some(item.clone());
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Armor => {
                    self.next_token();
                    match self.next_token() {
                        Token::Str(s) => {
                            let item = item.iter().find(|item| item.name() == s);
                            if let Some(item) = item {
                                armor = Some(item.clone());
                            }
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Exp => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            exp = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Level => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            level = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::ExpToLevelUp => {
                    self.next_token();
                    match self.next_token() {
                        Token::Int(i) => {
                            next_level = *i;
                        },
                        _ => {
                            break;
                        }
                    }
                },
                Token::Inventory => {
                    self.next_token();
                    inventory = self.parse_player_inventory(&item);
                },
                _ => {
                    break;
                },
            }
        }

        let mut player = Player::new_all(
            &name, 
            life, 
            max_life, 
            position, 
            weapon, 
            armor, 
            vec![],
            exp, 
            level, 
            next_level, 
        );

        for item in inventory {
            player.add_item(item);
        }

        player
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_weapon() {
        let input = "@item @weapon @name: Sword @atk: 10 @description: A sword @value: 100";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let item = parser.parse_item();
        assert_eq!(item, Some(Item::new_weapon("Sword", "A sword", 100, 10)));
    }

    #[test]
    fn test_parser_armor() {
        let input = "@item @armor @name: Shield @def: 10 @description: A shield @value: 100";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let item = parser.parse_item();
        assert_eq!(item, Some(Item::new_armor("Shield", "A shield", 100, 10)));
    }

    #[test]
    fn test_parser_gold() {
        let input = "@item @gold @value: 100";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let item = parser.parse_item();
        assert_eq!(item, Some(Item::new_gold(100)));
    }

    #[test]
    fn test_parser_exp() {
        let input = "@item @exp @name: Exp @value: 100 @description: An experience point";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let item = parser.parse_item();
        assert_eq!(item, Some(Item::new_exp("Exp", "An experience point", 100)));
    }

    #[test]
    fn test_parser_effect() {
        let input = "@item @effect @name: Heal @value: 10 @description: Heal ten HP @effect: @heal: 10";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let item = parser.parse_item();
        assert_eq!(item, Some(Item::new_effect("Heal", "Heal ten HP", 10, Effect::Heal(10))));
    }

    #[test]
    fn test_parser_monster() {
        let input_item = "@item @gold @value: 10";
        let input_weapon = "@item @weapon @name: Sword @atk: 10 @description: A sword @value: 100";
        
        let mut lexer = Lexer::new(input_item.to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let mut items = vec![parser.parse_item().unwrap()];
        assert_eq!(items[0], Item::new_gold(10));
        
        let mut lexer = Lexer::new(input_weapon.to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        items.push(parser.parse_item().unwrap());
        assert_eq!(items[1], Item::new_weapon("Sword", "A sword", 100, 10));
        
        
        let input = "@monster @name: Goblin @life: 10 @weapon: Sword @drop: Gold 10 @rareness: 1";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let monster = parser.load_monster(&items);
        assert_eq!(monster, Some(Monster::new("Goblin", 10, Item::new_weapon("Sword", "A sword", 100, 10), Some(Item::new_gold(10)), 10, 1)));
    }

    #[test]
    fn test_parser_player() {
        let input_item = "@item @gold @value: 10";
        let input_weapon = "@item @weapon @name: Sword @atk: 10 @description: A sword @value: 100";
        
        let mut lexer = Lexer::new(input_item.to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let mut items = vec![parser.parse_item().unwrap()];
        assert_eq!(items[0], Item::new_gold(10));
        
        let mut lexer = Lexer::new(input_weapon.to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        items.push(parser.parse_item().unwrap());
        assert_eq!(items[1], Item::new_weapon("Sword", "A sword", 100, 10));
        
        let input = "@player @name: Player @life: 10 @max-life: 100 @weapon: Sword @exp: 10 @level: 1 @exp-to-level-up: 100 @inventory: Gold 10";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let player = parser.parse_player(&items);
        assert_eq!(player, 
            Player::new_all(
                "Player", 
                10, 
                100, 
                Position::new(0, 0), 
                Some(Item::new_weapon("Sword", "A sword", 100, 10)), 
                None, 
                vec![Item::new_gold(100)], 
                10, 
                1, 
                100
            ));
    }
}