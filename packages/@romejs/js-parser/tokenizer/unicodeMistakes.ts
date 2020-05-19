/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export const UNICODE_MISTAKES: Map<string, [string, string]> = new Map();

UNICODE_MISTAKES.set("\u07fa", ["Nko Lajanyalan", "_"]);
UNICODE_MISTAKES.set("\ufe4d", ["Dashed Low Line", "_"]);
UNICODE_MISTAKES.set("\ufe4e", ["Centreline Low Line", "_"]);
UNICODE_MISTAKES.set("\ufe4f", ["Wavy Low Line", "_"]);
UNICODE_MISTAKES.set("\uff3f", ["Fullwidth Low Line", "_"]);

UNICODE_MISTAKES.set("\u2010", ["Hyphen", "-"]);
UNICODE_MISTAKES.set("\u2011", ["Non-Breaking Hyphen", "-"]);
UNICODE_MISTAKES.set("\u2012", ["Figure Dash", "-"]);
UNICODE_MISTAKES.set("\u2013", ["En Dash", "-"]);
UNICODE_MISTAKES.set("\u2014", ["Em Dash", "-"]);
UNICODE_MISTAKES.set("\ufe58", ["Small Em Dash", "-"]);
UNICODE_MISTAKES.set("\u06d4", ["Arabic Full Stop", "-"]);
UNICODE_MISTAKES.set("\u2043", ["Hyphen Bullet", "-"]);
UNICODE_MISTAKES.set("\u02d7", ["Modifier Letter Minus Sign", "-"]);
UNICODE_MISTAKES.set("\u2212", ["Minus Sign", "-"]);
UNICODE_MISTAKES.set("\u2796", ["Heavy Minus Sign", "-"]);
UNICODE_MISTAKES.set("\u2cba", ["Coptic Letter Dialect-P Ni", "-"]);
UNICODE_MISTAKES.set("\u30fc", ["Katakana-Hiragana Prolonged Sound Mark", "-"]);
UNICODE_MISTAKES.set("\uff0d", ["Fullwidth Hyphen-Minus", "-"]);
UNICODE_MISTAKES.set("\u2015", ["Horizontal Bar", "-"]);
UNICODE_MISTAKES.set("\u2500", ["Box Drawings Light Horizontal", "-"]);
UNICODE_MISTAKES.set("\u2501", ["Box Drawings Heavy Horizontal", "-"]);
UNICODE_MISTAKES.set("\u31d0", ["CJK Stroke H", "-"]);
UNICODE_MISTAKES.set("\ua7f7", ["Latin Epigraphic Letter Dideways", "-"]);
UNICODE_MISTAKES.set("\u1173", ["Hangul Jungseong Eu", "-"]);
UNICODE_MISTAKES.set("\u3161", ["Hangul Letter Eu", "-"]);
UNICODE_MISTAKES.set("\u4e00", ["CJK Unified Ideograph-4E00", "-"]);
UNICODE_MISTAKES.set("\u2f00", ["Kangxi Radical One", "-"]);

UNICODE_MISTAKES.set("\u060d", ["Arabic Date Separator", ","]);
UNICODE_MISTAKES.set("\u066b", ["Arabic Decimal Separator", ","]);
UNICODE_MISTAKES.set("\u201a", ["Single Low-9 Quotation Mark", ","]);
UNICODE_MISTAKES.set("\xb8", ["Cedilla", ","]);
UNICODE_MISTAKES.set("\ua4f9", ["Lisu Letter Tone Na Po", ","]);
UNICODE_MISTAKES.set("\uff0c", ["Fullwidth Comma", ","]);

UNICODE_MISTAKES.set("\u037e", ["Greek Question Mark", ";"]);
UNICODE_MISTAKES.set("\uff1b", ["Fullwidth Semicolon", ";"]);
UNICODE_MISTAKES.set(
	"\ufe14",
	["Presentation Form For Vertical Semicolon", ";"],
);

