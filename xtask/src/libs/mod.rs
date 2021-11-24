use std::{path::PathBuf, str::FromStr};

pub fn run() {
	let out = PathBuf::from_str("target").unwrap();
	let libs = include_str!("libs.txt").lines();
	for lib in libs {
		let url = url::Url::from_str(lib).unwrap();
		let filename = url.path_segments().unwrap().last().unwrap();

		let mut file = out.clone();
		file.push(filename);

		let code = match std::fs::read_to_string(&file) {
			Ok(code) => {
				println!("{} - using {}", filename, file.display());
				code
			}
			Err(_) => {
				println!("{} - Downloading {} to {}", filename, lib, file.display());
				match ureq::get(lib).call() {
					Ok(response) => {
						let mut reader = response.into_reader();

						let _ = std::fs::remove_file(&file);
						let mut writer = std::fs::File::create(&file).unwrap();
						let _ = std::io::copy(&mut reader, &mut writer);

						std::fs::read_to_string(&file).unwrap()
					}
					Err(_) => todo!(),
				}
			}
		};

		let _ = std::panic::catch_unwind(|| rslint_parser::parse_module(code.as_str(), 0));
	}

	println!("end");
}
