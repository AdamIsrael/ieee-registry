use super::utils;

const URL: &str = "http://standards-oui.ieee.org/oui/oui.csv";
const CACHE: &str = "~/.local/share/ieee/oui.csv";

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Cache the Mac Address Block Large (oui)
pub fn get_oui_path() -> BoxResult<String> {
    utils::download(URL, CACHE)?;

    Ok(utils::expand_path(CACHE))
}
