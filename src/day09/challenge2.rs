#[cfg(test)]
mod test {
    use crate::day09::challenge1::main_func;

    #[test]
    fn test_main_real() {
        let input = include_str!("input");
        let result = main_func(input, 2);
        assert_eq!(result, vec![33343]);
        println!("challenge 9.2: {:?}", result);
    }

    //    #[test]
    //    fn test_main_1() {
    //        assert_eq!(main_func(""), 0);
    //    }
    //
    //    #[test]
    //    fn test_main_2() {
    //        assert_eq!(main_func(""), 0);
    //    }
}
