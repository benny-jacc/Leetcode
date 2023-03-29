use std::collections::HashMap;

fn main() {
    let result: bool = is_isomorphic(
        String::from("add"),
        String::from("egg")
    );

    println!("{:?}", result);
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut subs: HashMap<char, Vec<usize>> = HashMap::new();

    for (ind, char) in s.chars().enumerate() {
        if subs.contains_key(&char) { continue }

        let ind_s: Vec<usize> = s.match_indices(char)
                                    .map(|(i, _)|i).collect();
        let ind_t: Vec<usize> = t.match_indices(t.chars().nth(ind).unwrap())
                                    .map(|(i, _)|i).collect();
        
        if ind_s == ind_t { subs.insert(char, ind_s); } else { return false; }
    }

    true
}