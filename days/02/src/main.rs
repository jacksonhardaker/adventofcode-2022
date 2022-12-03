use std::fs;

fn result(i: (&str, &str)) -> i32 {
    if (i.0.eq("A") && i.1.eq("X")) || (i.0.eq("B") && i.1.eq("Y")) || (i.0.eq("C") && i.1.eq("Z"))
    {
        return 3;
    }

    if (i.0.eq("A") && i.1.eq("Y")) || (i.0.eq("B") && i.1.eq("Z")) || (i.0.eq("C") && i.1.eq("X"))
    {
        return 6;
    }

    return 0;
}

fn get_score(m: &str) -> i32 {
    if m.eq("X") {
        return 1;
    }
    if m.eq("Y") {
        return 2;
    }
    if m.eq("Z") {
        return 3;
    }

    return 0;
}

fn get_winning_move(m: &str) -> i32 {
    if m.eq("A") {
        return get_score("Y");
    }
    if m.eq("B") {
        return get_score("Z");
    }
    if m.eq("C") {
        return get_score("X");
    }

    return 0;
}
fn get_losing_move(m: &str) -> i32 {
    if m.eq("A") {
        return get_score("Z");
    }
    if m.eq("B") {
        return get_score("X");
    }
    if m.eq("C") {
        return get_score("Y");
    }

    return 0;
}
fn get_drawing_move(m: &str) -> i32 {
    if m.eq("A") {
        return get_score("X");
    }
    if m.eq("B") {
        return get_score("Y");
    }
    if m.eq("C") {
        return get_score("Z");
    }

    return 0;
}

fn main() {
    let data = fs::read_to_string("./days/02/input.txt").expect("Error!");
    let split_data: Vec<i32> = data
        .trim()
        .split("\n")
        .map(|line| {
            let mut split_line = line.split(" ");
            let instructions = (split_line.next().unwrap(), split_line.next().unwrap());
            return get_score(instructions.1) + result(instructions);
        })
        .collect();

    // Part 1: 14827
    let s = split_data.iter().fold(0u64, |sum, i| sum + (*i as u64));
    println!("Part 1: {}", s);

    let part_2_data = data.clone();
    let part_2_split_data: Vec<i32> = part_2_data
        .trim()
        .split("\n")
        .map(|line| {
            let mut split_line = line.split(" ");
            let instructions = (split_line.next().unwrap(), split_line.next().unwrap());

            let mut result = 0;

            if instructions.1 == "X" {
                // lose
                result = 0 + get_losing_move(instructions.0);
            } else if instructions.1 == "Y" {
                // draw
                result = 3 + get_drawing_move(instructions.0);
            } else if instructions.1 == "Z" {
                // win
                result = 6 + get_winning_move(instructions.0);
            }

            return result;
        })
        .collect();

    // Part 2: 13889
    let s_2 = part_2_split_data
        .iter()
        .fold(0u64, |sum, i| sum + (*i as u64));
    println!("Part 2: {}", s_2);
}
