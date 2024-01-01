use super::utils;

const URL: &str = "http://standards-oui.ieee.org/oui36/oui36.csv";
const CACHE: &str = "~/.local/share/ieee/oui36.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the Mac Address Block Small (oui36)
pub fn get_oui36_path() -> BoxResult<String> {
    utils::download(URL, CACHE)?;

    Ok(utils::expand_path(CACHE))
}
