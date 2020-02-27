/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export const UNICODE_MISTAKES: Map<string, [string, string]> = new Map();

UNICODE_MISTAKES.set('ߺ', ['Nko Lajanyalan', '_']);
UNICODE_MISTAKES.set('﹍', ['Dashed Low Line', '_']);
UNICODE_MISTAKES.set('﹎', ['Centreline Low Line', '_']);
UNICODE_MISTAKES.set('﹏', ['Wavy Low Line', '_']);
UNICODE_MISTAKES.set('＿', ['Fullwidth Low Line', '_']);

UNICODE_MISTAKES.set('‐', ['Hyphen', '-']);
UNICODE_MISTAKES.set('‑', ['Non-Breaking Hyphen', '-']);
UNICODE_MISTAKES.set('‒', ['Figure Dash', '-']);
UNICODE_MISTAKES.set('–', ['En Dash', '-']);
UNICODE_MISTAKES.set('—', ['Em Dash', '-']);
UNICODE_MISTAKES.set('﹘', ['Small Em Dash', '-']);
UNICODE_MISTAKES.set('۔', ['Arabic Full Stop', '-']);
UNICODE_MISTAKES.set('⁃', ['Hyphen Bullet', '-']);
UNICODE_MISTAKES.set('˗', ['Modifier Letter Minus Sign', '-']);
UNICODE_MISTAKES.set('−', ['Minus Sign', '-']);
UNICODE_MISTAKES.set('➖', ['Heavy Minus Sign', '-']);
UNICODE_MISTAKES.set('Ⲻ', ['Coptic Letter Dialect-P Ni', '-']);
UNICODE_MISTAKES.set('ー', ['Katakana-Hiragana Prolonged Sound Mark', '-']);
UNICODE_MISTAKES.set('－', ['Fullwidth Hyphen-Minus', '-']);
UNICODE_MISTAKES.set('―', ['Horizontal Bar', '-']);
UNICODE_MISTAKES.set('─', ['Box Drawings Light Horizontal', '-']);
UNICODE_MISTAKES.set('━', ['Box Drawings Heavy Horizontal', '-']);
UNICODE_MISTAKES.set('㇐', ['CJK Stroke H', '-']);
UNICODE_MISTAKES.set('ꟷ', ['Latin Epigraphic Letter Dideways', '-']);
UNICODE_MISTAKES.set('ᅳ', ['Hangul Jungseong Eu', '-']);
UNICODE_MISTAKES.set('ㅡ', ['Hangul Letter Eu', '-']);
UNICODE_MISTAKES.set('一', ['CJK Unified Ideograph-4E00', '-']);
UNICODE_MISTAKES.set('⼀', ['Kangxi Radical One', '-']);

UNICODE_MISTAKES.set('؍', ['Arabic Date Separator', ',']);
UNICODE_MISTAKES.set('٫', ['Arabic Decimal Separator', ',']);
UNICODE_MISTAKES.set('‚', ['Single Low-9 Quotation Mark', ',']);
UNICODE_MISTAKES.set('¸', ['Cedilla', ',']);
UNICODE_MISTAKES.set('ꓹ', ['Lisu Letter Tone Na Po', ',']);
UNICODE_MISTAKES.set('，', ['Fullwidth Comma', ',']);

UNICODE_MISTAKES.set(';', ['Greek Question Mark', ';']);
UNICODE_MISTAKES.set('；', ['Fullwidth Semicolon', ';']);
UNICODE_MISTAKES.set('︔', ['Presentation Form For Vertical Semicolon', ';']);

