#[cfg(test)]
mod karpekar_tests {

    use crate::kaprekar::kaprekar::kaprekar;

    #[test]
    fn kaprekar_test() {
        assert_eq!(kaprekar(3524), 3);
        assert_eq!(kaprekar(495), 4);
        assert_eq!(kaprekar(6621), 5);
        assert_eq!(kaprekar(6554), 4);
        assert_eq!(kaprekar(1234), 3);
        assert_eq!(kaprekar(6174), 0);
    }

    #[test]
    fn kaprekar_can_go_the_distance() {
        for i in 1..9999 {
            let kaprekar_result = kaprekar(i);
            assert!(-1 <= kaprekar_result && kaprekar_result <= 7);
        }
    }
}
