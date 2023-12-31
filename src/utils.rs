use std::fs::{create_dir_all, metadata};
use std::io::Cursor;

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use expanduser::expanduser;

const MAX_AGE: u64 = 86400 * 30;

/// Download the file to a specific destination
pub fn download(url: &str, destination: &str) -> BoxResult<()> {
    let path = expanduser(destination)?;

    // Short-circuit if the file is already downloaded and less than MAX_AGE old
    if let Ok(age) = age(path.clone().into_os_string().to_str().unwrap()) {
        if age < MAX_AGE {
            return Ok(());
        }
    }

    // Download the file
    match reqwest::blocking::get(url) {
        Ok(response) => {
            // make sure the path exists
            create_dir_all(path.parent().unwrap())?;

            let mut file = std::fs::File::create(path.display().to_string())?;

            let mut content = Cursor::new(response.bytes()?);
            std::io::copy(&mut content, &mut file)?;

            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

/// Checks the age of a file, measured in days
pub fn age(target: &str) -> BoxResult<u64> {
    let md = metadata(target)?;
    let accessed = md.accessed()?;
    let age = accessed.elapsed()?;
    Ok(age.as_secs())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile;

    use super::*;

    const URL: &str = "https://www.ieee.org/robots.txt";

    #[test]
    fn test_download() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmp = tmpdir.path().join("robots.txt");
        let destination = tmp.to_str().unwrap();

        match download(URL, destination) {
            Ok(_) => {
                // Verify that the file exists and has content
                let md = fs::metadata(tmp).unwrap();
                assert_ne!(0, md.len());
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    /// Test the age of a newly created file
    fn test_age() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmp = tmpdir.path().join("robots.txt");
        let destination = tmp.to_str().unwrap();
        fs::write(destination, "Disallow: /").unwrap();

        let age = age(destination).unwrap();
        assert!(age < 60);
    }
}
