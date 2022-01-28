use std::char::decode_utf16;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use xtask::project_root;

use crate::results::emit_compare;
use crate::TestResults;

// this is the filename of the results coming from `main` branch
const BASE_RESULT_FILE: &str = "base_results.json";

// this is the filename of the results coming from the current PR
const NEW_RESULT_FILE: &str = "new_results.json";

pub fn coverage_compare(
    base_result_path: Option<&str>,
    new_result_path: Option<&str>,
    markdown: bool,
) {
    // resolve the path passed as argument, or retrieve the default one
    let base_result_dir = if let Some(base_result_path) = base_result_path {
        PathBuf::from(base_result_path)
    } else {
        project_root().join(BASE_RESULT_FILE)
    };

    // resolve the path passed as argument, or retrieve the default one
    let new_result_dir = if let Some(new_result_path) = new_result_path {
        PathBuf::from(new_result_path)
    } else {
        project_root().join(NEW_RESULT_FILE)
    };

    if !base_result_dir.exists() {
        panic!(
            "The path to the base results doesn't exist: {:?}",
            base_result_dir
        );
    }

    if !&new_result_dir.exists() {
        panic!(
            "The path to the new results doesn't exist: {:?}",
            new_result_dir
        );
    }

    let base_results = read_test_results(base_result_dir.as_path(), "base");
    let mut new_results = read_test_results(new_result_dir.as_path(), "new");

    for (suite, base) in base_results.into_iter() {
        let new_result = new_results.remove(&suite).unwrap_or_else(TestResults::new);

        emit_compare(&base, &new_result, suite.as_str(), markdown);
    }

    for (suite, new) in new_results.drain() {
        emit_compare(&TestResults::new(), &new, suite.as_str(), markdown);
    }
}

fn read_test_results(path: &Path, name: &'static str) -> HashMap<String, TestResults> {
    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

    enum FileEncoding {
        Unknown,
        Utf8,
        Utf16Le,
        Utf16Be,
    }

    let mut encoding = FileEncoding::Unknown;
    let mut content: &[u8] = &buffer;

    // Read the BOM if present and skip it
    let bom = content.get(0..3);
    if let Some(&[0xef, 0xbb, 0xbf]) = bom {
        content = &content[3..];
        encoding = FileEncoding::Utf8;
    } else if let Some(&[0xfe, 0xff, _]) = bom {
        content = &content[2..];
        encoding = FileEncoding::Utf16Be;
    } else if let Some(&[0xff, 0xfe, _]) = bom {
        content = &content[2..];
        encoding = FileEncoding::Utf16Le;
    }

    if matches!(encoding, FileEncoding::Unknown | FileEncoding::Utf8) {
        // Attempt to parse as UTF-8
        let result = serde_json::from_slice(content);

        if let FileEncoding::Utf8 = encoding {
            // If the file is known to be UTF-8 unwrap the result
            return result.unwrap_or_else(|err| {
                panic!(
                    "Can't parse the JSON file of the {} results: {:?}",
                    name, err
                )
            });
        } else if let Ok(result) = result {
            // Otherwise only return if the parsing was successful
            return result;
        }
    }

    // If a UTF-16 BOM was found or an error was encountered, attempt to parse as UTF-16
    let content_str = decode_utf16(content.chunks(2).map(|bytes| match encoding {
        FileEncoding::Utf16Be => u16::from_be_bytes([bytes[0], bytes[1]]),
        FileEncoding::Utf16Le => u16::from_le_bytes([bytes[0], bytes[1]]),
        // If the encoding is unknown attempt to decode in native endianness
        FileEncoding::Unknown => u16::from_ne_bytes([bytes[0], bytes[1]]),
        FileEncoding::Utf8 => unreachable!(),
    }))
    .collect::<Result<String, _>>()
    .unwrap_or_else(|err| panic!("Can't read the file of the {} results: {:?}", name, err));

    serde_json::from_str(&content_str).unwrap_or_else(|err| {
        panic!(
            "Can't parse the JSON file of the {} results: {:?}",
            name, err
        )
    })
}
