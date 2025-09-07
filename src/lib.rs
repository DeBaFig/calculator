pub mod calc1;
pub mod calc2;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = calc1::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
        fn test_adicao() {
            assert_eq!(calc1::add(2, 3), 5);
        }

    #[test]
        fn test_subtracao() {
            assert_eq!(calc1::sub(5, 2), 3);
        }

    #[test]
        fn test_multiplicacao() {
            assert_eq!(calc2::multiply(2, 3), 6);
        }

    #[test]
        fn test_divisao() {
            assert_eq!(calc2::rate(6, 2), 3);
        }
}