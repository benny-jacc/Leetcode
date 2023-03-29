fn main() {
    println!("{:?}", is_subsequence(
        String::from("abc"), 
        String::from("aaabbbeeeccc"))
    );
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect::<Vec<_>>();
    
    let mut it: usize = 0;
    
    let mut current: &char = match s_chars.get(it) {
        Some(char) => char,
        None => return true
    };

    for ch in t.chars() {
        if current.eq(&ch) { 
            it += 1;

            current = match s_chars.get(it) {
                Some(char) => char,
                None => return true
            }
        } 
        else { continue }
    }
    
    false
}
