use std::{collections::HashSet, fs::File, io::Read, path::Path};

fn get_names() -> Vec<String> {
    let mut file = File::open(Path::new("src/names.json")).expect("Could not open names.json");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read names.json");

    serde_json::from_str::<Vec<String>>(&contents).expect("Could not convert to vector from JSON")
}

fn check_if_permutation(name1: String, name2: String) -> bool {
    let name1_chars: HashSet<char> = name1.chars().collect();
    let name2_chars: HashSet<char> = name2.chars().collect();

    name1_chars == name2_chars
}

fn filter_names(
    permuted_names: Vec<(String, String)>,
    char_aligned: usize,
) -> Vec<(String, String)> {
    let mut filtered_names: Vec<(String, String)> = Vec::new();
    'outer: for (name1, name2) in permuted_names {
        let chars_name1: Vec<char> = name1.chars().collect();
        let chars_name2: Vec<char> = name2.chars().collect();

        for idx in 0..char_aligned {
            if chars_name1[idx] != chars_name2[idx] {
                break 'outer;
            }
        }

        filtered_names.push((name1, name2));
    }

    filtered_names
}

trait VecDifference {
    fn difference(&self, rhs: Vec<char>) -> Vec<char>;
}

impl VecDifference for Vec<char> {
    fn difference(&self, rhs: Vec<char>) -> Vec<char> {
        let diffed: Vec<char> = Vec::new();
        for idx in 0..min(self.len(), &rhs.len()) {
            if !self.contains[&rhs[idx]] {
                diffed.push(rhs[idx])
            }
        }

        diffed
    }
}

fn main() {
    let names = get_names();

    let mut names_filtered: Vec<(String, String)> = Vec::new();
    for idx in 0..(names.len() - 1) {
        if check_if_permutation(names[idx].clone(), names[idx + 1].clone()) {
            names_filtered.push((names[idx].clone(), names[idx + 1].clone()));
        }
    }

    names_filtered = filter_names(names_filtered, 3);
    println!("{:?}", names_filtered)
}
