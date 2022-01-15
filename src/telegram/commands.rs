use std::fmt;
use std::str::FromStr;

use crate::storage::Word;
use crate::translate;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Command {
    ListLangs,
    AddLang(translate::Lang),
    DeleteLang(translate::Lang),
    ListWords(String),
    AddWord(Word),
    DeleteWord(String),
}

#[derive(Debug, PartialEq)]
pub struct CommandParseError {
    pub description: String,
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl From<translate::LangParseError> for CommandParseError {
    fn from(e: translate::LangParseError) -> Self {
        CommandParseError {
            description: e.description,
        }
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\s+").unwrap();
        let mut clear_s = s.to_lowercase().to_string();
        clear_s = clear_s.trim().to_string();
        clear_s = re.replace_all(clear_s.as_str(), " ").to_string();
        let parts: Vec<&str> = clear_s.split(" ").collect();

        if parts.is_empty() {
            return Err(CommandParseError {
                description: "Command is empty".to_string(),
            });
        }
        let cmd = match parts[0] {
            "/w" => {
                if parts.len() < 3 {
                    return Err(CommandParseError {
                        description: "Not enough data to add new word".to_string(),
                    });
                }
                Command::AddWord(Word {
                    word: parts[1].to_string(),
                    lang: parts[2].parse()?,
                })
            }
            "/dw" => {
                if parts.len() == 1 {
                    return Err(CommandParseError {
                        description: "No word".to_string(),
                    });
                }
                return Ok(Command::DeleteWord(parts[1].to_string()));
            }
            "/lw" => {
                let mut pt = "".to_string();
                if parts.len() > 1 {
                    pt = parts[1].to_string()
                }
                return Ok(Command::ListWords(pt));
            }
            "/l" => {
                let mut l = "".to_string();
                if parts.len() > 1 {
                    l = parts[1].to_string();
                }
                return Ok(Command::AddLang(l.parse()?));
            }
            "/ll" => return Ok(Command::ListLangs),
            "/dl" => {
                let mut pt = "".to_string();
                if parts.len() > 1 {
                    pt = parts[1].to_string()
                }
                return Ok(Command::DeleteLang(pt.parse()?));
            }
            _ => {
                return Err(CommandParseError {
                    description: "Unknown command".to_string(),
                });
            }
        };

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Word;
    use crate::telegram::commands::{Command, CommandParseError};
    use crate::translate::Lang;
    use std::collections::HashMap;

    #[test]
    fn parse_command() {
        let mut table: HashMap<String, Result<Command, CommandParseError>> = HashMap::new();
        table.insert(
            "some text".to_string(),
            Err(CommandParseError {
                description: "Unknown command".to_string(),
            }),
        );
        table.insert(
            "/w word en".to_string(),
            Ok(Command::AddWord(Word {
                word: "word".to_string(),
                lang: Lang {
                    lang: "en".to_string(),
                },
            })),
        );
        table.insert(
            "/w word en ru".to_string(),
            Ok(Command::AddWord(Word {
                word: "word".to_string(),
                lang: Lang {
                    lang: "en".to_string(),
                },
            })),
        );
        table.insert(
            "/w word enn".to_string(),
            Err(CommandParseError {
                description: "Unsupported language".to_string(),
            }),
        );
        table.insert(
            "/w word".to_string(),
            Err(CommandParseError {
                description: "Not enough data to add new word".to_string(),
            }),
        );
        table.insert(
            "/dw word".to_string(),
            Ok(Command::DeleteWord("word".to_string())),
        );
        table.insert(
            "/dw word word1".to_string(),
            Ok(Command::DeleteWord("word".to_string())),
        );
        table.insert(
            "/dw".to_string(),
            Err(CommandParseError {
                description: "No word".to_string(),
            }),
        );
        table.insert("/lw".to_string(), Ok(Command::ListWords("".to_string())));
        table.insert(
            "/lw wo".to_string(),
            Ok(Command::ListWords("wo".to_string())),
        );
        table.insert(
            "/lw wo w".to_string(),
            Ok(Command::ListWords("wo".to_string())),
        );
        table.insert(
            "/l en".to_string(),
            Ok(Command::AddLang(Lang {
                lang: "en".to_string(),
            })),
        );
        table.insert(
            "/l enn".to_string(),
            Err(CommandParseError {
                description: "Unsupported language".to_string(),
            }),
        );
        table.insert(
            "/dl en".to_string(),
            Ok(Command::DeleteLang(Lang {
                lang: "en".to_string(),
            })),
        );
        table.insert(
            "/dl enn".to_string(),
            Err(CommandParseError {
                description: "Unsupported language".to_string(),
            }),
        );
        table.insert("/ll".to_string(), Ok(Command::ListLangs));
        for (command, expect) in table.iter() {
            let v: Result<self::Command, CommandParseError> = command.parse();
            assert_eq!(expect, &v, "Command: {}", command)
        }
    }
}
