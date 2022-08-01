fn main() {
    (1..=100).map(process_num)
        .for_each(|x| println!("{}", x));
}

fn process_num(num: u32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => String::from("CracklePop"),
        (0, _) => String::from("Crackle"),
        (_, 0) => String::from("Pop"),
        _ => num.to_string(),
    }
}