use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

use num;

#[derive(Default, Debug)]
struct Monkey {
    items: Vec<u64>,
    test_divisor: u64,
    operator: char,
    op_value: u64,
    true_target: usize,
    false_target: usize,
    inspected: u64,
}

impl Monkey {
    fn inspect_items_1(&mut self) -> Vec<(usize, u64)> {
        let mut thrown_items: Vec<(usize, u64)> = vec![];
        for item in self.items.iter() {
            self.inspected += 1;
            let new_value = Monkey::operate_on(self.operator, self.op_value, *item) / 3;
            let target: usize;
            match new_value % self.test_divisor {
                0 => target = self.true_target,
                _ => target = self.false_target,
            } 

            thrown_items.push((target, new_value));
        }
        self.items = vec![];
        thrown_items
    }
    
    fn inspect_items_2(&mut self) -> Vec<(usize, u64)> {
        let mut thrown_items: Vec<(usize, u64)> = vec![];
        for item in self.items.iter() {
            self.inspected += 1;
            let new_value = Monkey::operate_on(self.operator, self.op_value, *item);
            let target: usize;
            match new_value % self.test_divisor {
                0 => target = self.true_target,
                _ => target = self.false_target,
            } 

            thrown_items.push((target, new_value));
        }
        self.items = vec![];
        thrown_items
    }

    fn operate_on(operator: char, op_value: u64, item: u64) -> u64 {
        match operator {
            '+' => return item + op_value,
            '*' => return item * op_value,
            '^' => return item * item,
            _   => unreachable!()
        }
    }  
}


fn part_1<R: Read>(io: R) -> Result<u64, Error> {

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_idx: usize = 0;

    let br = BufReader::new(io);
    for (idx, line) in br.lines().enumerate() {
        let l = line.unwrap();
        let l_chars: Vec<char> = l.chars().collect();
        match idx % 7 {
            0 => {monkeys.push(Default::default());
                  monkey_idx = usize::try_from(l_chars[7].to_digit(10).unwrap()).unwrap();
                 },
            1 => monkeys[monkey_idx].items = l[18..].split(", ").collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect::<Vec<u64>>(),
            2 => {match l_chars[25] {
                    'o' => {monkeys[monkey_idx].operator = '^';
                            monkeys[monkey_idx].op_value = 2;
                           },
                    _   => {monkeys[monkey_idx].operator = l_chars[23];
                            monkeys[monkey_idx].op_value = l[25..].parse().unwrap();
                           }
                    }
                 },
            3 => monkeys[monkey_idx].test_divisor = l[21..].parse().unwrap(),
            4 => monkeys[monkey_idx].true_target = l[29..].parse().unwrap(),
            5 => monkeys[monkey_idx].false_target = l[30..].parse().unwrap(),
            6 => (),
            _ => unreachable!(),
        }
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let thrown_items = monkeys[idx].inspect_items_1();
            for item in thrown_items.iter() {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    let mut inspected: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();
    inspected.reverse();

    Ok(inspected[0] * inspected[1])

}


fn part_2<R: Read>(io: R) -> Result<u64, Error> {

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_idx: usize = 0;

    let br = BufReader::new(io);
    for (idx, line) in br.lines().enumerate() {
        let l = line.unwrap();
        let l_chars: Vec<char> = l.chars().collect();
        match idx % 7 {
            0 => {monkeys.push(Default::default());
                  monkey_idx = usize::try_from(l_chars[7].to_digit(10).unwrap()).unwrap();
                 },
            1 => monkeys[monkey_idx].items = l[18..].split(", ").collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect::<Vec<u64>>(),
            2 => {match l_chars[25] {
                    'o' => {monkeys[monkey_idx].operator = '^';
                            monkeys[monkey_idx].op_value = 2;
                           },
                    _   => {monkeys[monkey_idx].operator = l_chars[23];
                            monkeys[monkey_idx].op_value = l[25..].parse().unwrap();
                           }
                    }
                 },
            3 => monkeys[monkey_idx].test_divisor = l[21..].parse().unwrap(),
            4 => monkeys[monkey_idx].true_target = l[29..].parse().unwrap(),
            5 => monkeys[monkey_idx].false_target = l[30..].parse().unwrap(),
            6 => (),
            _ => unreachable!(),
        }
    }

    let mut lcm: u64 = 1;
    for monkey in monkeys.iter() {
        lcm = num::integer::lcm(lcm, monkey.test_divisor)
    }

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let thrown_items = monkeys[idx].inspect_items_2();
            for item in thrown_items.iter() {
                monkeys[item.0].items.push(item.1 % lcm);
            }
        }
    }

    let mut inspected: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();
    inspected.reverse();

    Ok(inspected[0] * inspected[1])

}


fn main() -> Result<(), Error> {

    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
    Ok(())
}