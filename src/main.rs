/// A CLI interface to the ieee-registry library
///
///
use ieee_registry::ieee::*;

fn main() {
    println!("Caching file(s)...");
    println!("\t{:?}", get_cid_path().unwrap());
    println!("\t{:?}", get_eth_path().unwrap());
    println!("\t{:?}", get_iab_path().unwrap());
    println!("\t{:?}", get_mam_path().unwrap());
    println!("\t{:?}", get_manid_path().unwrap());
    println!("\t{:?}", get_opid_path().unwrap());
    println!("\t{:?}", get_oui_path().unwrap());
    println!("\t{:?}", get_oui36_path().unwrap());
}
