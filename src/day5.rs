pub fn day5(orderings: &str, updates: &str) {
    let mut order_table = Vec::new();
    for line in orderings.lines() {
        let mut splt = line.split('|');
        let before: usize = splt.next().unwrap().parse().unwrap();
        let after: usize = splt.next().unwrap().parse().unwrap();
        while order_table.len() <= before {
            order_table.push(Vec::new());
        }
        order_table[before].push(after);
    }
    let order_table = order_table;

    let mut correct_acum = 0;
    let mut incorrect_acum = 0;

    let mut update: Vec<usize> = Vec::new();
    for line in updates.lines() {
        update.clear();
        for page in line.split(',') {
            update.push(page.parse().unwrap());
        }
        let mut correct = true;
        loop {
            let mut swapped = false;
            for i in 1..update.len() {
                let req = &order_table[update[i]];
                for j in 0..i {
                    if req.contains(&update[j]) {
                        correct = false;
                        swapped = true;
                        let e = update.remove(j);
                        update.insert(i, e);
                    }
                }
            }
            if !swapped {
                break;
            }
        }
        if correct {
            correct_acum += update[update.len() / 2];
        } else {
            incorrect_acum += update[update.len() / 2];
        }
    }

    println!("correct: {correct_acum}");
    println!("incorrect: {incorrect_acum}");
}

#[test]
fn test0() {
    day5(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
        "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
}
