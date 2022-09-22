use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq + Display + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let string_first_list = _first_list
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let string_second_list = _second_list
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();

    if string_first_list.contains(&string_second_list) {
        return Comparison::Superlist;
    }

    if string_second_list.contains(&string_first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

pub fn main() -> () {
    println!("SUBLIST: {:#?}", sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    println!("SUBLIST: {:#?}", sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    println!("SUBLIST: {:#?}", sublist(&[3, 4], &[1, 2, 3, 4, 5]));
    println!("EQUAL: {:#?}", sublist(&[1, 2, 3], &[1, 2, 3]));
    println!("SUPERLIST: {:#?}", sublist(&[1, 2, 3, 4, 5], &[2, 3, 4]));
    println!("UNEQUAL: {:#?}", sublist(&[1, 2, 4], &[1, 2, 3, 4, 5]));
}
