mod math_fns;

fn main() {
    use math_fns::{median, mode};

    let integers = vec![1, 2, 3, 3, 3, 4, 5, 6, 6, 7];

    match median(&integers) {
        Some(med) => println!("The median is: {med}"),
        None => println!("The list was empty. No median can be found."),
    }

    match mode(&integers) {
        Some(mode) => println!("The mode is: {mode}"),
        None => println!("The list was empty. No mode can be found."),
    }
}
