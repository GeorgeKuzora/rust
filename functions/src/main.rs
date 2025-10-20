fn main() {
    println!("Hello, world!");

    another_function(five(5), 'h');
}

fn another_function(x: i32, label: char) {
    println!("Another function! x = {x}, lable is {}", label);
}

fn five(x: i32) -> i32 {
    x
}
