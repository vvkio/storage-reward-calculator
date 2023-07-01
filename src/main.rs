use std::io;

const GIB: u64 = 32; // 32 GiB
const PIB_IN_GIB: u64 = 1024 * 1024; // 1 PiB = 1048576 GiB
const COLLATERAL_REQUIREMENT_IN_FIL: f64 = 0.2033;
const PRECOMMIT_VALUE_IN_FIL: f64 = 0.0596;
const PROVECOMMIT_VALUE_IN_FIL: f64 = 0.1684;

fn main() {
    println!("Enter the desired storage capacity in PiB:");

    let mut pib_str = String::new();
    io::stdin().read_line(&mut pib_str).unwrap();

    let pib: u64 = pib_str.trim().parse().unwrap();
    let total_gib: u64 = pib * PIB_IN_GIB;

    let sectors: u64 = total_gib / GIB;
    let (collateral_in_fil, verified_collateral_in_fil) = calculate_collateral(sectors);
    let (total_liqudity_required, total_liqudity_required_verified) = calculate_liquidity(collateral_in_fil);
  
}

fn calculate_collateral(sectors: u64) -> (f64, f64) {
    let collateral_in_fil: f64 = sectors as f64 * COLLATERAL_REQUIREMENT_IN_FIL;
    let verified_collateral_in_fil: f64 = collateral_in_fil * 10.0;
    (collateral_in_fil, verified_collateral_in_fil)
}

fn calculate_liquidity(collateral_in_fil: f64) -> (f64, f64) {
    let required_liqudity_cc: f64 = 0.2033+0.0596+0.1684;
    let required_liqudity_verified: f64 = required_liqudity_cc * 10.0;

    let total_liqudity_required: f64 = collateral_in_fil + required_liqudity_cc;
    let total_liqudity_required_verified: f64 = total_liqudity_required * 10.0;

    (total_liqudity_required, total_liqudity_required_verified)
}

fn print_collateral(sectors: u64, collateral: f64, verified_collateral: f64) {
    println!(
        "{} sectors, require {:.2} FIL of collateral for CC, while it requires {:.2} FIL for verified deals",
        sectors, collateral, verified_collateral
    );
}

fn print_liquidity(sectors: u64, liquidity: f64, verified_liquidity: f64) {
    println!(
        "{} sectors, require {:.2} FIL of liquidity for CC, while it requires {:.2} FIL for verified deals",
        sectors, liquidity, verified_liquidity
    );
}