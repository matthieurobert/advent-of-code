fn main() {
    let file = include_str!("./day1/input.txt");

    let value = problem_one(file);
    let val2 = problem_two(file);

    println!("max score {}", value);

    println!("sum top 3 {val2}")
}

pub fn problem_one(input: &str) -> i32 {
    let parsed = input.split("\n");
    let mut score = 0;
    let mut scores = Vec::new();

    for e in parsed{
        if e == "" {
            scores.push(score);
            score = 0;
        } else {
            score += e.parse::<i32>().unwrap();
        }
    }

    scores.push(score);

    return max(scores)
    
}

pub fn problem_two(input: &str) -> i32 {
    let parsed = input.split("\n");
    let mut score = 0;
    let mut scores = Vec::new();

    for e in parsed{
        if e == "" {
            scores.push(score);
            score = 0;
        } else {
            score += e.parse::<i32>().unwrap();
        }
    }

    scores.push(score);

    scores.sort();

    let mut val = scores.pop().unwrap();
    val += scores.pop().unwrap();
    val + scores.pop().unwrap()
}



fn max(array: Vec<i32>) -> i32 {
    let mut max = 0;

    for val in array {
        if val > max {
            max = val;
        }
    }

    return max;
}

#[cfg(test)]
mod tests {
    use crate::problem_one;
    use crate::problem_two;

    #[test]
    fn test_problem_one() {
        let input = include_str!("./day1/test_input.txt");

        let score = problem_one(input);

        assert_eq!(score, 24000)
    }

    #[test]
    fn test_problem_two() {
        let input = include_str!("./day1/test_input.txt");

        let top3_score = problem_two(input);

        assert_eq!(top3_score, 45000)
    }
}
