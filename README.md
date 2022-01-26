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
    pub fn new(sol: u64, eth: u64, btc: u64) -> TokenMap {
        TokenMap {
            sol,
            eth,
            btc,
        }
    }
    pub fn add(&self, other: TokenMap) -> TokenMap {
        TokenMap::new(
            sol: self.sol.checked_add(other.sol).unwrap(),
            eth: self.eth.checked_add(other.eth).unwrap(),
            btc: self.btc.checked_add(other.btc).unwrap(),
        )
    }
    pub fn add_assign(&mut self, other: TokenMap) {
        self.sol = self.sol.checked_sub(other.sol).unwrap();
        self.eth = self.eth.checked_sub(other.eth).unwrap();
        self.btc = self.btc.checked_sub(other.btc).unwrap();
    }
    ...
}
```

## You can also add a `_reserved` field (only with type [u8; N]!) which does not interfere with the arithmetics.

```rs
#[derive(StructArithmetic)]
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
    pub _reserved: [u8; 100],
}
```

turns into

```rs
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
    pub _reserved: [u8; 100],
}
impl TokenMap {
    pub fn new(sol: u64, eth: u64, btc: u64) -> TokenMap {
        TokenMap {
            sol,
            eth,
            btc,
            _reserved: [0; 100],
        }
    }
    pub fn add(&self, other: TokenMap) -> TokenMap {
        TokenMap::new(
            sol: self.sol.checked_add(other.sol).unwrap(),
            eth: self.eth.checked_add(other.eth).unwrap(),
            btc: self.btc.checked_add(other.btc).unwrap(),
        )
    }
    pub fn add_assign(&mut self, other: TokenMap) {
        self.sol = self.sol.checked_sub(other.sol).unwrap();
        self.eth = self.eth.checked_sub(other.eth).unwrap();
        self.btc = self.btc.checked_sub(other.btc).unwrap();
    }
    ...
```