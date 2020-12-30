use ansi_term::Colour;
use diffus::edit::{self, collection};
use diffus::{Diffable, Same};
use diffus_derive::Diffus;

#[derive(Diffus, Debug, Clone)]
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

impl PartialEq for TextAtom {
    fn eq(&self, other: &Self) -> bool {
        self.token_value == other.token_value
    }
}

#[derive(Diffus, Debug, Clone)]
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
        // println!("Token: {:#?}, delta_i: {}", &token, delta_i);
        atoms.push(token);
        i = i + delta_i;
    }
    ParseStruct { atoms }
}

fn parse_file(fname: &str) -> ParseStruct {
    let fdata = std::fs::read_to_string(fname).unwrap();
    parse_string(&fdata)
}

fn read_nth_arg(i: usize) -> String {
    let fname = std::env::args().nth(i).unwrap();
    let fdata = std::fs::read_to_string(fname).unwrap();
    fdata
}

fn parse_nth_arg(i: usize) -> ParseStruct {
    let fdata = read_nth_arg(i);
    parse_string(&fdata)
}

fn print_diff<'a>(diff: diffus::edit::Edit<'a, ParseStruct>) {
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

fn atom2str(atom: &TextAtom) -> String {
    format!("{}{}", atom.leading_ws, atom.token_value)
}

fn print_diff_c<'a>(right: &ParseStruct, diff: diffus::edit::Edit<'a, ParseStruct>) {
    let mut atom_index = 0;
    match diff {
        edit::Edit::Copy(x) => {
            println!("Identical parses: {:#?}", &x);
        }
        edit::Edit::Change(EditedParseStruct { atoms }) => {
            let diff = atoms;
            match diff {
                edit::Edit::Copy(x) => {
                    x.into_iter().map(|xx| {
                        print!("{}", &atom2str(&xx));
                        atom_index = atom_index + 1;
                    });
                }
                edit::Edit::Change(diff) => {
                    diff.into_iter()
                        .map(|edit| {
                            match edit {
                                collection::Edit::Copy(elem) => {
                                    print!("{}", &atom2str(&elem));
                                    atom_index = atom_index + 1;
                                }
                                collection::Edit::Insert(elem) => {
                                    print!("{}", Colour::Green.paint(atom2str(&elem)));
                                    atom_index = atom_index + 1;
                                }
                                collection::Edit::Remove(elem) => {
                                    print!("{}", Colour::Red.paint(atom2str(&elem)));
                                }
                                collection::Edit::Change(EditedTextAtom {
                                    token_value,
                                    token_uuid,
                                    leading_ws,
                                }) => {
                                    match token_value {
                                        edit::Edit::Copy(x) => {
                                            let ws = &right.atoms[atom_index].leading_ws;
                                            let tok = &right.atoms[atom_index].token_value;
                                            print!(
                                                "{}",
                                                Colour::Purple.paint(format!("{}{}", ws, &x))
                                            );
                                        }
                                        x => {
                                            println!("    changed: id {:?}", &x);
                                        } /*
                                          edit::Edit::Change((left_id, right_id)) => {
                                              println!("    token_value: {} => {}", left_id, right_id)
                                          }
                                          */
                                    }
                                    atom_index = atom_index + 1;
                                    /*
                                    println!("    token_uuid: {:?}", &token_uuid);
                                    println!("    leading_ws: {:?}", &leading_ws);
                                    */
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
    println!("");
}

fn test_diffus() {
    let left = parse_nth_arg(1);
    let right = parse_nth_arg(2);

    let diff = left.diff(&right);
    print_diff(diff);
}

fn join_lines(lines: &Vec<unidiff::Line>) -> String {
    format!(
        "\n{}",
        lines
            .into_iter()
            .map(|x| x.value.clone())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

enum HunkOp {
    ShowDiff,
    Patch(usize),
}

pub fn find_needle<N, H>(needle: &[N], haystack: &[H]) -> Option<usize>
where
    N: PartialEq + std::cmp::PartialEq<H> + std::fmt::Debug,
    H: PartialEq<N> + std::fmt::Debug,
{
    for (hi, ch) in haystack.into_iter().enumerate() {
        let mut match_count = 0;
        for (ni, cn) in needle.into_iter().enumerate() {
            if hi + ni < haystack.len() && cn == &haystack[hi + ni] {
                match_count = match_count + 1;
            }
        }
        if match_count > needle.len() / 2 {
            // println!("   matches: {} / {}", match_count, needle.len());
        }

        if match_count == needle.len() {
            return Some(hi);
        }
    }
    None
}

fn parse_patched_file(file: &unidiff::PatchedFile, p: usize) -> ParseStruct {
    let path = std::path::Path::new(&file.source_file);
    let mut comp = path.components();
    // I can't do this: let path = path.components().skip(p).as_path();
    // So I will do this:
    for i in 0..p {
        comp.next();
    }
    let path = comp.as_path();
    let src_path = &path.to_str().unwrap();

    println!("src path: {}", &src_path);
    parse_file(&src_path)
}

fn do_patch(
    src_file: &ParseStruct,
    file: &unidiff::PatchedFile,
    hunk: &unidiff::Hunk,
) -> ParseStruct {
    let src = join_lines(&hunk.source_lines())
        .to_string()
        .trim_end_matches(char::is_whitespace)
        .to_string();
    let src = parse_string(&src);

    let dst = join_lines(&hunk.target_lines())
        .to_string()
        .trim_end_matches(char::is_whitespace)
        .to_string();
    let dst = parse_string(&dst);

    let diff = src.diff(&dst);

    print_diff_c(&dst, diff);
    let find_pos = find_needle(&src.atoms, &src_file.atoms);
    println!("FindPos: {:?} (of {})", &find_pos, src.atoms.len());
    if let Some(p) = find_pos {
        let mut out_file = ParseStruct {
            atoms: src_file.atoms[0..p].to_vec(),
        };
        /* FIXME: apply edit ops */
        /* FIXME: copy the rest of src_file */
        return src_file.clone();
        return out_file;
    } else {
        // println!("needle: {:?}", &src.atoms);
        // println!("haystack: {:?}", &src_file.atoms)
        panic!("Can not find context");
    }

    return src_file.clone();
}

fn test_unidiff() {
    let diff_str = read_nth_arg(1);
    let mut patch = unidiff::PatchSet::new();
    patch.parse(diff_str).ok().expect("Error parsing diff");
    // println!("{:#?}", &patch);
    let hunk = patch.files().first().unwrap().hunks().first().unwrap();
    for file in patch.files() {
        println!("{}", Colour::Cyan.paint("==================="));
        println!("{} {}", Colour::Cyan.paint("==="), file.source_file);
        println!("{} {}", Colour::Cyan.paint("==="), file.target_file);
        let mut src_file = parse_patched_file(file, 1);
        for hunk in file.hunks() {
            println!("{} {}", Colour::Cyan.paint("==="), hunk.section_header);
            println!(
                "{} {:+0} lines [ {}[{}] {}[{}] ] =>\n",
                Colour::Cyan.paint("==="),
                hunk.added() as i64 - hunk.removed() as i64,
                hunk.source_start,
                hunk.source_length,
                hunk.target_start,
                hunk.target_length
            );
            src_file = do_patch(&src_file, file, hunk);
        }
    }
    // let src = hunk.source_lines().into_iter().map(|x| x.value.clone()).collect::<Vec<&str>>().join("\n");

    // println!("{:#?}", &hunk.target_lines().map(|x| x.value).collect());
}

fn main() {
    test_unidiff();
    println!("\n");
}