UNICODE_MISTAKES.set("\u0903", ["Devanagari Sign Visarga", ":"]);
UNICODE_MISTAKES.set("\u0a83", ["Gujarati Sign Visarga", ":"]);
UNICODE_MISTAKES.set("\uff1a", ["Fullwidth Colon", ":"]);
UNICODE_MISTAKES.set("\u0589", ["Armenian Full Stop", ":"]);
UNICODE_MISTAKES.set("\u0703", ["Syriac Supralinear Colon", ":"]);
UNICODE_MISTAKES.set("\u0704", ["Syriac Sublinear Colon", ":"]);
UNICODE_MISTAKES.set("\u16ec", ["Runic Multiple Punctuation", ":"]);
UNICODE_MISTAKES.set(
	"\ufe30",
	["Presentation Form For Vertical Two Dot Leader", ":"],
);
UNICODE_MISTAKES.set("\u1803", ["Mongolian Full Stop", ":"]);
UNICODE_MISTAKES.set("\u1809", ["Mongolian Manchu Full Stop", ":"]);
UNICODE_MISTAKES.set("\u205a", ["Two Dot Punctuation", ":"]);
UNICODE_MISTAKES.set("\u05c3", ["Hebrew Punctuation Sof Pasuq", ":"]);
UNICODE_MISTAKES.set("\u02f8", ["Modifier Letter Raised Colon", ":"]);
UNICODE_MISTAKES.set("\ua789", ["Modifier Letter Colon", ":"]);
UNICODE_MISTAKES.set("\u2236", ["Ratio", ":"]);
UNICODE_MISTAKES.set("\u02d0", ["Modifier Letter Triangular Colon", ":"]);
UNICODE_MISTAKES.set("\ua4fd", ["Lisu Letter Tone Mya Jeu", ":"]);
UNICODE_MISTAKES.set("\ufe13", ["Presentation Form For Vertical Colon", ":"]);

UNICODE_MISTAKES.set("\uff01", ["Fullwidth Exclamation Mark", "!"]);
UNICODE_MISTAKES.set("\u01c3", ["Latin Letter Retroflex Click", "!"]);
UNICODE_MISTAKES.set("\u2d51", ["Tifinagh Letter Tuareg Yang", "!"]);
UNICODE_MISTAKES.set(
	"\ufe15",
	["Presentation Form For Vertical Exclamation Mark", "!"],
);

UNICODE_MISTAKES.set("\u0294", ["Latin Letter Glottal Stop", "?"]);
UNICODE_MISTAKES.set("\u0241", ["Latin Capital Letter Glottal Stop", "?"]);
UNICODE_MISTAKES.set("\u097d", ["Devanagari Letter Glottal Stop", "?"]);
UNICODE_MISTAKES.set("\u13ae", ["Cherokee Letter He", "?"]);
UNICODE_MISTAKES.set("\ua6eb", ["Bamum Letter Ntuu", "?"]);
UNICODE_MISTAKES.set("\uff1f", ["Fullwidth Question Mark", "?"]);
UNICODE_MISTAKES.set(
	"\ufe16",
	["Presentation Form For Vertical Question Mark", "?"],
);

