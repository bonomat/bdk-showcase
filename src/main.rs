mod bkd018;
mod bkd019;
mod bkd020;
mod bkd021;
mod bkd022;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
struct BdkDetails {
    name: String,
    address_index_0: String,
    address_index_1: String,
    last_unused: String,
    balance: u64,
}

impl Display for BdkDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "name=`{}`, balance=`{}`, address_index_0=`{}`, address_index_1=`{}`, last_unused=`{}`",
            self.name, self.balance, self.address_index_0, self.address_index_1, self.last_unused
        );
        write!(f, "{}", string)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Mnemonic: danger assume iron tobacco advice floor ozone awesome erode describe minimum raw
    let xprv = "tprv8gA4ePCxg235CVo2dSXEFJQsoPzfhVzoyujqUbZ7mPANkmuW1gM1bgWoC9AWByFnj6LxXTpesrzccdNKYe1kBRaGrwyohnigwY3BHj3kroL";
    let bdk18_details = bkd018::execute(xprv).unwrap();
    let bdk19_details = bkd019::execute(xprv).unwrap();
    let bdk20_details = bkd020::execute(xprv).unwrap();
    let bdk21_details = bkd021::execute(xprv).unwrap();
    let bdk22_details = bkd022::execute(xprv).unwrap();

    println!("BDK 018 {}", bdk18_details);
    println!("BDK 019 {}", bdk19_details);
    println!("BDK 020 {}", bdk20_details);
    println!("BDK 021 {}", bdk21_details);
    println!("BDK 022 {}", bdk22_details);

    Ok(())
}
