pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(sum(2, 3), 5);
    }
}