UNICODE_MISTAKES.set(
	"\u{1d16d}",
	["Musical Symbol Combining Augmentation Dot", "."],
);
UNICODE_MISTAKES.set("\u2024", ["One Dot Leader", "."]);
UNICODE_MISTAKES.set("\u0701", ["Syriac Supralinear Full Stop", "."]);
UNICODE_MISTAKES.set("\u0702", ["Syriac Sublinear Full Stop", "."]);
UNICODE_MISTAKES.set("\ua60e", ["Vai Full Stop", "."]);
UNICODE_MISTAKES.set("\u{10a50}", ["Kharoshthi Punctuation Dot", "."]);
UNICODE_MISTAKES.set("\u0660", ["Arabic-Indic Digit Zero", "."]);
UNICODE_MISTAKES.set("\u06f0", ["Extended Arabic-Indic Digit Zero", "."]);
UNICODE_MISTAKES.set("\ua4f8", ["Lisu Letter Tone Mya Ti", "."]);
UNICODE_MISTAKES.set("\xb7", ["Middle Dot", "."]);
UNICODE_MISTAKES.set("\u30fb", ["Katakana Middle Dot", "."]);
UNICODE_MISTAKES.set("\uff65", ["Halfwidth Katakana Middle Dot", "."]);
UNICODE_MISTAKES.set("\u16eb", ["Runic Single Punctuation", "."]);
UNICODE_MISTAKES.set("\u0387", ["Greek Ano Teleia", "."]);
UNICODE_MISTAKES.set("\u2e31", ["Word Separator Middle Dot", "."]);
UNICODE_MISTAKES.set("\u{10101}", ["Aegean Word Separator Dot", "."]);
UNICODE_MISTAKES.set("\u2022", ["Bullet", "."]);
UNICODE_MISTAKES.set("\u2027", ["Hyphenation Point", "."]);
UNICODE_MISTAKES.set("\u2219", ["Bullet Operator", "."]);
UNICODE_MISTAKES.set("\u22c5", ["Dot Operator", "."]);
UNICODE_MISTAKES.set("\ua78f", ["Latin Letter Sinological Dot", "."]);
UNICODE_MISTAKES.set("\u1427", ["Canadian Syllabics Final Middle Dot", "."]);
UNICODE_MISTAKES.set("\u1427", ["Canadian Syllabics Final Middle Dot", "."]);
UNICODE_MISTAKES.set("\uff0e", ["Fullwidth Full Stop", "."]);
UNICODE_MISTAKES.set("\u3002", ["Ideographic Full Stop", "."]);
UNICODE_MISTAKES.set(
	"\ufe12",
	["Presentation Form For Vertical Ideographic Full Stop", "."],
);

UNICODE_MISTAKES.set("\u055d", ["Armenian Comma", "'"]);
UNICODE_MISTAKES.set("\uff07", ["Fullwidth Apostrophe", "'"]);
UNICODE_MISTAKES.set("\u2018", ["Left Single Quotation Mark", "'"]);
UNICODE_MISTAKES.set("\u2019", ["Right Single Quotation Mark", "'"]);
UNICODE_MISTAKES.set("\u201b", ["Single High-Reversed-9 Quotation Mark", "'"]);
UNICODE_MISTAKES.set("\u2032", ["Prime", "'"]);
UNICODE_MISTAKES.set("\u2035", ["Reversed Prime", "'"]);
UNICODE_MISTAKES.set("\u055a", ["Armenian Apostrophe", "'"]);
UNICODE_MISTAKES.set("\u05f3", ["Hebrew Punctuation Geresh", "'"]);
UNICODE_MISTAKES.set("`", ["Grave Accent", "'"]);
UNICODE_MISTAKES.set("\u1fef", ["Greek Varia", "'"]);
UNICODE_MISTAKES.set("\uff40", ["Fullwidth Grave Accent", "'"]);
UNICODE_MISTAKES.set("\xb4", ["Acute Accent", "'"]);
UNICODE_MISTAKES.set("\u0384", ["Greek Tonos", "'"]);
UNICODE_MISTAKES.set("\u1ffd", ["Greek Oxia", "'"]);
UNICODE_MISTAKES.set("\u1fbd", ["Greek Koronis", "'"]);
UNICODE_MISTAKES.set("\u1fbf", ["Greek Psili", "'"]);
UNICODE_MISTAKES.set("\u1ffe", ["Greek Dasia", "'"]);
UNICODE_MISTAKES.set("\u02b9", ["Modifier Letter Prime", "'"]);
UNICODE_MISTAKES.set("\u0374", ["Greek Numeral Sign", "'"]);
UNICODE_MISTAKES.set("\u02c8", ["Modifier Letter Vertical Line", "'"]);
UNICODE_MISTAKES.set("\u02ca", ["Modifier Letter Acute Accent", "'"]);
UNICODE_MISTAKES.set("\u02cb", ["Modifier Letter Grave Accent", "'"]);
UNICODE_MISTAKES.set("\u02f4", ["Modifier Letter Middle Grave Accent", "'"]);
UNICODE_MISTAKES.set("\u02bb", ["Modifier Letter Turned Comma", "'"]);
UNICODE_MISTAKES.set("\u02bd", ["Modifier Letter Reversed Comma", "'"]);
UNICODE_MISTAKES.set("\u02bc", ["Modifier Letter Apostrophe", "'"]);
UNICODE_MISTAKES.set("\u02be", ["Modifier Letter Right Half Ring", "'"]);
UNICODE_MISTAKES.set("\ua78c", ["Latin Small Letter Saltillo", "'"]);
UNICODE_MISTAKES.set("\u05d9", ["Hebrew Letter Yod", "'"]);
UNICODE_MISTAKES.set("\u07f4", ["Nko High Tone Apostrophe", "'"]);
UNICODE_MISTAKES.set("\u07f5", ["Nko Low Tone Apostrophe", "'"]);
UNICODE_MISTAKES.set("\u144a", ["Canadian Syllabics West-Cree P", "'"]);
UNICODE_MISTAKES.set("\u16cc", ["Runic Letter Short-Twig-Sol S", "'"]);
UNICODE_MISTAKES.set("\u{16f51}", ["Miao Sign Aspiration", "'"]);
UNICODE_MISTAKES.set("\u{16f52}", ["Miao Sign Reformed Voicing", "'"]);

