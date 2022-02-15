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

## It also supports adding and using non-reserved arrays of fixed lengths.

```rs
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
    pub tk1: [u64; 2],
    pub _reserved: [u8; 128],
    pub tk2: [u128; 3],
}
```

turns into

```rs
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
    pub tk1: [u64; 2],
    pub _reserved: [u8; 128],
    pub tk2: [u128; 3],
}

impl TokenMap {
    pub fn new(sol: u64, eth: u64, btc: u64, tk1: [u64; 2], tk2: [u128; 3]) -> TokenMap {
        TokenMap {
            sol,
            eth,
            btc,
            tk1,
            _reserved: [0; 128],
            tk2,
        }
    }
    pub fn add(&self, other: &TokenMap) -> Option<TokenMap> {
        let mut tk1 = [u64::default(); 2];
        for i in 0..self.tk1.len() {
            tk1[i] = self.tk1[i].checked_add(other.tk1[i])?;
        }
        let mut tk2 = [u128::default(); 3];
        for i in 0..self.tk2.len() {
            tk2[i] = self.tk2[i].checked_add(other.tk2[i])?;
        }
        Some(TokenMap::new(
            self.sol.checked_add(other.sol)?,
            self.eth.checked_add(other.eth)?,
            self.btc.checked_add(other.btc)?,
            tk1,
            tk2,
        ))
    }
    pub fn add_assign(&mut self, other: &TokenMap) -> Option<()> {
        self.sol = self.sol.checked_add(other.sol)?;
        self.eth = self.eth.checked_add(other.eth)?;
        self.btc = self.btc.checked_add(other.btc)?;
        for i in 0..self.tk1.len() {
            self.tk1[i] = self.tk1[i].checked_add(other.tk1[i])?;
        }
        for i in 0..self.tk2.len() {
            self.tk2[i] = self.tk2[i].checked_add(other.tk2[i])?;
        }
        Some(())
    }
}
```
