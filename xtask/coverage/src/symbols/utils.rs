pub fn parse_str<'a>(input: &'a str, s: &'a str) -> Option<(&'a str, &'a str)> {
    if input.starts_with(s) {
        Some((&input[s.len()..], &input[0..s.len()]))
    } else {
        None
    }
}

pub fn parse_until_chr<'a>(input: &'a str, f: impl Fn(char) -> bool) -> Option<(&'a str, &'a str)> {
    let mut qty = 0;

    let mut chars = input.chars();
    while let Some(chr) = chars.next() {
        if f(chr) {
            break;
        }

        qty += 1;
    }

    if qty > 0 {
        Some((&input[qty..], &input[0..qty]))
    } else {
        None
    }
}

pub fn parse_whitespace0<'a>(input: &'a str) -> (&'a str, &'a str) {
    parse_until_chr(input, |x| !x.is_whitespace()).unwrap_or((input, ""))
}

pub fn parse_separated_list<T>(
    input: &str,
    item: impl Fn(&str) -> Option<(&str, T)>,
    separator: impl Fn(&str) -> &str,
    trivia: impl Fn(&str) -> &str,
) -> (&str, Vec<T>) {
    let mut qty = 0;
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

    (&input[..qty], list)
}
