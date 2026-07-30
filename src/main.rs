mod math_fns;
mod string_fns;

fn main() {
    use math_fns::{median, mode};
    use string_fns::pig_latin;

    let integers = vec![1, 2, 3, 3, 3, 4, 5, 6, 6, 7];

    match median(&integers) {
        Some(med) => println!("The median is: {med}"),
        None => println!("The list was empty. No median can be found."),
    }

    match mode(&integers) {
        Some(mode) => println!("The mode is: {mode}"),
        None => println!("The list was empty. No mode can be found."),
    }

    let test_str = String::from("The pig eats the apple.");
    println!(
        "In pig latin, the sentence, '{test_str}', becomes: {}",
        pig_latin(&test_str)
    )
}