UNICODE_MISTAKES.set("\u1cd3", ["Vedic Sign Nihshvasa", '"']);
UNICODE_MISTAKES.set("\uff02", ["Fullwidth Quotation Mark", '"']);
UNICODE_MISTAKES.set("\u201c", ["Left Double Quotation Mark", '"']);
UNICODE_MISTAKES.set("\u201d", ["Right Double Quotation Mark", '"']);
UNICODE_MISTAKES.set("\u201f", ["Double High-Reversed-9 Quotation Mark", '"']);
UNICODE_MISTAKES.set("\u2033", ["Double Prime", '"']);
UNICODE_MISTAKES.set("\u2036", ["Reversed Double Prime", '"']);
UNICODE_MISTAKES.set("\u3003", ["Ditto Mark", '"']);
UNICODE_MISTAKES.set("\u05f4", ["Hebrew Punctuation Gershayim", '"']);
UNICODE_MISTAKES.set("\u02dd", ["Double Acute Accent", '"']);
UNICODE_MISTAKES.set("\u02ba", ["Modifier Letter Double Prime", '"']);
UNICODE_MISTAKES.set(
	"\u02f6",
	["Modifier Letter Middle Double Acute Accent", '"'],
);
UNICODE_MISTAKES.set(
	"\u02f5",
	["Modifier Letter Middle Double Grave Accent", '"'],
);
UNICODE_MISTAKES.set("\u02ee", ["Modifier Letter Double Apostrophe", '"']);
UNICODE_MISTAKES.set("\u05f2", ["Hebrew Ligature Yiddish Double Yod", '"']);
UNICODE_MISTAKES.set(
	"\u275e",
	["Heavy Double Comma Quotation Mark Ornament", '"'],
);
UNICODE_MISTAKES.set(
	"\u275d",
	["Heavy Double Turned Comma Quotation Mark Ornament", '"'],
);

UNICODE_MISTAKES.set("\uff08", ["Fullwidth Left Parenthesis", "("]);
UNICODE_MISTAKES.set("\u2768", ["Medium Left Parenthesis Ornament", "("]);
UNICODE_MISTAKES.set("\ufd3e", ["Ornate Left Parenthesis", "("]);

UNICODE_MISTAKES.set("\uff09", ["Fullwidth Right Parenthesis", ")"]);
UNICODE_MISTAKES.set("\u2769", ["Medium Right Parenthesis Ornament", ")"]);
UNICODE_MISTAKES.set("\ufd3f", ["Ornate Right Parenthesis", ")"]);

