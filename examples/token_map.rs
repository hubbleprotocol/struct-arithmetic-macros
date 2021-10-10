use token_derive::StructArithmetic;

#[derive(StructArithmetic, Debug)]
struct TokenMap {
    pub sol: u64,
    pub eth: u64,
    pub btc: u64,
}
fn main() {
    let x = TokenMap {
        sol: 10,
        eth: 20,
        btc: 30,
    };
    let y = TokenMap {
        sol: 10,
        eth: 20,
        btc: 30,
    };
    let z = x.add(y);
    println!("{:?}", z);
}
