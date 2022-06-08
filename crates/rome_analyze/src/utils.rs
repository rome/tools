// export function naturalCompare(
// 	a: string,
// 	b: string,
// 	insensitive: boolean = true,
// ): number {
// 	if (insensitive) {
// 		a = a.toLowerCase();
// 		b = b.toLowerCase();
// 	}

// 	const lengthA = a.length;
// 	const lengthB = b.length;

// 	let aIndex: number = 0;
// 	let bIndex: number = 0;

// 	while (aIndex < lengthA && bIndex < lengthB) {
// 		let charCodeA = a.charCodeAt(aIndex);
// 		let charCodeB = b.charCodeAt(bIndex);

// 		if (isDigit(charCodeA)) {
// 			if (!isDigit(charCodeB)) {
// 				return charCodeA - charCodeB;
// 			}

// 			let numStartA = aIndex;
// 			let numStartB = bIndex;

// 			while (charCodeA === 48 && ++numStartA < lengthA) {
// 				charCodeA = a.charCodeAt(numStartA);
// 			}
// 			while (charCodeB === 48 && ++numStartB < lengthB) {
// 				charCodeB = b.charCodeAt(numStartB);
// 			}

// 			let numEndA = numStartA;
// 			let numEndB = numStartB;

// 			while (numEndA < lengthA && isDigit(a.charCodeAt(numEndA))) {
// 				++numEndA;
// 			}
// 			while (numEndB < lengthB && isDigit(b.charCodeAt(numEndB))) {
// 				++numEndB;
// 			}

// 			let difference = numEndA - numStartA - numEndB + numStartB; // numA length - numB length
// 			if (difference) {
// 				return difference;
// 			}

// 			while (numStartA < numEndA) {
// 				difference = a.charCodeAt(numStartA++) - b.charCodeAt(numStartB++);
// 				if (difference) {
// 					return difference;
// 				}
// 			}

// 			aIndex = numEndA;
// 			bIndex = numEndB;
// 			continue;
// 		}

// 		if (charCodeA !== charCodeB) {
// 			return charCodeA - charCodeB;
// 		}

// 		++aIndex;
// 		++bIndex;
// 	}

// 	return lengthA - lengthB;
// }

use std::borrow::Cow;

// test(
// 	"naturalCompare",
// 	(t) => {
// 		t.is(naturalCompare("1", "2"), -1);
// 		t.is(naturalCompare("100", "2"), 2);
// 		t.is(naturalCompare("-100", "2"), -5);
// 		t.is(naturalCompare("007", "8"), -1);
// 		t.is(naturalCompare("007", "7"), 2);
// 		t.is(naturalCompare("test1", "9000"), 59);
// 		t.is(naturalCompare("1testrome", "2t"), -1);
// 		t.is(naturalCompare("1test", "1TEST"), 0);
// 		t.is(naturalCompare("1test", "1TEST", false), 32);
// 	},
// );
///
/// ```rust
/// use rome_analyze::natural_compare;
/// // assert_eq!(natural_compare("1", "2", false), -1);
/// assert_eq!(natural_compare("100", "2", false), 2);
/// assert_eq!(natural_compare("-100", "2", false), -5);
/// //assert_eq!(natural_compare("007", "8", false), -1);
/// assert_eq!(natural_compare("007", "7", false), -2);
/// ```
pub fn natural_compare(a: &str, b: &str, intensive: bool) -> i32 {
    let mut a = Cow::Borrowed(a);
    let mut b = Cow::Borrowed(b);
    if intensive {
        a = Cow::Owned(a.to_lowercase());
        b = Cow::Owned(b.to_lowercase());
    }
    let len_a = a.len();
    let len_b = b.len();

    let mut a_iter = a.chars();
    let mut b_iter = b.chars();

    while !matches!(a_iter.size_hint().1, Some(0)) && !matches!(b_iter.size_hint().1, Some(0)) {
        // SAFETY: we check `a_iter` is some and `b_iter` is some before we begin the new iteration
        let mut char_code_a = a_iter.next().unwrap();
        let mut char_code_b = b_iter.next().unwrap();

        if char_code_a.is_ascii_digit() {
            if !char_code_b.is_ascii_digit() {
                return char_code_a as i32 - char_code_b as i32;
            }

            let mut iter_start_a = a_iter.clone();
            let mut iter_start_b = b_iter.clone();

            while char_code_a as u32 == 48 && !matches!(iter_start_a.size_hint().1, Some(1)) {
                // SAFETY: we check `iter_start_a` is some before we begin the new iteration
                char_code_a = iter_start_a.next().unwrap();
            }

            while char_code_b as u32 == 48 && !matches!(iter_start_b.size_hint().1, Some(1)) {
                // SAFETY: we check `iter_start_b` is some before we begin the new iteration
                char_code_b = iter_start_b.next().unwrap();
            }

            let mut iter_end_a = iter_start_a.clone();
            let mut iter_end_b = iter_start_b.clone();

            while !matches!(iter_end_a.size_hint().1, Some(0))
                && iter_end_a.clone().next().unwrap().is_ascii_digit()
            {
                iter_end_a.next();
            }

            while !matches!(iter_end_b.size_hint().1, Some(0))
                && iter_end_b.clone().next().unwrap().is_ascii_digit()
            {
                iter_end_b.next();
            }

            let mut difference = (len_a - iter_end_a.size_hint().1.unwrap()) as i32
                - (len_a - iter_start_a.size_hint().1.unwrap()) as i32
                - (len_b - iter_end_b.size_hint().1.unwrap()) as i32
                + (len_b - iter_start_b.size_hint().1.unwrap()) as i32;

            if difference != 0{
                return difference;
            }

            while iter_start_a.size_hint().1.unwrap() > iter_end_a.size_hint().1.unwrap() {
                difference = iter_start_a.next().unwrap() as i32 - iter_start_b.next().unwrap() as i32;
                if difference != 0 {
                    return difference;
                }
            }

            a_iter = iter_end_a;
            b_iter = iter_end_b;
            continue;
        }

        if char_code_a != char_code_b {
            return char_code_a as i32 - char_code_b as i32;
        }
        a_iter.next();
        b_iter.next();
    }
    len_a as i32 - len_b as i32
}
