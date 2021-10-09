use std::vec::Vec;

pub fn number_to_name(i: &str) -> String {
    let veci = i.bytes().rev().collect::<Vec<u8>>();
    let dpp = veci.iter().position(|item| *item == '.' as u8);
    if let Some(d) = dpp {
        if let Some(result) = decimal(&veci[..d]) {
            return format!("{} point {}", sub_ntn(&veci[d+1..], 0, 0), result);
        } else {
            return format!("{}", sub_ntn(&veci[d+1..], 0, 0));
        }
    }
    format!("{}", sub_ntn(&veci[..], 0, 0))
}
pub fn number_to_name_loop(i: &str) -> String {
    let veci = i.bytes().rev().collect::<Vec<u8>>();
    let dpp = veci.iter().position(|item| *item == '.' as u8);
    if let Some(d) = dpp {
        if let Some(result) = decimal(&veci[..d]) {
            return format!("{} point {}", sub_ntn_loop(&veci[d+1..], 0), result);
        } else {
            return format!("{}", sub_ntn_loop(&veci[d+1..], 0));
        }
    }
    format!("{}", sub_ntn_loop(&veci[..], 0))
}

fn make_name(i: u16) -> String {
    match i {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        100..=999 => {
            let first_two_place = i % 100;
            if first_two_place == 0 {
                format!("{} hundred", make_name(i / 100))
            } else {
                format!("{} hundred and {}", make_name(i / 100), make_name(first_two_place))
            }
        }
        _ => {
            let one_place = i%10;
            format!("{}-{}", make_name(i - one_place), make_name(one_place))
        }
    }
}

const GROUP_NAME: [&str; 5] = ["", "thousand", "milion", "bilion", "trilliion"]; 
const CODE8: u8 = '0' as u8;

//recursion
fn sub_ntn(i: &[u8], pointer: usize, group: usize) -> String {
    let group_name = GROUP_NAME.get(group).unwrap_or(&GROUP_NAME[0]);

    if group == GROUP_NAME.len() - 1 {
        return format!("{} {}", sub_ntn(i, pointer, 0), group_name);
    }
    let current = parse(i, pointer);

    if has_more(i, pointer) {
        if group == 0 {
            make_name(current)
        } else {
            format!("{} {}", make_name(current), group_name)
        }
    } else if current == 0 {
        sub_ntn(i, pointer + 3, group + 1)
    } else if group == 0 {
        if !has_more(i, pointer) && current < 100 {
            format!("{} and {}", sub_ntn(i, pointer + 3, group + 1), make_name(current))
        } else {
            format!("{}, {}", sub_ntn(i, pointer + 3, group + 1), make_name(current))
        }
    } else {
        format!("{}, {} {}", sub_ntn(i, pointer + 3, group + 1), make_name(current), group_name)
    }
}

//
fn sub_ntn_loop(i: &[u8], pointer: usize) -> String {
    let current = parse(i, pointer);

    if has_more(i, pointer) {
        return make_name(current);
    }

    let mut builder = if current == 0 {
            String::new()
        } else if current < 100 {
            format!(" and {}", make_name(current))
        } else {
            format!(", {}", make_name(current))
        };

    let mut group = 1;
    let mut pointer = pointer + 3;
    while group < GROUP_NAME.len() - 1 {
        let current = parse(i, pointer);
        let group_name = GROUP_NAME.get(group).unwrap_or(&GROUP_NAME[0]);

        if has_more(i, pointer) {
            builder = format!("{} {}{}", make_name(current), group_name, builder);
            break;
        } else if current != 0 {
            builder = format!(", {} {}{}", make_name(current), group_name, builder);
        }

        group += 1; //maker sure it end with current group
        pointer += 3; //make sure it break with current pointer
    }
    
    if group == GROUP_NAME.len() - 1 {
        return format!("{} {}{}", sub_ntn_loop(i, pointer), GROUP_NAME.last().unwrap(), builder);
    }

    builder
}

fn decimal(i: &[u8]) -> Option<String> {
    let mut result = String::new();

    let mut pointer = 0;

    //consume zeros in the end
    while pointer < i.len() && i[pointer] == CODE8 {
        pointer += 1;
    }

    if pointer == i.len() {
        None
    } else {
        i.iter().step_by(pointer).for_each(|item| result = format!("{} {}", make_name((*item as u8 - CODE8).into()), result));
        Some(result)
    }
}

#[inline]
fn parse(i: &[u8], pointer: usize) -> u16 {
    (*i.get(pointer + 2).unwrap_or(&CODE8) - CODE8) as u16 * 100u16 + 
        (*i.get(pointer + 1).unwrap_or(&CODE8) - CODE8) as u16 * 10u16 + 
        (i[pointer] - CODE8) as u16
}

#[inline]
//has nore group after the group whith current pointer at
fn has_more(i: &[u8], pointer: usize) -> bool {
    pointer + 4 > i.len()
}