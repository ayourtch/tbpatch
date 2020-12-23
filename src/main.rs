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

fn main() {
    let left = vec![
        TextAtom {
            token_value: format!("1"),
            token_uuid: format!("x"),
            leading_ws: format!("xy"),
        },
        TextAtom {
            token_value: format!("2"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("3"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("4"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("5"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("6"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("7"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("8"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
    ];
    let right = vec![
        TextAtom {
            token_value: format!("1"),
            token_uuid: format!("x"),
            leading_ws: format!("xx"),
        },
        TextAtom {
            token_value: format!("1"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("2"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("1"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("3"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("5"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("6"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
        TextAtom {
            token_value: format!("7"),
            token_uuid: format!(""),
            leading_ws: format!(""),
        },
    ];

    let diff = left.diff(&right);

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
