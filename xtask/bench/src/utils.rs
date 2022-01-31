use crate::err_to_string;
use ansi_rgb::{red, Foreground};
use std::path::PathBuf;
use std::str::FromStr;

pub fn get_code(lib: &str) -> Result<(String, String), String> {
    let url = url::Url::from_str(lib).map_err(err_to_string)?;
    let segments = url
        .path_segments()
        .ok_or_else(|| "lib url has no segments".to_string())?;
    let filename = segments
        .last()
        .ok_or_else(|| "lib url has no segments".to_string())?;

    let mut file = PathBuf::from_str("target").map_err(err_to_string)?;
    file.push(filename);

    match std::fs::read_to_string(&file) {
        Ok(code) => {
            println!("[{}] - using [{}]", filename.fg(red()), file.display());
            Ok((filename.to_string(), code))
        }
        Err(_) => {
            println!(
                "[{}] - Downloading [{}] to [{}]",
                filename,
                lib,
                file.display()
            );
            match ureq::get(lib).call() {
                Ok(response) => {
                    let mut reader = response.into_reader();

                    let _ = std::fs::remove_file(&file);
                    let mut writer = std::fs::File::create(&file).map_err(err_to_string)?;
                    let _ = std::io::copy(&mut reader, &mut writer);

                    std::fs::read_to_string(&file)
                        .map_err(err_to_string)
                        .map(|code| (filename.to_string(), code))
                }
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    }
}
