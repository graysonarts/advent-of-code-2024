use std::collections::HashMap;

fn part_one() {
    let (mut left_list, mut right_list) = get_lists();

    let sum: u32 = left_list
        .into_iter()
        .zip(right_list.into_iter())
        .map(|(l, r)| {
            let diff = l.abs_diff(r);
            println!("{} - {} = {}", l, r, diff);
            diff
        })
        .sum();

    println!("Sum: {}", sum);
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let data = include_str!("./input.txt");
    let lists = data
        .split_terminator('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut left_list: Vec<_> = lists.iter().map(|s| *s.get(0).unwrap()).collect();
    let mut right_list: Vec<_> = lists.into_iter().map(|s| *s.get(1).unwrap()).collect();

    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn main() {
    let (left_list, right_list) = get_lists();
    let lookup = right_list
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut acc, x| {
            if acc.contains_key(&x) {
                let count = acc.get_mut(&x).unwrap();
                *count += 1;
            } else {
                acc.insert(x, 1);
            }
            acc
        });
    let sum = left_list
        .into_iter()
        .map(|x| x * lookup.get(&x).map(|x| *x).unwrap_or_default())
        .sum::<i32>();

    println!("Similarity score: {}", sum);
}
