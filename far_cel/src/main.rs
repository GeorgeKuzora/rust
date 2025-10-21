use std::io;

fn main() {

    println!("What convertion you want?:\n1. far->cel\n2. cel->far\nEnter a number:");
    let flow: u32;
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error");
        flow = match buf.trim().parse() {
            Ok(num) => {
                match num {
                    1 => 1,
                    2 => 2,
                    _ => {
                        println!("Enter a valid number");
                        continue;
                    },
                }
            },
            Err(_) => {
                println!("Enter a valid number");
                continue;
            },
        };
        break;
    }

    println!("Write a value to convert:");
    let value: i32;
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error");
        value = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            },
        };
        break;
    }
    let temp = match flow {
        1 => convert_far_to_cel(value),
        2 => convert_cel_to_far(value),
        _ => 0,
    };
    println!("value = {value}, temp = {temp}");
}

fn convert_far_to_cel(far: i32) -> i32 {
    ((far - 32) * 5) / 9
}

fn convert_cel_to_far(cel: i32) -> i32 {
    cel * 9 / 5 + 32
}
