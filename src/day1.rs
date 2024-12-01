use std::collections::HashMap;

pub fn day1(input: &str) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let mut hashmap1: HashMap<i32, i32> = HashMap::new();
    let mut hashmap2: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut line_tokens = line.split_ascii_whitespace();
        let a: i32 = line_tokens.next().unwrap().parse().unwrap();
        let b: i32 = line_tokens.next().unwrap().parse().unwrap();

        list1.push(a);
        list2.push(b);

        match hashmap1.get_mut(&a) {
            Some(c) => {
                *c = *c + 1;
            }
            None => {
                hashmap1.insert(a, 1);
            }
        };
        match hashmap2.get_mut(&b) {
            Some(c) => {
                *c = *c + 1;
            }
            None => {
                hashmap2.insert(b, 1);
            }
        };
    }

    list1.sort();
    list2.sort();

    let mut acum = 0;
    for i in 0..list1.len() {
        acum += (list1[i] - list2[i]).abs();
    }

    let mut similarity = 0;
    for (key, value) in hashmap1 {
        if let Some(c) = hashmap2.get(&key) {
            similarity += key * value * *c;
        }
    }

    println!("{acum}");
    println!("{similarity}");
}

#[test]
fn test0() {
    day1(
        "3   4
4   3
2   5
1   3
3   9
3   3",
    );
}