UNICODE_MISTAKES.set('ः', ['Devanagari Sign Visarga', ':']);
UNICODE_MISTAKES.set('ઃ', ['Gujarati Sign Visarga', ':']);
UNICODE_MISTAKES.set('：', ['Fullwidth Colon', ':']);
UNICODE_MISTAKES.set('։', ['Armenian Full Stop', ':']);
UNICODE_MISTAKES.set('܃', ['Syriac Supralinear Colon', ':']);
UNICODE_MISTAKES.set('܄', ['Syriac Sublinear Colon', ':']);
UNICODE_MISTAKES.set('᛬', ['Runic Multiple Punctuation', ':']);
UNICODE_MISTAKES.set('︰', [
  'Presentation Form For Vertical Two Dot Leader',
  ':',
]);
UNICODE_MISTAKES.set('᠃', ['Mongolian Full Stop', ':']);
UNICODE_MISTAKES.set('᠉', ['Mongolian Manchu Full Stop', ':']);
UNICODE_MISTAKES.set('⁚', ['Two Dot Punctuation', ':']);
UNICODE_MISTAKES.set('׃', ['Hebrew Punctuation Sof Pasuq', ':']);
UNICODE_MISTAKES.set('˸', ['Modifier Letter Raised Colon', ':']);
UNICODE_MISTAKES.set('꞉', ['Modifier Letter Colon', ':']);
UNICODE_MISTAKES.set('∶', ['Ratio', ':']);
UNICODE_MISTAKES.set('ː', ['Modifier Letter Triangular Colon', ':']);
UNICODE_MISTAKES.set('ꓽ', ['Lisu Letter Tone Mya Jeu', ':']);
UNICODE_MISTAKES.set('︓', ['Presentation Form For Vertical Colon', ':']);

UNICODE_MISTAKES.set('！', ['Fullwidth Exclamation Mark', '!']);
UNICODE_MISTAKES.set('ǃ', ['Latin Letter Retroflex Click', '!']);
UNICODE_MISTAKES.set('ⵑ', ['Tifinagh Letter Tuareg Yang', '!']);
UNICODE_MISTAKES.set('︕', [
  'Presentation Form For Vertical Exclamation Mark',
  '!',
]);

UNICODE_MISTAKES.set('ʔ', ['Latin Letter Glottal Stop', '?']);
UNICODE_MISTAKES.set('Ɂ', ['Latin Capital Letter Glottal Stop', '?']);
UNICODE_MISTAKES.set('ॽ', ['Devanagari Letter Glottal Stop', '?']);
UNICODE_MISTAKES.set('Ꭾ', ['Cherokee Letter He', '?']);
UNICODE_MISTAKES.set('ꛫ', ['Bamum Letter Ntuu', '?']);
UNICODE_MISTAKES.set('？', ['Fullwidth Question Mark', '?']);
UNICODE_MISTAKES.set('︖', [
  'Presentation Form For Vertical Question Mark',
  '?',
]);

UNICODE_MISTAKES.set('𝅭', ['Musical Symbol Combining Augmentation Dot', '.']);
UNICODE_MISTAKES.set('․', ['One Dot Leader', '.']);
UNICODE_MISTAKES.set('܁', ['Syriac Supralinear Full Stop', '.']);
UNICODE_MISTAKES.set('܂', ['Syriac Sublinear Full Stop', '.']);
UNICODE_MISTAKES.set('꘎', ['Vai Full Stop', '.']);
UNICODE_MISTAKES.set('𐩐', ['Kharoshthi Punctuation Dot', '.']);
UNICODE_MISTAKES.set('٠', ['Arabic-Indic Digit Zero', '.']);
UNICODE_MISTAKES.set('۰', ['Extended Arabic-Indic Digit Zero', '.']);
UNICODE_MISTAKES.set('ꓸ', ['Lisu Letter Tone Mya Ti', '.']);
UNICODE_MISTAKES.set('·', ['Middle Dot', '.']);
UNICODE_MISTAKES.set('・', ['Katakana Middle Dot', '.']);
UNICODE_MISTAKES.set('･', ['Halfwidth Katakana Middle Dot', '.']);
UNICODE_MISTAKES.set('᛫', ['Runic Single Punctuation', '.']);
UNICODE_MISTAKES.set('·', ['Greek Ano Teleia', '.']);
UNICODE_MISTAKES.set('⸱', ['Word Separator Middle Dot', '.']);
UNICODE_MISTAKES.set('𐄁', ['Aegean Word Separator Dot', '.']);
UNICODE_MISTAKES.set('•', ['Bullet', '.']);
UNICODE_MISTAKES.set('‧', ['Hyphenation Point', '.']);
UNICODE_MISTAKES.set('∙', ['Bullet Operator', '.']);
UNICODE_MISTAKES.set('⋅', ['Dot Operator', '.']);
UNICODE_MISTAKES.set('ꞏ', ['Latin Letter Sinological Dot', '.']);
UNICODE_MISTAKES.set('ᐧ', ['Canadian Syllabics Final Middle Dot', '.']);
UNICODE_MISTAKES.set('ᐧ', ['Canadian Syllabics Final Middle Dot', '.']);
UNICODE_MISTAKES.set('．', ['Fullwidth Full Stop', '.']);
UNICODE_MISTAKES.set('。', ['Ideographic Full Stop', '.']);
UNICODE_MISTAKES.set('︒', [
  'Presentation Form For Vertical Ideographic Full Stop',
  '.',
]);

