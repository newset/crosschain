fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let value = 4;
        assert_eq!(4, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = 8;
        assert_eq!(5, value);
    }
}
