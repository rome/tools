use rome_js_syntax::JsAnyExpression;
///
/// ```rust
/// use rome_js_analyze::utils::natural_compare;
/// assert_eq!(natural_compare("1", "2", true), -1);
/// assert_eq!(natural_compare("100", "2", true), 2);
/// assert_eq!(natural_compare("-100", "2", true), -5);
/// assert_eq!(natural_compare("007", "8", true), -1);
/// assert_eq!(natural_compare("007", "7", true), 2);
/// assert_eq!(natural_compare("test1", "9000", true), 59);
/// assert_eq!(natural_compare("1testrome", "2t", true), -1);
/// assert_eq!(natural_compare("1test", "1TEST", true), 0);
/// assert_eq!(natural_compare("1test", "1TEST", false), 32);
/// ```
pub fn natural_compare(a: &str, b: &str, intensive: bool) -> i32 {
    let (a, b): (Vec<char>, Vec<char>) = if intensive {
        (
            a.to_lowercase().chars().collect(),
            b.to_lowercase().chars().collect(),
        )
    } else {
        (a.chars().collect(), b.chars().collect())
    };

    let len_a = a.len();
    let len_b = b.len();

    let mut a_index = 0usize;
    let mut b_index = 0usize;

    while a_index < len_a && b_index < len_b {
        let mut char_code_a = a[a_index];
        let mut char_code_b = b[b_index];

        if char_code_a.is_ascii_digit() {
            if !char_code_b.is_ascii_digit() {
                return char_code_a as i32 - char_code_b as i32;
            }

            let mut num_start_a = a_index;
            let mut num_start_b = b_index;

            while char_code_a as u32 == 48 {
                num_start_a += 1;
                if num_start_a >= len_a {
                    break;
                }
                char_code_a = a[num_start_a];
            }

            while char_code_b as u32 == 48 {
                num_start_b += 1;
                if num_start_b >= len_b {
                    break;
                }
                char_code_b = b[num_start_b];
            }

            println!("{}", char_code_a);
            println!("{}", char_code_b);
            let mut num_end_a = num_start_a;
            let mut num_end_b = num_start_b;
            while num_end_a < len_a && a[num_end_a].is_ascii_digit() {
                num_end_a += 1;
            }

            while num_end_b < len_b && b[num_end_b].is_ascii_digit() {
                num_end_b += 1;
            }

            println!("{}, {}", num_end_a, num_end_b);
            let mut difference =
                num_end_a as i32 - num_start_a as i32 - num_end_b as i32 + num_start_b as i32;
            println!("{}", difference);
            if difference != 0 {
                return difference;
            }

            while num_start_a < num_end_a {
                difference = a[num_start_a] as i32 - b[num_start_b] as i32;
                num_start_a += 1;
                num_start_b += 1;
                if difference != 0 {
                    return difference;
                }
            }

            a_index = num_end_a;
            b_index = num_end_b;
            continue;
        }

        if char_code_a != char_code_b {
            return char_code_a as i32 - char_code_b as i32;
        }

        a_index += 1;
        b_index += 1;
    }
    return len_a as i32 - len_b as i32;
}


pub fn is_boolean_literal(expr: JsAnyExpression) -> bool {
    matches!(
        expr,
        JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsBooleanLiteralExpression(_)
        )
    )
}
