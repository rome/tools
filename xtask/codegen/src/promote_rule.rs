use case::CaseExt;
use fs_extra::dir::{move_dir, CopyOptions};
use fs_extra::file;
use fs_extra::file::move_file;
use std::env;
use std::path::PathBuf;

const KNOWN_GROUPS: [&str; 7] = [
    "a11y",
    "suspicious",
    "correctness",
    "performance",
    "security",
    "style",
    "complexity",
];

const KNOWN_PATHS: [&str; 3] = [
    "crates/rome_js_analyze/src/analyzers",
    "crates/rome_js_analyze/src/semantic_analyzers",
    "crates/rome_js_analyze/src/aria_analyzers",
];
pub fn promote_rule(rule_name: &str, new_group: &str) {
    let current_dir = env::current_dir().ok().unwrap();

    if !KNOWN_GROUPS.contains(&new_group) {
        panic!(
            "The group '{}' doesn't exist. Available groups: {}",
            new_group,
            KNOWN_GROUPS.join(", ")
        )
    }

    let rule_name_snake = rule_name.to_snake();

    // look for the rule in the source code
    let mut rule_path = None;
    let mut analyzers_path = None;
    for known_path in KNOWN_PATHS {
        let local_rule_path = current_dir
            .join(known_path)
            .join("nursery")
            .join(format!("{}.rs", &rule_name_snake));
        if local_rule_path.exists() {
            rule_path = Some(local_rule_path);
            analyzers_path = Some(PathBuf::from(known_path));
            break;
        }
    }

    if let (Some(rule_path), Some(analyzers_path)) = (rule_path, analyzers_path) {
        // rule found!
        let new_group_path = analyzers_path.join(new_group);
        let new_rule_path = new_group_path.join(format!("{}.rs", rule_name_snake));

        let categories_path = "crates/rome_diagnostics_categories/src/categories.rs";
        let categories = std::fs::read_to_string(categories_path).unwrap();

        let categories = categories.replace(
            &format!("lint/nursery/{}", rule_name),
            &format!("lint/{}/{}", new_group, rule_name),
        );

        move_file(rule_path, new_rule_path, &file::CopyOptions::default()).unwrap();
        std::fs::write(categories_path, categories).unwrap();

        let old_test_path = current_dir
            .join("crates/rome_js_analyze/tests/specs/nursery")
            .join(rule_name);
        let new_test_path = current_dir
            .join("crates/rome_js_analyze/tests/specs")
            .join(new_group)
            .join(rule_name);

        std::fs::create_dir(new_test_path).unwrap();
        move_dir(
            old_test_path.display().to_string(),
            current_dir
                .join("crates/rome_js_analyze/tests/specs")
                .join(new_group)
                .display()
                .to_string(),
            &CopyOptions::new(),
        )
        .unwrap();
    } else {
        panic!("Couldn't find the rule {}", rule_name);
    }
}
