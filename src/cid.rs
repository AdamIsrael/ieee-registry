use super::utils;

const URL: &str = "http://standards-oui.ieee.org/cid/cid.csv";
const CACHE: &str = "~/.local/share/ieee/cid.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the Company ID (cid)
pub fn get_cid_path() -> BoxResult<&'static str> {
    utils::download(URL, CACHE)?;

    Ok(CACHE)
}
