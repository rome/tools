use std::{path::PathBuf, str::FromStr};

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

pub fn get_code(lib: &str) -> Result<String, String> {
    let url = url::Url::from_str(lib).map_err(err_to_string)?;
    let segments = url.path_segments() 
        .ok_or_else(|| "lib url has no segments".to_string())?;
    let filename = segments.last()
        .ok_or_else(|| "lib url has no segments".to_string())?;

    let out = PathBuf::from_str("target").map_err(err_to_string)?;
    let mut file = out.clone();
    file.push(filename);

    match std::fs::read_to_string(&file) {
        Ok(code) => {
            println!("[{}] - using [{}]", filename, file.display());
            Ok(code)
        }
        Err(_) => {
            println!("[{}] - Downloading [{}] to [{}]", filename, lib, file.display());
            match ureq::get(lib).call() {
                Ok(response) => {
                    let mut reader = response.into_reader();

                    let _ = std::fs::remove_file(&file);
                    let mut writer = std::fs::File::create(&file).map_err(err_to_string)?;
                    let _ = std::io::copy(&mut reader, &mut writer);

                    std::fs::read_to_string(&file)
                        .map_err(err_to_string)
                }
                Err(e) => Err(format!("{:?}", e))
            }
        }
    }
}

pub fn run() {
	
	let libs = include_str!("libs.txt").lines();
	for lib in libs {
		let code = get_code(lib);
        match code {
            Ok(code ) => {
                let _ = std::panic::catch_unwind(|| rslint_parser::parse_module(code.as_str(), 0));
            },
            Err(e) => println!("{:?}", e),
        }
	}

	println!("end");
}
