use diffus::edit::{self, collection};
use diffus::{Diffable, Same};
use diffus_derive::Diffus;

#[derive(Diffus, Debug)]
struct TextAtom {
    token_value: String,
    token_uuid: String,
    leading_ws: String,
}

impl Same for TextAtom {
    fn same(&self, other: &Self) -> bool {
        if self.token_uuid == "" {
            if other.token_uuid == "" {
                /* we assume that with empty IDs the tokens can match */
                self.token_value == other.token_value
            } else {
                false // one tagged, the other not = no match
            }
        } else {
            if other.token_uuid == "" {
                false // one tagged, the other not = no match
            } else {
                self.token_uuid == other.token_uuid && self.token_value == other.token_value
            }
        }
    }
}

#[derive(Diffus, Debug)]
struct ParseStruct {
    atoms: Vec<TextAtom>,
}

enum ParseTokenState {
    LeadingWhiteSpace,
    TokenValue,
}

fn parse_token(input: &str, i: usize) -> (Option<TextAtom>, usize) {
    let mut atom = TextAtom {
        token_value: format!(""),
        token_uuid: format!(""),
        leading_ws: format!(""),
    };

    if i >= input.len() {
        return (None, 0);
    }

    let mut state = ParseTokenState::LeadingWhiteSpace;

    let char_indices = input[i..].char_indices();
    for (ci, ch) in char_indices {
        match state {
            ParseTokenState::LeadingWhiteSpace => {
                if ch.is_whitespace() {
                    atom.leading_ws.push(ch)
                } else {
                    atom.token_value.push(ch);
                    state = ParseTokenState::TokenValue;
                }
            }
            ParseTokenState::TokenValue => {
                if ch.is_whitespace() {
                    return (Some(atom), ci);
                } else {
                    atom.token_value.push(ch);
                }
            }
        }
    }

    (Some(atom), input[i..].len())
}

fn parse_string(input: &str) -> ParseStruct {
    let mut atoms: Vec<TextAtom> = vec![];
    let mut i = 0;

    while let (Some(token), delta_i) = parse_token(input, i) {
        println!("Token: {:#?}, delta_i: {}", &token, delta_i);
        atoms.push(token);
        i = i + delta_i;
    }
    ParseStruct { atoms }
}

fn main() {
    let left = parse_string("    xx yy    zzz   ");
    let right = parse_string("xx tt    zzz   ");

    let diff = left.diff(&right);
    match diff {
        edit::Edit::Copy(x) => {
            println!("Identical parses: {:#?}", &x);
        }
        edit::Edit::Change(EditedParseStruct { atoms }) => {
            let diff = atoms;
            match diff {
                edit::Edit::Copy(x) => println!("no difference: {:?}", &x),
                edit::Edit::Change(diff) => {
                    diff.into_iter()
                        .map(|edit| {
                            match edit {
                                collection::Edit::Copy(elem) => println!("copy: {:?}", elem),
                                collection::Edit::Insert(elem) => println!("insert: {:?}", elem),
                                collection::Edit::Remove(elem) => println!("remove: {:?}", elem),
                                collection::Edit::Change(EditedTextAtom {
                                    token_value,
                                    token_uuid,
                                    leading_ws,
                                }) => {
                                    println!("changed:");
                                    match token_value {
                                        edit::Edit::Copy(x) => println!("    copy: id {:?}", &x),
                                        x => {
                                            println!("    changed: id {:?}", &x);
                                        } /*
                                          edit::Edit::Change((left_id, right_id)) => {
                                              println!("    token_value: {} => {}", left_id, right_id)
                                          }
                                          */
                                    }
                                    println!("    token_uuid: {:?}", &token_uuid);
                                    println!("    leading_ws: {:?}", &leading_ws);
                                    /*
                                    match leading_ws {
                                        edit::Edit::Copy(x) => println!("    copy: ws {:?}", &x),
                                        edit::Edit::Change((left_ws, right_ws)) => {
                                            println!("    value: {} => {}", left_ws, right_ws)
                                        }
                                    }
                                    */
                                }
                            };
                        })
                        .collect::<Vec<_>>();
                }
            };
        }
    }
}
