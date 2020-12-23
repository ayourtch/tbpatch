use diffus::{
    edit::{self, collection},
    Diffable, Same,
};
use diffus_derive::Diffus;

#[derive(Diffus, Debug)]
struct Identified {
    id: u32,
    value: u32,
}

impl Same for Identified {
    fn same(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let left = vec![
        Identified { id: 1, value: 0 },
        Identified { id: 2, value: 0 },
        Identified { id: 3, value: 0 },
        Identified { id: 4, value: 0 },
        Identified { id: 5, value: 0 },
        Identified { id: 6, value: 0 },
        Identified { id: 7, value: 0 },
    ];
    let right = vec![
        Identified { id: 1, value: 0 },
        Identified { id: 2, value: 0 },
        Identified { id: 4, value: 0 },
        Identified { id: 3, value: 0 },
        Identified { id: 5, value: 0 },
        Identified { id: 6, value: 0 },
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
                        collection::Edit::Change(EditedIdentified { id, value }) => {
                            println!("changed:");
                            match id {
                                edit::Edit::Copy(x) => println!("    copy: id {:?}", &x),
                                edit::Edit::Change((left_id, right_id)) => {
                                    println!("    id: {} => {}", left_id, right_id)
                                }
                            }
                            match value {
                                edit::Edit::Copy(x) => println!("    copy: value {:?}", &x),
                                edit::Edit::Change((left_value, right_value)) => {
                                    println!("    value: {} => {}", left_value, right_value)
                                }
                            }
                        }
                    };
                })
                .collect::<Vec<_>>();
        }
    };
}
