enum SpreadsheatCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v1 = vec![1, 2, 3];
    v.push(1);
    v.push(2);

    v.push(3);
    v.push(4);

    let third: &i32 = &v[0];

    println!("The third element is {third}");

    let fifth: Option<&i32> = v.get(4);
    match fifth {
        Some(num) => println!("fifth element {num}"),
        None => println!("There is no element"),
    }
    for i in &mut v {
        *i += 50;
        println!("{:?}", i)
    }

    let row = vec![
        SpreadsheatCell::Int(45),
        SpreadsheatCell::Float(1.5),
        SpreadsheatCell::Text(String::from("this is the text"))
    ];
    for i in &row {
        match i {
            SpreadsheatCell::Int(int) => println!("this is the int {int}"),
            SpreadsheatCell::Float(float) => println!("this is the float {float}"),
            SpreadsheatCell::Text(text) => println!("{text}"),
        }
    }
}
