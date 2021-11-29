extern crate struct_arithmetic;

#[cfg(test)]
mod tests {
    #[derive(struct_arithmetic::StructArithmetic, Debug, Clone)]
    struct TokenMap {
        pub sol: u64,
        pub eth: u64,
        pub btc: u64,
    }

    const X: TokenMap = TokenMap {
        sol: 10,
        eth: 20,
        btc: 30,
    };
    const Y: TokenMap = TokenMap {
        sol: 10,
        eth: 20,
        btc: 30,
    };

    #[test]
    fn test_add() {
        let z = X.add(&Y);
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_add_assign() {
        let mut x = X.clone();
        x.add_assign(&Y);
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
    }

    #[test]
    fn test_sub() {
        let z = X.sub(&Y);
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x = X.clone();
        x.sub_assign(&Y);
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
    }

    #[test]
    fn test_mul() {
        let z = X.mul(&Y);
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
    }

    #[test]
    fn test_mul_scalar() {
        let z = X.mul_scalar(2);
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_div() {
        let z = X.div(&Y);
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
    }

    #[test]
    fn test_div_scalar() {
        let z = X.div_scalar(10);
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
    }
}
