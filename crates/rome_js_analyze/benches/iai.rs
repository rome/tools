fn rome_all_lowercase() {
    let _ = rome_js_analyze::utils::to_camel_case(iai::black_box("lowercase"));
}

fn case_all_lowercase() {
    let _ = case::CaseExt::to_camel(iai::black_box("lowercase"));
}

fn rome_already_camel_case() {
    let _ = rome_js_analyze::utils::to_camel_case(iai::black_box("camelCase"));
}

fn case_already_camel_case() {
    let _ = case::CaseExt::to_camel(iai::black_box("camelCase"));
}

fn rome_pascal_case() {
    let _ = rome_js_analyze::utils::to_camel_case(iai::black_box("CamelCase"));
}

fn case_pascal_case() {
    let _ = case::CaseExt::to_camel(iai::black_box("CamelCase"));
}

iai::main!(
    rome_all_lowercase,
    case_all_lowercase,
    rome_already_camel_case,
    case_already_camel_case,
    rome_pascal_case,
    case_pascal_case
);
