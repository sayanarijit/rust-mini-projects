#[cfg(test)]
mod tests {

    fn add_two(x: i32) -> i32 {
        if x < 0 {
            panic!("{} is negative", x);
        }
        x + 2
    }

    #[test]
    fn it_works() {
        assert!(add_two(2) == 4);
        assert!(!(add_two(10) == 14));
        assert_eq!(add_two(6), 8);
        assert_ne!(add_two(10), 14);
    }

    #[test]
    fn it_fails() {
        assert!(add_two(3) == 6, "{} is not {}", 3, 6)
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        add_two(-1);
    }

    #[test]
    #[should_panic(expected = "is negative")]
    fn it_panics_for_negative() {
        add_two(-1);
    }

    #[test]
    fn it_also_works() -> Result<(), String> {
        if add_two(2) == 4 {
            return Ok(());
        }
        Err(String::from("didn't add two"))
    }
}