UNICODE_MISTAKES.set('՝', ['Armenian Comma', "'"]);
UNICODE_MISTAKES.set('＇', ['Fullwidth Apostrophe', "'"]);
UNICODE_MISTAKES.set('‘', ['Left Single Quotation Mark', "'"]);
UNICODE_MISTAKES.set('’', ['Right Single Quotation Mark', "'"]);
UNICODE_MISTAKES.set('‛', ['Single High-Reversed-9 Quotation Mark', "'"]);
UNICODE_MISTAKES.set('′', ['Prime', "'"]);
UNICODE_MISTAKES.set('‵', ['Reversed Prime', "'"]);
UNICODE_MISTAKES.set('՚', ['Armenian Apostrophe', "'"]);
UNICODE_MISTAKES.set('׳', ['Hebrew Punctuation Geresh', "'"]);
UNICODE_MISTAKES.set('`', ['Grave Accent', "'"]);
UNICODE_MISTAKES.set('`', ['Greek Varia', "'"]);
UNICODE_MISTAKES.set('｀', ['Fullwidth Grave Accent', "'"]);
UNICODE_MISTAKES.set('´', ['Acute Accent', "'"]);
UNICODE_MISTAKES.set('΄', ['Greek Tonos', "'"]);
UNICODE_MISTAKES.set('´', ['Greek Oxia', "'"]);
UNICODE_MISTAKES.set('᾽', ['Greek Koronis', "'"]);
UNICODE_MISTAKES.set('᾿', ['Greek Psili', "'"]);
UNICODE_MISTAKES.set('῾', ['Greek Dasia', "'"]);
UNICODE_MISTAKES.set('ʹ', ['Modifier Letter Prime', "'"]);
UNICODE_MISTAKES.set('ʹ', ['Greek Numeral Sign', "'"]);
UNICODE_MISTAKES.set('ˈ', ['Modifier Letter Vertical Line', "'"]);
UNICODE_MISTAKES.set('ˊ', ['Modifier Letter Acute Accent', "'"]);
UNICODE_MISTAKES.set('ˋ', ['Modifier Letter Grave Accent', "'"]);
UNICODE_MISTAKES.set('˴', ['Modifier Letter Middle Grave Accent', "'"]);
UNICODE_MISTAKES.set('ʻ', ['Modifier Letter Turned Comma', "'"]);
UNICODE_MISTAKES.set('ʽ', ['Modifier Letter Reversed Comma', "'"]);
UNICODE_MISTAKES.set('ʼ', ['Modifier Letter Apostrophe', "'"]);
UNICODE_MISTAKES.set('ʾ', ['Modifier Letter Right Half Ring', "'"]);
UNICODE_MISTAKES.set('ꞌ', ['Latin Small Letter Saltillo', "'"]);
UNICODE_MISTAKES.set('י', ['Hebrew Letter Yod', "'"]);
UNICODE_MISTAKES.set('ߴ', ['Nko High Tone Apostrophe', "'"]);
UNICODE_MISTAKES.set('ߵ', ['Nko Low Tone Apostrophe', "'"]);
UNICODE_MISTAKES.set('ᑊ', ['Canadian Syllabics West-Cree P', "'"]);
UNICODE_MISTAKES.set('ᛌ', ['Runic Letter Short-Twig-Sol S', "'"]);
UNICODE_MISTAKES.set('𖽑', ['Miao Sign Aspiration', "'"]);
UNICODE_MISTAKES.set('𖽒', ['Miao Sign Reformed Voicing', "'"]);

