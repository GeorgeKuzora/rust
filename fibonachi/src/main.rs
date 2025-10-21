use std::io;

fn main() {
    let position: u32;
    loop {
        let mut input = String::new();
        println!("Write a fibonacci position: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        position = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    };
    let fibonacci_num = fibonachi(position);
    println!("position = {position}, fibonacci number = {fibonacci_num}")
}

fn fibonachi(position: u32) -> u32 {
    match position {
        0 => 0,
        1 => 1,
        _ => find_fibonachi(position),
    }
}

fn find_fibonachi(position: u32) -> u32 {
    let shifted_pos = position;
    let mut prev_num = 0;
    let mut current_num = 1;
    let mut temp_num;

    for _ in 1..shifted_pos {
        temp_num = current_num;
        current_num = current_num + prev_num;
        prev_num = temp_num;
    }
    current_num
}
