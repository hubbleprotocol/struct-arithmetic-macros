extern crate struct_arithmetic;

#[cfg(test)]
mod tests_simple {
    #[derive(struct_arithmetic::StructArithmetic, Debug, Clone)]
    struct TokenMap {
        pub sol: u64,
        pub eth: u64,
        pub btc: u64,
    }

    #[test]
    fn test_add() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.add(&Y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_add_assign() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let mut x = X.clone();
        x.add_assign(&Y);
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
    }

    #[test]
    fn test_sub() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.sub(&Y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
    }

    #[test]
    fn test_sub_assign() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let mut x = X.clone();
        x.sub_assign(&Y);
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
    }

    #[test]
    fn test_mul() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.mul(&Y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
    }

    #[test]
    fn test_mul_scalar() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_mul_fraction() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.mul_fraction(1, 2).unwrap();
        assert_eq!(z.sol, 5);
        assert_eq!(z.eth, 10);
        assert_eq!(z.btc, 15);
    }

    #[test]
    fn test_div() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.div(&Y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
    }

    #[test]
    fn test_div_scalar() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
    }
}

#[cfg(test)]
mod tests_reserved {
    #[derive(struct_arithmetic::StructArithmetic, Debug, Clone)]
    struct TokenMap {
        pub sol: u64,
        pub eth: u64,
        pub btc: u64,
        pub _reserved: [u8; 128],
    }

    #[test]
    fn test_add() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.add(&Y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_add_assign() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let mut x = X.clone();
        x.add_assign(&Y).unwrap();
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
    }

    #[test]
    fn test_sub() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.sub(&Y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
    }

    #[test]
    fn test_sub_assign() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let mut x = X.clone();
        x.sub_assign(&Y).unwrap();
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
    }

    #[test]
    fn test_mul() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.mul(&Y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
    }

    #[test]
    fn test_mul_scalar() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_div() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.div(&Y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
    }

    #[test]
    fn test_div_scalar() {
        let X: TokenMap = TokenMap::new(10, 20, 30);
        let Y: TokenMap = TokenMap::new(10, 20, 30);
        let z = X.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
    }
}