UNICODE_MISTAKES.set('᳓', ['Vedic Sign Nihshvasa', '"']);
UNICODE_MISTAKES.set('＂', ['Fullwidth Quotation Mark', '"']);
UNICODE_MISTAKES.set('“', ['Left Double Quotation Mark', '"']);
UNICODE_MISTAKES.set('”', ['Right Double Quotation Mark', '"']);
UNICODE_MISTAKES.set('‟', ['Double High-Reversed-9 Quotation Mark', '"']);
UNICODE_MISTAKES.set('″', ['Double Prime', '"']);
UNICODE_MISTAKES.set('‶', ['Reversed Double Prime', '"']);
UNICODE_MISTAKES.set('〃', ['Ditto Mark', '"']);
UNICODE_MISTAKES.set('״', ['Hebrew Punctuation Gershayim', '"']);
UNICODE_MISTAKES.set('˝', ['Double Acute Accent', '"']);
UNICODE_MISTAKES.set('ʺ', ['Modifier Letter Double Prime', '"']);
UNICODE_MISTAKES.set('˶', ['Modifier Letter Middle Double Acute Accent', '"']);
UNICODE_MISTAKES.set('˵', ['Modifier Letter Middle Double Grave Accent', '"']);
UNICODE_MISTAKES.set('ˮ', ['Modifier Letter Double Apostrophe', '"']);
UNICODE_MISTAKES.set('ײ', ['Hebrew Ligature Yiddish Double Yod', '"']);
UNICODE_MISTAKES.set('❞', ['Heavy Double Comma Quotation Mark Ornament', '"']);
UNICODE_MISTAKES.set('❝', [
  'Heavy Double Turned Comma Quotation Mark Ornament',
  '"',
]);

UNICODE_MISTAKES.set('（', ['Fullwidth Left Parenthesis', '(']);
UNICODE_MISTAKES.set('❨', ['Medium Left Parenthesis Ornament', '(']);
UNICODE_MISTAKES.set('﴾', ['Ornate Left Parenthesis', '(']);

UNICODE_MISTAKES.set('）', ['Fullwidth Right Parenthesis', ')']);
UNICODE_MISTAKES.set('❩', ['Medium Right Parenthesis Ornament', ')']);
UNICODE_MISTAKES.set('﴿', ['Ornate Right Parenthesis', ')']);

UNICODE_MISTAKES.set('［', ['Fullwidth Left Square Bracket', '[']);
UNICODE_MISTAKES.set('❲', ['Light Left Tortoise Shell Bracket Ornament', '[']);
UNICODE_MISTAKES.set('「', ['Left Corner Bracket', '[']);
UNICODE_MISTAKES.set('『', ['Left White Corner Bracket', '[']);
UNICODE_MISTAKES.set('【', ['Left Black Lenticular Bracket', '[']);
UNICODE_MISTAKES.set('〔', ['Left Tortoise Shell Bracket', '[']);
UNICODE_MISTAKES.set('〖', ['Left White Lenticular Bracket', '[']);
UNICODE_MISTAKES.set('〘', ['Left White Tortoise Shell Bracket', '[']);
UNICODE_MISTAKES.set('〚', ['Left White Square Bracket', '[']);

UNICODE_MISTAKES.set('］', ['Fullwidth Right Square Bracket', ']']);
UNICODE_MISTAKES.set('❳', ['Light Right Tortoise Shell Bracket Ornament', ']']);
UNICODE_MISTAKES.set('」', ['Right Corner Bracket', ']']);
UNICODE_MISTAKES.set('』', ['Right White Corner Bracket', ']']);
UNICODE_MISTAKES.set('】', ['Right Black Lenticular Bracket', ']']);
UNICODE_MISTAKES.set('〕', ['Right Tortoise Shell Bracket', ']']);
UNICODE_MISTAKES.set('〗', ['Right White Lenticular Bracket', ']']);
UNICODE_MISTAKES.set('〙', ['Right White Tortoise Shell Bracket', ']']);
UNICODE_MISTAKES.set('〛', ['Right White Square Bracket', ']']);

UNICODE_MISTAKES.set('❴', ['Medium Left Curly Bracket Ornament', '{']);
UNICODE_MISTAKES.set('𝄔', ['Musical Symbol Brace', '{']);
UNICODE_MISTAKES.set('｛', ['Fullwidth Left Curly Bracket', '{']);

UNICODE_MISTAKES.set('❵', ['Medium Right Curly Bracket Ornament', '}']);
UNICODE_MISTAKES.set('｝', ['Fullwidth Right Curly Bracket', '}']);

