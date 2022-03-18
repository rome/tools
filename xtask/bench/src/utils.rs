use ansi_rgb::{red, Foreground};
use regex::Regex;
use rome_benchmarker::Code;
use std::env;
use std::path::Path;
use std::str::FromStr;

pub fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

/// Given a filter and a comma separated list of suites, fetch all of the suites
/// that conform to that filter
pub fn fetch_suites(filter: String, suites: String) -> Result<Vec<Code>, String> {
    let filter_regex = regex::Regex::new(filter.as_str()).unwrap();

    let suites_to_run = suites.split(',');
    let mut code = Vec::new();
    for suite_name in suites_to_run {
        match suite_name {
            "*" => {
                code.extend(fetch_suite(
                    include_str!("libs-js.txt").lines(),
                    &filter_regex,
                )?);
                code.extend(fetch_suite(
                    include_str!("libs-ts.txt").lines(),
                    &filter_regex,
                )?);
            }
            "js" => code.extend(fetch_suite(
                include_str!("libs-js.txt").lines(),
                &filter_regex,
            )?),
            "ts" => code.extend(fetch_suite(
                include_str!("libs-ts.txt").lines(),
                &filter_regex,
            )?),
            unknown => {
                eprintln!("Unknown suite: {}", unknown);
            }
        }
    }

    Ok(code)
}

/// Given a suite, i.e. a list of urls, fetch all of the ones that conform to a given filter
fn fetch_suite<'a>(
    suite: impl Iterator<Item = &'a str>,
    filter: &Regex,
) -> Result<Vec<Code>, String> {
    let mut code = Vec::new();
    for code_url in suite {
        if !filter.is_match(code_url) {
            continue;
        }

        code.push(fetch_code(code_url)?);
    }

    Ok(code)
}

/// Given a url for some code, fetches it
fn fetch_code(code_url: &str) -> Result<Code, String> {
    let url = url::Url::from_str(code_url).map_err(err_to_string)?;
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
            Ok(Code {
                id: filename.to_string(),
                url: code_url.to_string(),
                source: code,
            })
        }
        Err(_) => {
            println!(
                "[{}] - Downloading [{}] to [{}]",
                filename,
                code_url,
                file.display()
            );
            match ureq::get(code_url).call() {
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
                        .map(|code| Code {
                            id: filename.to_string(),
                            url: code_url.to_string(),
                            source: code,
                        })
                }
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    }
}
