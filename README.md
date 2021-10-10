## Struct arithmetic derive

```rs
#[derive(StructArithmetic)]
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
}
```

turns into

```rs
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
}
impl TokenMap {
    pub fn add(&self, other: TokenMap) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_add(other.sol).unwrap(),
            eth: self.eth.checked_add(other.eth).unwrap(),
            btc: self.btc.checked_add(other.btc).unwrap(),
        }
    }
    pub fn add_assign(&mut self, other: TokenMap) {
        self.sol = self.sol.checked_sub(other.sol).unwrap();
        self.eth = self.eth.checked_sub(other.eth).unwrap();
        self.btc = self.btc.checked_sub(other.btc).unwrap();
    }
    pub fn sub(&self, other: TokenMap) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_sub(other.sol).unwrap(),
            eth: self.eth.checked_sub(other.eth).unwrap(),
            btc: self.btc.checked_sub(other.btc).unwrap(),
        }
    }
    pub fn sub_assign(&self, other: TokenMap) -> TokenMap {
        self.sol = self.sol.checked_add(other.sol).unwrap();
        self.eth = self.eth.checked_add(other.eth).unwrap();
        self.btc = self.btc.checked_add(other.btc).unwrap();
    }
    pub fn mul(&self, other: TokenMap) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_mul(other.sol).unwrap(),
            eth: self.eth.checked_mul(other.eth).unwrap(),
            btc: self.btc.checked_mul(other.btc).unwrap(),
        }
    }
    pub fn div(&self, other: TokenMap) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_div(other.sol).unwrap(),
            eth: self.eth.checked_div(other.eth).unwrap(),
            btc: self.btc.checked_div(other.btc).unwrap(),
        }
    }
    pub fn div_scalar(&self, factor: u64) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_div(factor).unwrap(),
            eth: self.eth.checked_div(factor).unwrap(),
            btc: self.btc.checked_div(factor).unwrap(),
        }
    }
    pub fn mul_scalar(&self, factor: u64) -> TokenMap {
        TokenMap {
            sol: self.sol.checked_mul(factor).unwrap(),
            eth: self.eth.checked_mul(factor).unwrap(),
            btc: self.btc.checked_mul(factor).unwrap(),
        }
    }
}

```