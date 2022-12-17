use regex::Regex;

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

#[derive(Debug, PartialEq)]
enum ListEntry {
    List(Vec<ListEntry>),
    Value(u64),
}

fn parse_list(captures: &mut Vec<String>) -> ListEntry {

    let mut new_list = ListEntry::List(vec![]);
    loop {
        let c = captures.remove(0);
        let open = String::from("[");
        let close = String::from("]");

        if open.eq(&c) {
            let content = match new_list {
                ListEntry::List(ref mut content) => content,
                _ => unreachable!(),
            };
            content.push(parse_list(captures));
        } else if close.eq(&c) {
            return new_list;
        } else {
            let content = match new_list {
                ListEntry::List(ref mut content) => content,
                _ => unreachable!(),
            };
            content.push(ListEntry::Value(c.parse::<u64>().unwrap()));
        }
    }
}

fn compare_list_entries(entry_1: &ListEntry, entry_2: &ListEntry) -> Option<bool> {

    match (entry_1, entry_2) {
        (ListEntry::Value(v1), ListEntry::Value(v2)) => {
            if v1 == v2 {
                return None;
            } else {
                return Some(v1 < v2);
            }
        },
        (ListEntry::List(l1), ListEntry::List(l2)) => {
            let mut idx: usize = 0;
            loop {
                if l1.len() >= idx + 1 && l2.len() >= idx +1 {
                    let item_1 = &l1[idx];
                    let item_2 = &l2[idx];
                    match compare_list_entries(&item_1, &item_2) {
                        Some(x) => return Some(x),
                        None => (),
                    }
                } else if l1.len() == l2.len() {
                    break;
                } else {
                    return Some(l1.len() < l2.len());
                }
                idx += 1;
            } 
            return None;          
        },
        (ListEntry::List(_l1), ListEntry::Value(v2)) => {
            let l2 = ListEntry::List(vec![ListEntry::Value(*v2)]);    
            return compare_list_entries(&entry_1, &l2);
        },
        (ListEntry::Value(v1), ListEntry::List(_l2)) => {
            let l1 = ListEntry::List(vec![ListEntry::Value(*v1)]);
            return compare_list_entries(&l1, &entry_2);
        },
    }
}


fn part_1<R: Read>(io: R) -> Result<usize, Error> {

    let mut running_total: usize = 0;

    let br = BufReader::new(io);

    let mut line_iter = br.lines();
    let mut i: usize = 1;

    let extraction_re = Regex::new(r"(\d+|\[{1}|]{1})").unwrap();

    while let (Some(l1), Some(l2), Some(_l3)) = (line_iter.next(), line_iter.next(), line_iter.next()) {        
        let l1_unwrapped = &l1.unwrap();
        let l2_unwrapped = &l2.unwrap();
        let captures_1 = extraction_re.captures_iter(l1_unwrapped);
        let captures_2 = extraction_re.captures_iter(l2_unwrapped);

        let mut c1: Vec<String> = vec![];
        for cap in captures_1 {
            c1.push(cap[1].to_string());
        }
        let mut c2: Vec<String> = vec![];
        for cap in captures_2 {
            c2.push(cap[1].to_string());
        }
        c1.remove(0);
        c2.remove(0);
        let list_1 = parse_list(&mut c1);
        let list_2 = parse_list(&mut c2);
         if compare_list_entries(&list_1, &list_2).unwrap() {
             running_total += i;
        }
        i += 1;
    }
    Ok(running_total)
}

fn part_2<R: Read>(io: R) -> Result<usize, Error> {
    let mut lists: Vec<ListEntry> = vec![];


    lists.push(ListEntry::List(vec![ListEntry::List(vec![ListEntry::Value(2)])]));
    lists.push(ListEntry::List(vec![ListEntry::List(vec![ListEntry::Value(6)])]));

    let extraction_re = Regex::new(r"(\d+|\[{1}|]{1})").unwrap();

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = &line.unwrap();
        if l.len() > 0 {
            let captures = extraction_re.captures_iter(l);
            let mut c: Vec<String> = vec![];
            for cap in captures {
                c.push(cap[1].to_string());
            }
            c.remove(0);
            lists.push(parse_list(&mut c));
        }
    }

    fn order_map(input: bool) -> Ordering {
        match input {
            true => {
                Ordering::Less
            },
            false => {
                Ordering::Greater
            },
        }
    }

    lists.sort_by(|a, b| order_map(compare_list_entries(&a, &b).unwrap()));

    let mut product: usize = 1;

    let divider_2 = ListEntry::List(vec![ListEntry::List(vec![ListEntry::Value(2)])]);
    let divider_6 = ListEntry::List(vec![ListEntry::List(vec![ListEntry::Value(6)])]);

    for (i, list) in lists.iter().enumerate() {
        if *list == divider_2 || *list == divider_6 {
            product *= i+1;
        }
    }

    Ok(product)
}

fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
}
