use super::utils;

const URL: &str = "http://standards-oui.ieee.org/oui28/mam.csv";
const CACHE: &str = "~/.local/share/ieee/mam.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the MAC Address Block Medium (mam)
pub fn get_mam_path() -> BoxResult<String> {
    utils::download(URL, CACHE)?;

    Ok(utils::expand_path(CACHE))
}
