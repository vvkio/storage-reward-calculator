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
    let (total_liquidity_required, total_liquidity_required_verified) = estimate_fees(sectors as f64);
    println!("{} sectors require {} FIL in precommit deposit fees, and need {} FIL for provecommit ",
             sectors, total_liquidity_required, total_liquidity_required_verified);
    println!()
}

fn calculate_collateral(sectors: u64) -> (f64, f64) {
    let collateral_in_fil: f64 = sectors as f64 * COLLATERAL_REQUIREMENT_IN_FIL;
    let verified_collateral_in_fil: f64 = collateral_in_fil * 10.0;
    (collateral_in_fil, verified_collateral_in_fil)
}

fn estimate_fees(sectors : f64) -> (f64, f64) {
    let precommit_desposit: f64 = 0.1684;
    let provecommit_fee : f64 = 0.1684;
    // let required_liquidity_verified: f64 = required_liquidity_cc * 10.0;
    let total_precommit_deposits: f64 = sectors * precommit_desposit;
    let total_provecommit_fee: f64 = sectors * provecommit_fee;
    (total_precommit_deposits,total_provecommit_fee)
}


fn print_fees(sectors: u64, precommit_deposit: f64, provecommit_fee: f64) {
    println!(
        "{} sectors, require {:.2} FIL in precommit deposit fees, and need {:.2} FIL for provecommit ",
        sectors, precommit_deposit, provecommit_fee
    );
}

fn print_collateral_requirement(sectors: u64, collateral_in_fil: f64, verified_collateral_in_fil: f64) {
    println!(
        "{} sectors, require {:.2} FIL in collateral, and need {:.2} FIL for verified deals ",
        sectors, collateral_in_fil, verified_collateral_in_fil
    );
}