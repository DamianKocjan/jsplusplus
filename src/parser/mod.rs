use std::{error::Error, fmt};

use self::{io::read_file, keyword::Keyword};

mod io;
mod keyword;
mod token;

#[derive(Debug)]
struct Location {
    file: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
    location: Location,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{} - {}",
            self.location.line, self.location.column, self.message
        )
    }
}

// fn get_file_name(path: &str) -> String {
//     let mut file_name = String::new();
//     let mut is_file_name = false;

//     for c in path.chars().rev() {
//         if c == '/' {
//             break;
//         }

//         if is_file_name {
//             file_name.push(c);
//         }

//         if c == '.' {
//             is_file_name = true;
//         }
//     }

//     file_name.chars().rev().collect()
// }

#[derive(Debug)]
struct Token {
    pub kind: Keyword,
    pub location: Location,
    pub length: usize,
    pub value: String,
}

fn tokenize(file: &str, file_content: String) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::new();

    let mut line = 1;
    let mut column = 1;

    let mut chars = file_content.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // ' ' => {
            //     tokens.push(Token {
            //         kind: Keyword::SPACE,
            //         location: Location {
            //             file: String::from(""),
            //             line,
            //             column,
            //         },
            //         length: 1,
            //         value: None,
            //     });
            // }
            // '\t' => {
            //     tokens.push(Token {
            //         kind: Keyword::SPACE,
            //         location: Location {
            //             file: String::from(""),
            //             line,
            //             column,
            //         },
            //         length: 1,
            //         value: None,
            //     });
            // }
            ' ' | '\t' => continue,

            // comments
            '/' => {
                let mut token = Token {
                    kind: Keyword::COMMENT,
                    location: Location {
                        file: file.to_string(),
                        line,
                        column,
                    },
                    length: 1,
                    value: String::from("/"),
                };

                while let Some(c) = chars.next() {
                    // append to token value
                    token.value.push(c);
                    token.length += 1;

                    if c == '*' {
                        token.kind = Keyword::BLOCK_COMMENT;
                        continue;
                    }

                    if token.kind == Keyword::COMMENT && c == '\n' {
                        break;
                    }
                }
                tokens.push(token);
            }

            _ => {
                let mut last_token = tokens.pop().unwrap();

                if last_token.kind == Keyword::BLOCK_COMMENT {
                    while let Some(c) = chars.next() {
                        if last_token.value.ends_with("*/") {
                            break;
                        }

                        // append to token value
                        last_token.value.push(c);
                        last_token.length += 1;
                    }
                }
                tokens.push(last_token);

                // return Err(ParserError {
                //     message: String::from(format!("Unexpected character - {}", &c.to_string())),
                //     location: Location {
                //         file: String::from(""),
                //         line,
                //         column,
                //     },
                // });
                continue;
            }
        }

        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    Ok(tokens)
}

pub fn parse(path: &str) -> Result<bool, ParserError> {
    let file_content = read_file(path);

    let tokens = tokenize(path, file_content);

    if tokens.is_err() {
        return Err(tokens.unwrap_err());
    }

    let tokens = tokens.unwrap();

    println!("{:?}", tokens);

    Ok(true)
}
