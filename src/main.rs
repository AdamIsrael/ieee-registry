/// A CLI interface to the ieee-registry library
use expanduser::expanduser;

use ieee_registry::*;

fn main() {
    println!("Caching IEEE registry file(s)...");
    println!("✔ {}", expanduser(get_cid_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_eth_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_iab_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_mam_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_manid_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_opid_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_oui_path().unwrap()).unwrap().display());
    println!("✔ {}", expanduser(get_oui36_path().unwrap()).unwrap().display());
}
