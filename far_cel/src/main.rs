use std::io;

fn main() {
    println!("What convertion you want?:\n1. far->cel\n2. cel->far\nEnter a number:");

    let result: i32;
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error");
        result = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let temp = match result {
            1 => convert_far_to_cel(result),
            2 => convert_cel_to_far(result),
            _ => continue,
        };

        match result {
            1 => println!("{}"),
            2 => convert_cel_to_far(result),
            _ => break,
        }
        break;
    }
}

fn convert_far_to_cel(far: i32) -> i32 {
    ((far - 32) * 5) / 9
}

fn convert_cel_to_far(cel: i32) -> i32 {
    cel * 9 / 5 + 32
}
