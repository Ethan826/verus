#[no_mangle]
pub extern "C" fn doubler(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubler() {
        assert_eq!(doubler(2), 4);
    }
}
