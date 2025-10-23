fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{s1}' is {len}.");

    let len = calculate_length(&mut s1);

    println!("The length of '{s1}' is {len}.");

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("dsdsd");
    s.len()
}

fn mutate_error() {
    let mut s = String::from("hello");

       let r1 = &s; // no problem
       let r2 = &s; // no problem
       println!("{r1} and {r2}");
       // Variables r1 and r2 will not be used after this point.

       let r3 = &mut s; // no problem
       println!("{r3}");
       println!("{r3}");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