UNICODE_MISTAKES.set("\uff3b", ["Fullwidth Left Square Bracket", "["]);
UNICODE_MISTAKES.set(
	"\u2772",
	["Light Left Tortoise Shell Bracket Ornament", "["],
);
UNICODE_MISTAKES.set("\u300c", ["Left Corner Bracket", "["]);
UNICODE_MISTAKES.set("\u300e", ["Left White Corner Bracket", "["]);
UNICODE_MISTAKES.set("\u3010", ["Left Black Lenticular Bracket", "["]);
UNICODE_MISTAKES.set("\u3014", ["Left Tortoise Shell Bracket", "["]);
UNICODE_MISTAKES.set("\u3016", ["Left White Lenticular Bracket", "["]);
UNICODE_MISTAKES.set("\u3018", ["Left White Tortoise Shell Bracket", "["]);
UNICODE_MISTAKES.set("\u301a", ["Left White Square Bracket", "["]);

UNICODE_MISTAKES.set("\uff3d", ["Fullwidth Right Square Bracket", "]"]);
UNICODE_MISTAKES.set(
	"\u2773",
	["Light Right Tortoise Shell Bracket Ornament", "]"],
);
UNICODE_MISTAKES.set("\u300d", ["Right Corner Bracket", "]"]);
UNICODE_MISTAKES.set("\u300f", ["Right White Corner Bracket", "]"]);
UNICODE_MISTAKES.set("\u3011", ["Right Black Lenticular Bracket", "]"]);
UNICODE_MISTAKES.set("\u3015", ["Right Tortoise Shell Bracket", "]"]);
UNICODE_MISTAKES.set("\u3017", ["Right White Lenticular Bracket", "]"]);
UNICODE_MISTAKES.set("\u3019", ["Right White Tortoise Shell Bracket", "]"]);
UNICODE_MISTAKES.set("\u301b", ["Right White Square Bracket", "]"]);

UNICODE_MISTAKES.set("\u2774", ["Medium Left Curly Bracket Ornament", "{"]);
UNICODE_MISTAKES.set("\u{1d114}", ["Musical Symbol Brace", "{"]);
UNICODE_MISTAKES.set("\uff5b", ["Fullwidth Left Curly Bracket", "{"]);

UNICODE_MISTAKES.set("\u2775", ["Medium Right Curly Bracket Ornament", "}"]);
UNICODE_MISTAKES.set("\uff5d", ["Fullwidth Right Curly Bracket", "}"]);

UNICODE_MISTAKES.set("\u204e", ["Low Asterisk", "*"]);
UNICODE_MISTAKES.set("\u066d", ["Arabic Five Pointed Star", "*"]);
UNICODE_MISTAKES.set("\u2217", ["Asterisk Operator", "*"]);
UNICODE_MISTAKES.set("\u{1031f}", ["Old Italic Letter Ess", "*"]);
UNICODE_MISTAKES.set("\uff0a", ["Fullwidth Asterisk", "*"]);

UNICODE_MISTAKES.set("\u1735", ["Philippine Single Punctuation", "/"]);
UNICODE_MISTAKES.set("\u2041", ["Caret Insertion Point", "/"]);
UNICODE_MISTAKES.set("\u2215", ["Division Slash", "/"]);
UNICODE_MISTAKES.set("\u2044", ["Fraction Slash", "/"]);
UNICODE_MISTAKES.set(
	"\u2571",
	["Box Drawings Light Diagonal Upper Right To Lower Left", "/"],
);
UNICODE_MISTAKES.set("\u27cb", ["Mathematical Rising Diagonal", "/"]);
UNICODE_MISTAKES.set("\u29f8", ["Big Solidus", "/"]);
UNICODE_MISTAKES.set(
	"\u{1d23a}",
	["Greek Instrumental Notation Symbol-47", "/"],
);
UNICODE_MISTAKES.set("\u31d3", ["CJK Stroke Sp", "/"]);
UNICODE_MISTAKES.set("\u3033", ["Vertical Kana Repeat Mark Upper Half", "/"]);
UNICODE_MISTAKES.set("\u2cc6", ["Coptic Capital Letter Old Coptic Esh", "/"]);
UNICODE_MISTAKES.set("\u30ce", ["Katakana Letter No", "/"]);
UNICODE_MISTAKES.set("\u4e3f", ["CJK Unified Ideograph-4E3F", "/"]);
UNICODE_MISTAKES.set("\u2f03", ["Kangxi Radical Slash", "/"]);
UNICODE_MISTAKES.set("\uff0f", ["Fullwidth Solidus", "/"]);

