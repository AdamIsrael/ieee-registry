use super::utils;

const URL: &str = "http://standards-oui.ieee.org/bopid/opid.csv";
const CACHE: &str = "~/.local/share/ieee/opid.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the Operator ID (opid)
pub fn get_opid_path() -> BoxResult<String> {
    utils::download(URL, CACHE)?;

    Ok(utils::expand_path(CACHE))
}
