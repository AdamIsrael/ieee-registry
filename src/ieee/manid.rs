use super::utils;

const URL: &str = "http://standards-oui.ieee.org/manid/manid.csv";
const CACHE: &str = "~/.local/share/ieee/man.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the Manufacturer ID (man)
pub fn get_manid_path() -> BoxResult<&'static str> {
    utils::download(URL, CACHE)?;

    Ok(CACHE)
}
