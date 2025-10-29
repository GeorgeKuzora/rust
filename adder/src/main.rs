fn main() {}

pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5)
    }

    #[test]
    fn fails() {
        panic!("this test must fail")
    }
}
