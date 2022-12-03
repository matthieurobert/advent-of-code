use std::collections::HashMap;

fn main() {
    let input = include_str!("./day3/input.txt");

    let res1 = problem1(input);
    let res2 = problem2(input);

    println!("Problem 1 : {}", res1);
    println!("Problem 2 : {}", res2);
}

pub fn problem1(input: &str) -> i32 {
    let parsed = input.split("\n");
    let mut sum = 0;

    for e in parsed {
        let (_begin, _end) = e.split_at(e.len() / 2);
        
        let mut same: char = ' ';
        
        for (_, b) in _begin.chars().enumerate() {
            if _end.contains(b) {
                same = b;
            }
        }
        sum += get_char_value(same);
    }
    
    sum
}

pub fn problem2(input: &str) -> i32 {
    let mut sum = 0;
    let mut parsed = input.split("\n");
    let len = input.split("\n").count();    
    
    for _ in (1..len).step_by(3) {
        let _one = parsed.next().unwrap();
        let _two = parsed.next().unwrap();
        let _three = parsed.next().unwrap();

        let mut same = Vec::new();

        for (_, b) in _one.chars().enumerate() {
            if _two.contains(b) {
                same.push(b);
            }
        }
        
        let mut def = ' ';

        for (_, b) in _three.chars().enumerate() {
            if same.contains(&b) {
                def = b;
            }
        }

        let val = get_char_value(def);
        sum += val
    }

    sum
}

fn get_char_value(c: char) -> i32 {
    let mut hmap = HashMap::new();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for (i, e) in chars.chars().enumerate() {
        hmap.entry(e).or_insert((i + 1) as i32);
    }

    match hmap.get(&c) {
        None => 0,
        Some(x) => *x
    }
}

#[cfg(test)]
mod tests {
    use crate::problem1;
    use crate::problem2;

    #[test]
    fn test_problem1() {
        let input = include_str!("./day3/test_input.txt");

        let res = problem1(input);

        assert_eq!(res, 157);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("./day3/test_input.txt");

        let res = problem2(input);

        assert_eq!(res, 70);
    }
}
