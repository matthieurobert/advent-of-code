fn main() {
    let input = include_str!("./day4/input.txt");

    let res1 = problem1(input);
    let res2 = problem2(input);

    println!("Problem 1 : {}", res1);
    println!("Problem 2 : {}", res2);
}

pub fn problem1(input: &str) -> i32 {
    let parsed = input.split("\n");
    let mut sum = 0;

    for e in parsed {
        if e != "" {
            let _line: Vec<i32> = e.split(|c| c == ',' || c == '-').map(|x| x.parse::<i32>().unwrap()).collect();

            if _line.get(0) <= _line.get(2) && _line.get(1) >= _line.get(3) {
                sum += 1;
            } else if _line.get(2) <= _line.get(0) && _line.get(3) >= _line.get(1) {
                sum += 1;
            }
        }
    }

    sum
}

pub fn problem2(input: &str) -> i32 {
    let parsed = input.split("\n");
    let mut sum = 0;

    for e in parsed {
        if e != "" {
            let _line: Vec<i32> = e.split(|c| c == ',' || c == '-').map(|x| x.parse::<i32>().unwrap()).collect();

            if _line.get(0) <= _line.get(3) && _line.get(1) >= _line.get(2) {
                sum += 1;
            } else if _line.get(2) <= _line.get(1) && _line.get(3) >= _line.get(0) {
                sum += 1;
            }
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use crate::problem1;
    use crate::problem2;

    #[test]
    fn test_problem1() {
        let input = include_str!("./day4/test_input.txt");

        let res = problem1(input);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("./day4/test_input.txt");

        let res = problem2(input);

        assert_eq!(res, 4);
    }
}
