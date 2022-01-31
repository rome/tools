pub(crate) mod features;
pub(crate) mod utils;

use crate::features::parser::benchmark_parse_lib;
use ansi_rgb::{red, Foreground};
use std::time::Duration;
use std::{path::PathBuf, str::FromStr};

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

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

pub fn run(filter: String, criterion: bool, baseline: Option<String>) {
    let regex = regex::Regex::new(filter.as_str()).unwrap();
    let libs = include_str!("libs.txt").lines();

    let mut summary = vec![];

    for lib in libs {
        if !regex.is_match(lib) {
            continue;
        }

        let code = get_code(lib);

        match code {
            Ok((id, code)) => {
                let code = code.as_str();

                // Do all steps with criterion now
                if criterion {
                    let mut criterion = criterion::Criterion::default()
                        .without_plots()
                        .measurement_time(Duration::new(10, 0));
                    if let Some(ref baseline) = baseline {
                        criterion = criterion.save_baseline(baseline.to_string());
                    }
                    let mut group = criterion.benchmark_group("parser");
                    group.throughput(criterion::Throughput::Bytes(code.len() as u64));
                    group.bench_function(&id, |b| {
                        b.iter(|| {
                            let _ = criterion::black_box(rslint_parser::parse_module(code, 0));
                        })
                    });
                    group.finish();
                } else {
                    //warmup
                    rslint_parser::parse_module(code, 0);
                }

                let result = benchmark_parse_lib(&id, code);
                summary.push(result.summary());

                println!("Benchmark: {}", lib);
                println!("{}", result);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Summary");
    println!("-------");
    for l in summary {
        println!("{}", l);
    }
}
