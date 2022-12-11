use regex::Regex;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    _name: String,
    items: VecDeque<u128>,
    operation_modifier: String,
    operation_operator: String,
    test_divisor: u128,
    truthy_monkey_index: usize,
    falsy_monkey_index: usize,
    inspect_count: usize,
    relief_modifier: u128,
}

impl Monkey {
    fn new(
        items: VecDeque<u128>,
        op_args: (&str, &str),
        test_divisor: u128,
        truthy_monkey_index: usize,
        falsy_monkey_index: usize,
        index: usize,
        relief_modifier: u128,
    ) -> Monkey {
        Monkey {
            _name: format!("Monkey {}", index),
            items: items,
            operation_modifier: op_args.1.to_string(),
            operation_operator: op_args.0.to_string(),
            test_divisor: test_divisor,
            truthy_monkey_index: truthy_monkey_index,
            falsy_monkey_index: falsy_monkey_index,
            inspect_count: 0,
            relief_modifier: relief_modifier,
        }
    }

    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    fn from(index: usize, monkey_string: &str, relief_modifier: u128) -> Monkey {
        let items: VecDeque<u128> = Regex::new(r"Starting items:(.*)")
            .unwrap()
            .captures(monkey_string)
            .unwrap()[1]
            .split(",")
            .map(|part| part.trim().parse::<u128>().unwrap())
            .collect();

        let op_cap = Regex::new(r"Operation: new = old (\*|\+) (old|\d+)")
            .unwrap()
            .captures(monkey_string)
            .unwrap();

        let test_divisor: u128 = Regex::new(r"Test: divisible by (\d+)")
            .unwrap()
            .captures(monkey_string)
            .unwrap()[1]
            .parse::<u128>()
            .unwrap();

        let truthy_monkey_index: usize = Regex::new(r"If true: throw to monkey (\d+)")
            .unwrap()
            .captures(monkey_string)
            .unwrap()[1]
            .parse::<usize>()
            .unwrap();
        let falsy_monkey_index: usize = Regex::new(r"If false: throw to monkey (\d+)")
            .unwrap()
            .captures(monkey_string)
            .unwrap()[1]
            .parse::<usize>()
            .unwrap();

        Monkey::new(
            items,
            (&op_cap[1], &op_cap[2]),
            test_divisor,
            truthy_monkey_index,
            falsy_monkey_index,
            index,
            relief_modifier,
        )
    }

    fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    fn inspect_first_item(&mut self, common_modulo: u128) -> (u128, usize) {
        if self.has_items() {
            self.inspect_count += 1;
            let mut item = self.items.pop_front().unwrap();
            item = self.operation(item, common_modulo);

            if self.test(item) {
                // throw truthy
                (item, self.truthy_monkey_index)
            } else {
                //throw falsy
                (item, self.falsy_monkey_index)
            }
        } else {
            (0, 0)
        }
    }

    fn catch_item(&mut self, item: u128) {
        self.items.push_back(item);
    }

    fn operation(&mut self, item: u128, common_modulo: u128) -> u128 {
        let mut val = item;
        let modifier = if self.operation_modifier == "old" {
            val
        } else {
            self.operation_modifier.parse::<u128>().unwrap()
        };

        if self.operation_operator == "+" {
            val = val + modifier
        } else if self.operation_operator == "*" {
            val = val * modifier
        }

        if self.relief_modifier > 1 {
            val = val / self.relief_modifier
        } else {
            val = val % common_modulo
        }

        val
    }

    fn test(&self, val: u128) -> bool {
        val % self.test_divisor == 0
    }
}

fn calc_monkey_business(input: String, relief_modifier: u128, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(index, monkey_string)| Monkey::from(index, monkey_string, relief_modifier))
        .collect();
    let len = monkeys.len();

    let common_modulo = if rounds > 20 {
        monkeys
            .iter()
            .fold(1, |common, monkey| common * monkey.test_divisor)
    } else {
        0
    };

    for _ in 1..=rounds {
        for index in 0..len {
            let current_monkey = &mut monkeys[index];
            let mut throw_buff: VecDeque<(u128, usize)> = VecDeque::new();

            while current_monkey.has_items() {
                throw_buff.push_back(current_monkey.inspect_first_item(common_modulo));
            }

            throw_buff.iter().for_each(|throw| {
                // (0,0) represents a noop
                if !(throw.0 == 0 && throw.1 == 0) {
                    // println!("throwing {} to monkey {}", throw.0, throw.1);
                    monkeys[throw.1].catch_item(throw.0);
                }
            });
        }
    }

    let mut inspect_count = monkeys
        .iter()
        .map(|monkey| monkey.inspect_count)
        .collect::<Vec<usize>>();
    inspect_count.sort();

    let monkey_business = inspect_count.as_slice()[inspect_count.len() - 2..]
        .to_vec()
        .iter()
        .fold(1, |t, c| t * c);

    monkey_business
}

fn part1(input: String) -> usize {
    calc_monkey_business(input, 3, 20)
}

fn part2(input: String) -> usize {
    calc_monkey_business(input, 1, 10000)
}

fn main() {
    let input = fs::read_to_string("./days/11/input.txt").expect("Error!");

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[test]
fn test_part1() {
    let input = fs::read_to_string("./test-input.txt").expect("Error!");
    assert_eq!(part1(input), 10605);
}
#[test]
fn test_part2() {
    let input = fs::read_to_string("./test-input.txt").expect("Error!");
    assert_eq!(part2(input), 2713310158);
}