UNICODE_MISTAKES.set('⁎', ['Low Asterisk', '*']);
UNICODE_MISTAKES.set('٭', ['Arabic Five Pointed Star', '*']);
UNICODE_MISTAKES.set('∗', ['Asterisk Operator', '*']);
UNICODE_MISTAKES.set('𐌟', ['Old Italic Letter Ess', '*']);
UNICODE_MISTAKES.set('＊', ['Fullwidth Asterisk', '*']);

UNICODE_MISTAKES.set('᜵', ['Philippine Single Punctuation', '/']);
UNICODE_MISTAKES.set('⁁', ['Caret Insertion Point', '/']);
UNICODE_MISTAKES.set('∕', ['Division Slash', '/']);
UNICODE_MISTAKES.set('⁄', ['Fraction Slash', '/']);
UNICODE_MISTAKES.set('╱', [
  'Box Drawings Light Diagonal Upper Right To Lower Left',
  '/',
]);
UNICODE_MISTAKES.set('⟋', ['Mathematical Rising Diagonal', '/']);
UNICODE_MISTAKES.set('⧸', ['Big Solidus', '/']);
UNICODE_MISTAKES.set('𝈺', ['Greek Instrumental Notation Symbol-47', '/']);
UNICODE_MISTAKES.set('㇓', ['CJK Stroke Sp', '/']);
UNICODE_MISTAKES.set('〳', ['Vertical Kana Repeat Mark Upper Half', '/']);
UNICODE_MISTAKES.set('Ⳇ', ['Coptic Capital Letter Old Coptic Esh', '/']);
UNICODE_MISTAKES.set('ノ', ['Katakana Letter No', '/']);
UNICODE_MISTAKES.set('丿', ['CJK Unified Ideograph-4E3F', '/']);
UNICODE_MISTAKES.set('⼃', ['Kangxi Radical Slash', '/']);
UNICODE_MISTAKES.set('／', ['Fullwidth Solidus', '/']);

UNICODE_MISTAKES.set('＼', ['Fullwidth Reverse Solidus', '\\']);
UNICODE_MISTAKES.set('﹨', ['Small Reverse Solidus', '\\']);
UNICODE_MISTAKES.set('∖', ['Set Minus', '\\']);
UNICODE_MISTAKES.set('⟍', ['Mathematical Falling Diagonal', '\\']);
UNICODE_MISTAKES.set('⧵', ['Reverse Solidus Operator', '\\']);
UNICODE_MISTAKES.set('⧹', ['Big Reverse Solidus', '\\']);
UNICODE_MISTAKES.set('⧹', ['Greek Vocal Notation Symbol-16', '\\']);
UNICODE_MISTAKES.set('⧹', ['Greek Instrumental Symbol-48', '\\']);
UNICODE_MISTAKES.set('㇔', ['CJK Stroke D', '\\']);
UNICODE_MISTAKES.set('丶', ['CJK Unified Ideograph-4E36', '\\']);
UNICODE_MISTAKES.set('⼂', ['Kangxi Radical Dot', '\\']);
UNICODE_MISTAKES.set('、', ['Ideographic Comma', '\\']);
UNICODE_MISTAKES.set('ヽ', ['Katakana Iteration Mark', '\\']);

UNICODE_MISTAKES.set('ꝸ', ['Latin Small Letter Um', '&']);
UNICODE_MISTAKES.set('＆', ['Fullwidth Ampersand', '&']);

UNICODE_MISTAKES.set('᛭', ['Runic Cross Punctuation', '+']);
UNICODE_MISTAKES.set('➕', ['Heavy Plus Sign', '+']);
UNICODE_MISTAKES.set('𐊛', ['Lycian Letter H', '+']);
UNICODE_MISTAKES.set('﬩', ['Hebrew Letter Alternative Plus Sign', '+']);
UNICODE_MISTAKES.set('＋', ['Fullwidth Plus Sign', '+']);