UNICODE_MISTAKES.set("\uff3c", ["Fullwidth Reverse Solidus", "\\"]);
UNICODE_MISTAKES.set("\ufe68", ["Small Reverse Solidus", "\\"]);
UNICODE_MISTAKES.set("\u2216", ["Set Minus", "\\"]);
UNICODE_MISTAKES.set("\u27cd", ["Mathematical Falling Diagonal", "\\"]);
UNICODE_MISTAKES.set("\u29f5", ["Reverse Solidus Operator", "\\"]);
UNICODE_MISTAKES.set("\u29f9", ["Big Reverse Solidus", "\\"]);
UNICODE_MISTAKES.set("\u29f9", ["Greek Vocal Notation Symbol-16", "\\"]);
UNICODE_MISTAKES.set("\u29f9", ["Greek Instrumental Symbol-48", "\\"]);
UNICODE_MISTAKES.set("\u31d4", ["CJK Stroke D", "\\"]);
UNICODE_MISTAKES.set("\u4e36", ["CJK Unified Ideograph-4E36", "\\"]);
UNICODE_MISTAKES.set("\u2f02", ["Kangxi Radical Dot", "\\"]);
UNICODE_MISTAKES.set("\u3001", ["Ideographic Comma", "\\"]);
UNICODE_MISTAKES.set("\u30fd", ["Katakana Iteration Mark", "\\"]);

UNICODE_MISTAKES.set("\ua778", ["Latin Small Letter Um", "&"]);
UNICODE_MISTAKES.set("\uff06", ["Fullwidth Ampersand", "&"]);

UNICODE_MISTAKES.set("\u16ed", ["Runic Cross Punctuation", "+"]);
UNICODE_MISTAKES.set("\u2795", ["Heavy Plus Sign", "+"]);
UNICODE_MISTAKES.set("\u{1029b}", ["Lycian Letter H", "+"]);
UNICODE_MISTAKES.set("\ufb29", ["Hebrew Letter Alternative Plus Sign", "+"]);
UNICODE_MISTAKES.set("\uff0b", ["Fullwidth Plus Sign", "+"]);

UNICODE_MISTAKES.set(
	"\u2039",
	["Single Left-Pointing Angle Quotation Mark", "<"],
);
UNICODE_MISTAKES.set(
	"\u276e",
	["Heavy Left-Pointing Angle Quotation Mark Ornament", "<"],
);
UNICODE_MISTAKES.set("\u02c2", ["Modifier Letter Left Arrowhead", "<"]);
UNICODE_MISTAKES.set("\u{1d236}", ["Greek Instrumental Symbol-40", "<"]);
UNICODE_MISTAKES.set("\u1438", ["Canadian Syllabics Pa", "<"]);
UNICODE_MISTAKES.set("\u16b2", ["Runic Letter Kauna", "<"]);
UNICODE_MISTAKES.set(
	"\u276c",
	["Medium Left-Pointing Angle Bracket Ornament", "<"],
);
UNICODE_MISTAKES.set("\u27e8", ["Mathematical Left Angle Bracket", "<"]);
UNICODE_MISTAKES.set("\u2329", ["Left-Pointing Angle Bracket", "<"]);
UNICODE_MISTAKES.set("\u3008", ["Left Angle Bracket", "<"]);
UNICODE_MISTAKES.set("\u31db", ["CJK Stroke Pd", "<"]);
UNICODE_MISTAKES.set("\u304f", ["Hiragana Letter Ku", "<"]);
UNICODE_MISTAKES.set("\u{21fe8}", ["CJK Unified Ideograph-21FE8", "<"]);
UNICODE_MISTAKES.set("\u300a", ["Left Double Angle Bracket", "<"]);
UNICODE_MISTAKES.set("\uff1c", ["Fullwidth Less-Than Sign", "<"]);

