use super::utils;

const URL: &str = "http://standards-oui.ieee.org/ethertype/eth.csv";
const CACHE: &str = "~/.local/share/ieee/eth.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the EtherType™ (eth)
pub fn get_eth_path() -> BoxResult<String> {
    utils::download(URL, CACHE)?;

    Ok(utils::expand_path(CACHE))
}
