use std::collections::HashMap;
use std::io;

fn part1() {
    let mut buffer = String::new();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let smth: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        left.push(smth[0]);
        right.push(smth[1]);
        buffer.clear();
    }
    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| (l - r).abs() + acc);

    println!("{}", sum);
}

fn part2() {
    let mut buffer = String::new();
    let mut left: Vec<i32> = Vec::new();
    let mut right_counter: HashMap<i32, i32> = HashMap::new();

    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let smth: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        left.push(smth[0]);
        match right_counter.get_mut(&smth[1]) {
            Some(value) => *value += 1,
            None => {
                right_counter.insert(smth[1], 1);
            }
        }
        buffer.clear();
    }
    println!("{:?}", right_counter);

    let sum = left
        .into_iter()
        .fold(0, |acc, x| match right_counter.get(&x) {
            Some(value) => acc + x * value,
            None => acc,
        });

    println!("{}", sum);
}
fn main() {
    part1();
    part2();
}
