#[derive(Debug, PartialEq)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let sublist = _first_list.is_empty()
        || _second_list
            .windows(_first_list.len())
            .any(|x| x == _first_list);

    let superlist = _second_list.is_empty()
        || _first_list
            .windows(_second_list.len())
            .any(|x| x == _second_list);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (false, true) => Comparison::Superlist,
        (true, false) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

pub fn main() -> () {
    println!("SUBLIST: {:#?}", sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    println!("SUBLIST: {:#?}", sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    println!("SUBLIST: {:#?}", sublist(&[3, 4], &[1, 2, 3, 4, 5]));
    println!("EQUAL: {:#?}", sublist(&[1, 2, 3], &[1, 2, 3]));
    println!("SUPERLIST: {:#?}", sublist(&[1, 2, 3, 4, 5], &[2, 3, 4]));
    println!("UNEQUAL: {:#?}", sublist(&[1, 2, 4], &[1, 2, 3, 4, 5]));
}