UNICODE_MISTAKES.set("\u1400", ["Canadian Syllabics Hyphen", "="]);
UNICODE_MISTAKES.set("\u2e40", ["Double Hyphen", "="]);
UNICODE_MISTAKES.set("\u30a0", ["Katakana-Hiragana Double Hyphen", "="]);
UNICODE_MISTAKES.set("\ua4ff", ["Lisu Punctuation Full Stop", "="]);
UNICODE_MISTAKES.set("\uff1d", ["Fullwidth Equals Sign", "="]);

UNICODE_MISTAKES.set(
	"\u203a",
	["Single Right-Pointing Angle Quotation Mark", ">"],
);
UNICODE_MISTAKES.set(
	"\u276f",
	["Heavy Right-Pointing Angle Quotation Mark Ornament", ">"],
);
UNICODE_MISTAKES.set("\u02c3", ["Modifier Letter Right Arrowhead", ">"]);
UNICODE_MISTAKES.set("\u{1d237}", ["Greek Instrumental Symbol-42", ">"]);
UNICODE_MISTAKES.set("\u1433", ["Canadian Syllabics Po", ">"]);
UNICODE_MISTAKES.set("\u{16f3f}", ["Miao Letter Archaic Zza", ">"]);
UNICODE_MISTAKES.set(
	"\u276d",
	["Medium Right-Pointing Angle Bracket Ornament", ">"],
);
UNICODE_MISTAKES.set("\u27e9", ["Mathematical Right Angle Bracket", ">"]);
UNICODE_MISTAKES.set("\u232a", ["Right-Pointing Angle Bracket", ">"]);
UNICODE_MISTAKES.set("\u3009", ["Right Angle Bracket", ">"]);
UNICODE_MISTAKES.set("\u300b", ["Right Double Angle Bracket", ">"]);
UNICODE_MISTAKES.set("\uff1e", ["Fullwidth Greater-Than Sign", ">"]);

export const ASCII_NAMES: Map<string, string> = new Map();
ASCII_NAMES.set(" ", "Space");
ASCII_NAMES.set("_", "Underscore");
ASCII_NAMES.set("-", "Minus/Hyphen");
ASCII_NAMES.set(",", "Comma");
ASCII_NAMES.set(";", "Semicolon");
ASCII_NAMES.set(":", "Colon");
ASCII_NAMES.set("!", "Exclamation Mark");
ASCII_NAMES.set("?", "Question Mark");
ASCII_NAMES.set(".", "Period");
ASCII_NAMES.set("'", "Single Quote");
ASCII_NAMES.set('"', "Quotation Mark");
ASCII_NAMES.set("(", "Left Parenthesis");
ASCII_NAMES.set(")", "Right Parenthesis");
ASCII_NAMES.set("[", "Left Square Bracket");
ASCII_NAMES.set("]", "Right Square Bracket");
ASCII_NAMES.set("{", "Left Curly Brace");
ASCII_NAMES.set("}", "Right Curly Brace");
ASCII_NAMES.set("*", "Asterisk");
ASCII_NAMES.set("/", "Slash");
ASCII_NAMES.set("\\", "Backslash");
ASCII_NAMES.set("&", "Ampersand");
ASCII_NAMES.set("+", "Plus Sign");
ASCII_NAMES.set("<", "Less-Than Sign");
ASCII_NAMES.set("=", "Equals Sign");
ASCII_NAMES.set(">", "Greater-Than Sign");
