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
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.add(&y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_add_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        x.add_assign(&y);
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
    }

    #[test]
    fn test_sub() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.sub(&y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        x.sub_assign(&y);
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
    }

    #[test]
    fn test_mul() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.mul(&y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
    }

    #[test]
    fn test_mul_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_div() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.div(&y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
    }

    #[test]
    fn test_div_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
    }
}

mod tests_array {
    #[derive(struct_arithmetic::StructArithmetic, Debug, Clone)]
    struct TokenMap {
        pub sol: u64,
        pub eth: u64,
        pub btc: u64,
        pub secondary: [u64; 2],
    }

    #[test]
    fn test_add() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.add(&y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
        assert_eq!(z.secondary[0], 2);
        assert_eq!(z.secondary[1], 4);
    }

    #[test]
    fn test_add_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        x.add_assign(&y);
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
        assert_eq!(x.secondary[0], 2);
        assert_eq!(x.secondary[1], 4);
    }

    #[test]
    fn test_sub() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.sub(&y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
        assert_eq!(z.secondary[0], 0);
        assert_eq!(z.secondary[1], 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        x.sub_assign(&y);
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
        assert_eq!(x.secondary[0], 0);
        assert_eq!(x.secondary[1], 0);
    }

    #[test]
    fn test_mul() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.mul(&y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
        assert_eq!(z.secondary[0], 1);
        assert_eq!(z.secondary[1], 4);
    }

    #[test]
    fn test_mul_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
        assert_eq!(z.secondary[0], 2);
        assert_eq!(z.secondary[1], 4);
    }

    #[test]
    fn test_div() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.div(&y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
        assert_eq!(z.secondary[0], 1);
        assert_eq!(z.secondary[1], 1);
    }

    #[test]
    fn test_div_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2]);
        let z = x.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
        assert_eq!(z.secondary[0], 0);
        assert_eq!(z.secondary[1], 0);
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
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.add(&y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_add_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        x.add_assign(&y).unwrap();
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
    }

    #[test]
    fn test_sub() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.sub(&y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        x.sub_assign(&y).unwrap();
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
    }

    #[test]
    fn test_mul() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.mul(&y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
    }

    #[test]
    fn test_mul_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
    }

    #[test]
    fn test_div() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let y: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.div(&y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
    }

    #[test]
    fn test_div_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30);
        let z = x.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
    }
}

#[cfg(test)]
mod tests_multiple_scenarios {
    #[derive(struct_arithmetic::StructArithmetic, Debug, Clone)]
    struct TokenMap {
        pub sol: u64,
        pub eth: u64,
        pub btc: u64,
        pub tk1: [u64; 2],
        pub _reserved: [u8; 128],
        pub tk2: [u128; 3],
    }

    #[test]
    fn test_add() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.add(&y).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
        assert_eq!(z.tk1[0], 2);
        assert_eq!(z.tk1[1], 4);
        assert_eq!(z.tk2[0], 6);
        assert_eq!(z.tk2[1], 8);
        assert_eq!(z.tk2[2], 10);
    }

    #[test]
    fn test_add_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        x.add_assign(&y).unwrap();
        assert_eq!(x.sol, 20);
        assert_eq!(x.eth, 40);
        assert_eq!(x.btc, 60);
        assert_eq!(x.tk1[0], 2);
        assert_eq!(x.tk1[1], 4);
        assert_eq!(x.tk2[0], 6);
        assert_eq!(x.tk2[1], 8);
        assert_eq!(x.tk2[2], 10);
    }

    #[test]
    fn test_sub() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.sub(&y).unwrap();
        assert_eq!(z.sol, 0);
        assert_eq!(z.eth, 0);
        assert_eq!(z.btc, 0);
        assert_eq!(z.tk1[0], 0);
        assert_eq!(z.tk1[1], 0);
        assert_eq!(z.tk2[0], 0);
        assert_eq!(z.tk2[1], 0);
        assert_eq!(z.tk2[2], 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        x.sub_assign(&y).unwrap();
        assert_eq!(x.sol, 0);
        assert_eq!(x.eth, 0);
        assert_eq!(x.btc, 0);
        assert_eq!(x.tk1[0], 0);
        assert_eq!(x.tk1[1], 0);
        assert_eq!(x.tk2[0], 0);
        assert_eq!(x.tk2[1], 0);
        assert_eq!(x.tk2[2], 0);
    }

    #[test]
    fn test_mul() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.mul(&y).unwrap();
        assert_eq!(z.sol, 100);
        assert_eq!(z.eth, 400);
        assert_eq!(z.btc, 900);
        assert_eq!(z.tk1[0], 1);
        assert_eq!(z.tk1[1], 4);
        assert_eq!(z.tk2[0], 9);
        assert_eq!(z.tk2[1], 16);
        assert_eq!(z.tk2[2], 25);
    }

    #[test]
    fn test_mul_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.mul_scalar(2).unwrap();
        assert_eq!(z.sol, 20);
        assert_eq!(z.eth, 40);
        assert_eq!(z.btc, 60);
        assert_eq!(z.tk1[0], 2);
        assert_eq!(z.tk1[1], 4);
        assert_eq!(z.tk2[0], 6);
        assert_eq!(z.tk2[1], 8);
        assert_eq!(z.tk2[2], 10);
    }

    #[test]
    fn test_div() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let y: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.div(&y).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 1);
        assert_eq!(z.btc, 1);
        assert_eq!(z.tk1[0], 1);
        assert_eq!(z.tk1[1], 1);
        assert_eq!(z.tk2[0], 1);
        assert_eq!(z.tk2[1], 1);
        assert_eq!(z.tk2[2], 1);
    }

    #[test]
    fn test_div_scalar() {
        let x: TokenMap = TokenMap::new(10, 20, 30, [1, 2], [3, 4, 5]);
        let z = x.div_scalar(10).unwrap();
        assert_eq!(z.sol, 1);
        assert_eq!(z.eth, 2);
        assert_eq!(z.btc, 3);
        assert_eq!(z.tk1[0], 0);
        assert_eq!(z.tk1[1], 0);
        assert_eq!(z.tk2[0], 0);
        assert_eq!(z.tk2[1], 0);
        assert_eq!(z.tk2[2], 0);
    }
}
