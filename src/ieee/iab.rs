use super::utils;

const URL: &str = "http://standards-oui.ieee.org/iab/iab.csv";
const CACHE: &str = "~/.local/share/ieee/iab.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the IAB assignments
pub fn get_iab_path() -> BoxResult<&'static str> {
    utils::download(URL, CACHE)?;

    Ok(CACHE)
}
