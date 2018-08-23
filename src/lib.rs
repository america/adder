pub fn add_six(a: i32) -> i32 {
    a + 6
}

#[cfg(test)]
mod tests {
    use super::add_six;
    #[test]
    fn it_works() {
        assert_eq!(8, add_six(2));
    }
}