UNICODE_MISTAKES.set('‹', ['Single Left-Pointing Angle Quotation Mark', '<']);
UNICODE_MISTAKES.set('❮', [
  'Heavy Left-Pointing Angle Quotation Mark Ornament',
  '<',
]);
UNICODE_MISTAKES.set('˂', ['Modifier Letter Left Arrowhead', '<']);
UNICODE_MISTAKES.set('𝈶', ['Greek Instrumental Symbol-40', '<']);
UNICODE_MISTAKES.set('ᐸ', ['Canadian Syllabics Pa', '<']);
UNICODE_MISTAKES.set('ᚲ', ['Runic Letter Kauna', '<']);
UNICODE_MISTAKES.set('❬', ['Medium Left-Pointing Angle Bracket Ornament', '<']);
UNICODE_MISTAKES.set('⟨', ['Mathematical Left Angle Bracket', '<']);
UNICODE_MISTAKES.set('〈', ['Left-Pointing Angle Bracket', '<']);
UNICODE_MISTAKES.set('〈', ['Left Angle Bracket', '<']);
UNICODE_MISTAKES.set('㇛', ['CJK Stroke Pd', '<']);
UNICODE_MISTAKES.set('く', ['Hiragana Letter Ku', '<']);
UNICODE_MISTAKES.set('𡿨', ['CJK Unified Ideograph-21FE8', '<']);
UNICODE_MISTAKES.set('《', ['Left Double Angle Bracket', '<']);
UNICODE_MISTAKES.set('＜', ['Fullwidth Less-Than Sign', '<']);

UNICODE_MISTAKES.set('᐀', ['Canadian Syllabics Hyphen', '=']);
UNICODE_MISTAKES.set('⹀', ['Double Hyphen', '=']);
UNICODE_MISTAKES.set('゠', ['Katakana-Hiragana Double Hyphen', '=']);
UNICODE_MISTAKES.set('꓿', ['Lisu Punctuation Full Stop', '=']);
UNICODE_MISTAKES.set('＝', ['Fullwidth Equals Sign', '=']);

UNICODE_MISTAKES.set('›', ['Single Right-Pointing Angle Quotation Mark', '>']);
UNICODE_MISTAKES.set('❯', [
  'Heavy Right-Pointing Angle Quotation Mark Ornament',
  '>',
]);
UNICODE_MISTAKES.set('˃', ['Modifier Letter Right Arrowhead', '>']);
UNICODE_MISTAKES.set('𝈷', ['Greek Instrumental Symbol-42', '>']);
UNICODE_MISTAKES.set('ᐳ', ['Canadian Syllabics Po', '>']);
UNICODE_MISTAKES.set('𖼿', ['Miao Letter Archaic Zza', '>']);
UNICODE_MISTAKES.set('❭', [
  'Medium Right-Pointing Angle Bracket Ornament',
  '>',
]);
UNICODE_MISTAKES.set('⟩', ['Mathematical Right Angle Bracket', '>']);
UNICODE_MISTAKES.set('〉', ['Right-Pointing Angle Bracket', '>']);
UNICODE_MISTAKES.set('〉', ['Right Angle Bracket', '>']);
UNICODE_MISTAKES.set('》', ['Right Double Angle Bracket', '>']);
UNICODE_MISTAKES.set('＞', ['Fullwidth Greater-Than Sign', '>']);

export const ASCII_NAMES: Map<string, string> = new Map();
ASCII_NAMES.set(' ', 'Space');
ASCII_NAMES.set('_', 'Underscore');
ASCII_NAMES.set('-', 'Minus/Hyphen');
ASCII_NAMES.set(',', 'Comma');
ASCII_NAMES.set(';', 'Semicolon');
ASCII_NAMES.set(':', 'Colon');
ASCII_NAMES.set('!', 'Exclamation Mark');
ASCII_NAMES.set('?', 'Question Mark');
ASCII_NAMES.set('.', 'Period');
ASCII_NAMES.set("'", 'Single Quote');
ASCII_NAMES.set('"', 'Quotation Mark');
ASCII_NAMES.set('(', 'Left Parenthesis');
ASCII_NAMES.set(')', 'Right Parenthesis');
ASCII_NAMES.set('[', 'Left Square Bracket');
ASCII_NAMES.set(']', 'Right Square Bracket');
ASCII_NAMES.set('{', 'Left Curly Brace');
ASCII_NAMES.set('}', 'Right Curly Brace');
ASCII_NAMES.set('*', 'Asterisk');
ASCII_NAMES.set('/', 'Slash');
ASCII_NAMES.set('\\', 'Backslash');
ASCII_NAMES.set('&', 'Ampersand');
ASCII_NAMES.set('+', 'Plus Sign');
ASCII_NAMES.set('<', 'Less-Than Sign');
ASCII_NAMES.set('=', 'Equals Sign');
ASCII_NAMES.set('>', 'Greater-Than Sign');
