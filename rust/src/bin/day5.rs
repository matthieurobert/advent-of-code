fn main() {
    let input = include_str!("./day5/input.txt");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let stack1: Vec<char> = vec!['B', 'V', 'S', 'N', 'T', 'C', 'H', 'Q'];
    let stack2: Vec<char> = vec!['W', 'D', 'B', 'G'];
    let stack3: Vec<char> = vec!['F', 'W', 'R', 'T', 'S', 'Q', 'B'];
    let stack4: Vec<char> = vec!['L', 'G', 'W', 'S', 'Z', 'J', 'D', 'N'];
    let stack5: Vec<char> = vec!['M', 'P', 'D', 'V', 'F'];
    let stack6: Vec<char> = vec!['F', 'W', 'J'];
    let stack7: Vec<char> = vec!['L', 'N', 'Q', 'B', 'J', 'V'];
    let stack8: Vec<char> = vec!['G', 'T', 'R', 'C', 'J', 'Q', 'S', 'N'];
    let stack9: Vec<char> = vec!['J', 'S', 'Q', 'C', 'W', 'D', 'M'];

    stacks.push(stack1);
    stacks.push(stack2);
    stacks.push(stack3);
    stacks.push(stack4);
    stacks.push(stack5);
    stacks.push(stack6);
    stacks.push(stack7);
    stacks.push(stack8);
    stacks.push(stack9);

    let res1 = problem1(input, stacks.clone());
    let res2 = problem2(input, stacks.clone());

    println!("Problem 1 : {}", res1);
    println!("Problem 2 : {}", res2);
}

pub fn problem1(input: &str,mut stacks: Vec<Vec<char>>) -> String {
    let parsed = input.split("\n");

    for e in parsed {
        //println!("{e}");
        if e != "" {
            let test: Vec<&str> = e.split(' ').collect();

            let _line: Vec<i32> = vec![test[1].parse::<i32>().unwrap(), test[3].parse::<i32>().unwrap(), test[5].parse::<i32>().unwrap()];
            
            for _ in 0.._line[0] {
                let tmp = stacks[(_line[1] - 1) as usize].pop().unwrap();
                stacks[(_line[2]  - 1) as usize].push(tmp);
            }
        }
    }

    let mut res = String::from("");

    for i in 0..stacks.len() {
        res.push(stacks[i].pop().unwrap());
    }

    res
}

pub fn problem2(input: &str, mut stacks: Vec<Vec<char>>) -> String {
    let parsed = input.split("\n");

    for e in parsed {
        if e != "" {
            let test: Vec<&str> = e.split(' ').collect();

            let _line: Vec<i32> = vec![test[1].parse::<i32>().unwrap(), test[3].parse::<i32>().unwrap(), test[5].parse::<i32>().unwrap()];

            let mut tmp = String::from("");

            for _ in 0.._line[0] {
                tmp.push(stacks[(_line[1] - 1) as usize].pop().unwrap());
            }

            for _ in 0.._line[0] {
                stacks[(_line[2] - 1) as usize].push(tmp.pop().unwrap());
            }
        }
    }
    
    let mut res = String::from("");

    for i in 0..stacks.len() {
        res.push(stacks[i].pop().unwrap());
    }

    res
}


#[cfg(test)]
mod tests {
    use crate::problem1;
    use crate::problem2;

    #[test]
    fn test_problem1() {
        let input = include_str!("./day5/test_input.txt");
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stack1: Vec<char> = vec!['Z', 'N'];
        
        let stack2: Vec<char> = vec!['M', 'C', 'D'];

        let stack3: Vec<char> = vec!['P'];

        stacks.push(stack1);
        stacks.push(stack2);
        stacks.push(stack3);

        let res = problem1(input, stacks.clone());

        assert_eq!(res, "CMZ");
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("./day5/test_input.txt");
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let stack1: Vec<char> = vec!['Z', 'N'];
        
        let stack2: Vec<char> = vec!['M', 'C', 'D'];

        let stack3: Vec<char> = vec!['P'];

        stacks.push(stack1);
        stacks.push(stack2);
        stacks.push(stack3);

        let res = problem2(input, stacks.clone());

        assert_eq!(res, "MCD");
    }
}
