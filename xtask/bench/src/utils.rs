use crate::err_to_string;
use ansi_rgb::{red, Foreground};
use std::env;
use std::path::Path;
use std::str::FromStr;

pub fn get_code(lib: &str) -> Result<(String, String), String> {
    let url = url::Url::from_str(lib).map_err(err_to_string)?;
    let segments = url
        .path_segments()
        .ok_or_else(|| "lib url has no segments".to_string())?;
    let filename = segments
        .last()
        .ok_or_else(|| "lib url has no segments".to_string())?;

    let file = Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .join("target")
    .join(filename);

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

                    let mut writer = std::fs::File::create(&file).map_err(err_to_string)?;
                    if let Err(err) = std::io::copy(&mut reader, &mut writer) {
                        drop(writer);
                        std::fs::remove_file(&file).ok();
                        return Err(err_to_string(err));
                    }
                    std::fs::read_to_string(&file)
                        .map_err(err_to_string)
                        .map(|code| (filename.to_string(), code))
                }
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    }
}
