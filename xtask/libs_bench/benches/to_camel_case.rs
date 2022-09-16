fn to_camel_case_rome_all_lowercase() {
    let _ = rome_js_analyze::shared::to_camel_case(iai::black_box("lowercase"));
}

fn to_camel_case_case_all_lowercase() {
    let _ = case::CaseExt::to_camel(iai::black_box("lowercase"));
}

fn to_camel_case_rome_already_camel_case() {
    let _ = rome_js_analyze::shared::to_camel_case(iai::black_box("camelCase"));
}

fn to_camel_case_case_already_camel_case() {
    let _ = case::CaseExt::to_camel(iai::black_box("camelCase"));
}

fn to_camel_case_rome_pascal_case() {
    let _ = rome_js_analyze::shared::to_camel_case(iai::black_box("CamelCase"));
}

fn to_camel_case_case_pascal_case() {
    let _ = case::CaseExt::to_camel(iai::black_box("CamelCase"));
}

iai::main!(
    to_camel_case_rome_all_lowercase,
    to_camel_case_case_all_lowercase,
    to_camel_case_rome_already_camel_case,
    to_camel_case_case_already_camel_case,
    to_camel_case_rome_pascal_case,
    to_camel_case_case_pascal_case,
);
