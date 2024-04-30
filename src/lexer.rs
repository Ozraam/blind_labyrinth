use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Item,
    Weapon,
    Armor,
    Gold,
    Effect,
    Exp,

    Treasure,

    Monster,
    Life,
    Drop,

    Atk,
    Def,
    Name,
    Description,
    Value,

    Int(i32),
    Str(String),

    Colon,

    Eof,

    Level,
    MaxLife,
    ExpToLevelUp,
    Inventory,
}

pub struct Lexer {
    input: String,
    position: usize,
    table: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut table = HashMap::new();
        table.insert("@item".to_string(), Token::Item);
        table.insert("@weapon".to_string(), Token::Weapon);
        table.insert("@armor".to_string(), Token::Armor);
        table.insert("@gold".to_string(), Token::Gold);
        table.insert("@effect".to_string(), Token::Effect);
        table.insert("@exp".to_string(), Token::Exp);

        table.insert("@treasure".to_string(), Token::Treasure);

        table.insert("@monster".to_string(), Token::Monster);
        table.insert("@life".to_string(), Token::Life);
        table.insert("@drop".to_string(), Token::Drop);

        table.insert("@atk".to_string(), Token::Atk);
        table.insert("@def".to_string(), Token::Def);
        table.insert("@name".to_string(), Token::Name);
        table.insert("@description".to_string(), Token::Description);
        table.insert("@value".to_string(), Token::Value);

        table.insert("@level".to_string(), Token::Level);
        table.insert("@max-life".to_string(), Token::MaxLife);
        table.insert("@exp-to-level-up".to_string(), Token::ExpToLevelUp);
        table.insert("@inventory".to_string(), Token::Inventory);

        Lexer {
            input,
            position: 0,
            table: table,
        }
    }

    fn is_alphanumericorat(c: char) -> bool {
        c.is_alphabetic() || c.is_numeric() || c == '@' || c == '-'
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut token = None;
        
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c.is_whitespace() {
                self.position += 1;
                continue;
            }

            if c == ':' {
                token = Some(Token::Colon);
                self.position += 1;
                break;
            }

            if c.is_numeric() {
                let mut value = 0;
                while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_numeric() {
                    value = value * 10 + self.input.chars().nth(self.position).unwrap().to_digit(10).unwrap() as i32;
                    self.position += 1;
                }
                token = Some(Token::Int(value));
                break;
            }

            if Lexer::is_alphanumericorat(c) {
                let mut value = String::new();
                while self.position < self.input.len() && Lexer::is_alphanumericorat(self.input.chars().nth(self.position).unwrap()) {
                    value.push(self.input.chars().nth(self.position).unwrap());
                    self.position += 1;
                }
                token = self.table.get(&value).cloned().or(Some(Token::Str(value)));
                break;
            }
        }

        if token == None && self.position >= self.input.len() {
            token = Some(Token::Eof);
        }

        token
    }

    fn combine_string_next_to_each_others(&self, tokens: Vec<Token>) -> Vec<Token> {
        let mut new_tokens = Vec::new();
        let mut iter = tokens.iter().peekable();
        while let Some(token) = iter.next() {
            if let Token::Str(s) = token {
                let mut combined = s.clone();
                while let Some(Token::Str(s)) = iter.peek() {
                    combined.push_str(" ");
                    combined.push_str(s);
                    iter.next();
                }
                new_tokens.push(Token::Str(combined));
            } else {
                new_tokens.push(token.clone());
            }
        }
        new_tokens
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        self.combine_string_next_to_each_others(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "@item @weapon @armor @gold @effect @exp @monster";
        let mut lexer = Lexer::new(input.to_string());
        assert_eq!(lexer.next_token(), Some(Token::Item));
        assert_eq!(lexer.next_token(), Some(Token::Weapon));
        assert_eq!(lexer.next_token(), Some(Token::Armor));
        assert_eq!(lexer.next_token(), Some(Token::Gold));
        assert_eq!(lexer.next_token(), Some(Token::Effect));
        assert_eq!(lexer.next_token(), Some(Token::Exp));
        assert_eq!(lexer.next_token(), Some(Token::Monster));
    }

    #[test]
    fn test_lexer_lex() {
        let input = "@item @weapon @armor @gold @effect @exp @monster";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![
            Token::Item,
            Token::Weapon,
            Token::Armor,
            Token::Gold,
            Token::Effect,
            Token::Exp,
            Token::Monster,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_lexer_lex_with_assign() {
        let input = "@atk: 10";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![
            Token::Atk,
            Token::Colon,
            Token::Int(10),
            Token::Eof,
        ])
    }

    #[test]
    fn test_lexer_lex_with_string() {
        let input = "@name: Sword";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![
            Token::Name,
            Token::Colon,
            Token::Str("Sword".to_string()),
            Token::Eof,
        ])
    }

    #[test]
    fn test_lexer_lex_with_string_and_number() {
        let input = "@name: Sword @value: 10";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![
            Token::Name,
            Token::Colon,
            Token::Str("Sword".to_string()),
            Token::Value,
            Token::Colon,
            Token::Int(10),
            Token::Eof,
        ])
    }

    #[test]
    fn test_lexer_lex_with_string_sentence() {
        let input = "@description: A sharp sword @name: Big Sword name";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![
            Token::Description,
            Token::Colon,
            Token::Str("A sharp sword".to_string()),
            Token::Name,
            Token::Colon,
            Token::Str("Big Sword name".to_string()),
            Token::Eof,
        ])
    }
}