pub fn parse_str<'a>(input: &'a str, s: &'a str) -> Option<(&'a str, &'a str)> {
    input
        .strip_prefix(s)
        .map(|stripped| (stripped, &input[0..s.len()]))
}

pub fn parse_until_chr(input: &'_ str, f: impl Fn(char) -> bool) -> Option<(&'_ str, &'_ str)> {
    let mut qty = 0;

    for chr in input.chars() {
        if f(chr) {
            break;
        }

        qty += chr.len_utf8();
    }

    if qty > 0 {
        Some((&input[qty..], &input[0..qty]))
    } else {
        None
    }
}

pub fn parse_whitespace0(input: &'_ str) -> (&'_ str, &'_ str) {
    parse_until_chr(input, |x| !x.is_whitespace()).unwrap_or((input, ""))
}

pub fn parse_separated_list<T>(
    input: &str,
    item: impl Fn(&str) -> Option<(&str, T)>,
    separator: impl Fn(&str) -> &str,
    trivia: impl Fn(&str) -> &str,
) -> (&str, Vec<T>) {
    let mut list = vec![];

    let mut input = input;
    loop {
        let s = trivia(input);

        let s = if let Some((s, item)) = item(s) {
            list.push(item);
            s
        } else {
            break;
        };

        let s = trivia(s);
        let s = separator(s);
        input = s;
    }

    (input, list)
}
