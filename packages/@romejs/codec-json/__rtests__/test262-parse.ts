/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * This file contains test cases taken from the test262 project found at https://github.com/tc39/test262 and is licensed
 * as follows:
 *
 *   The << Software identified by reference to the Ecma Standard* ("Software)">>  is protected by copyright and is being
 *   made available under the  "BSD License", included below. This Software may be subject to third party rights (rights
 *   from 'parties other than Ecma International), including patent rights, and no licenses under such third party rights
 *   are granted under this license even if the third party concerned is a member of Ecma International.  SEE THE ECMA
 *   CODE OF CONDUCT IN PATENT MATTERS AVAILABLE AT http://www.ecma-international.org/memento/codeofconduct.htm FOR
 *   INFORMATION REGARDING THE LICENSING OF PATENT CLAIMS THAT ARE REQUIRED TO IMPLEMENT ECMA INTERNATIONAL STANDARDS*.
 *
 *   Copyright (C) 2012-2013 Ecma International
 *   All rights reserved.
 *
 *   Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
 *   following conditions are met:
 *   1.   Redistributions of source code must retain the above copyright notice, this list of conditions and the following
 *       disclaimer.
 *   2.   Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
 *       following disclaimer in the documentation and/or other materials provided with the distribution.
 *   3.   Neither the name of the authors nor Ecma International may be used to endorse or promote products derived from
 *       this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE ECMA INTERNATIONAL "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 *   SHALL ECMA INTERNATIONAL BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 *   (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 *   DAMAGE.
 *
 *   * Ecma International Standards hereafter means Ecma International Standards as well as Ecma Technical Reports
 */

import '@romejs/string-markup';
import {DiagnosticsError} from '@romejs/diagnostics';
import {parseJSON} from '@romejs/codec-json';
import test from '@romejs/test';

function parse(input: string) {
  return parseJSON({input});
}

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar treats whitespace as a token seperator', t => {
  t.throws(function() {
    parse('12\t\r\n 34'); // should produce a syntax error as whitespace results in two tokens
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('<VT> is not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\u000b1234'); // should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('<FF> is not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\u000c1234'); // should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('<NBSP> is not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\u00a01234'); // should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test(' <ZWSPP> is not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\u200b1234'); // should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('<BOM> is not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\ufeff1234'); // should produce a syntax error a
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('U+2028 and U+2029 are not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(function() {
    parse('\u2028\u20291234'); // should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('Whitespace characters can appear before/after any JSONtoken', () => {
  parse(
    `\t\r \n{\t\r \n"property"\t\r \n:\t\r \n{\t\r \n}\t\r \n,\t\r \n"prop2"\t\r \n:\t\r \n` +
      `[\t\r \ntrue\t\r \n,\t\r \nnull\t\r \n,123.456\t\r \n]\t\r \n}\t\r \n`,
  ); // should JOSN parse without error
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar treats <TAB> as a whitespace character', t => {
  t.is(parse('\t1234'), 1234, '<TAB> should be ignored');

  t.throws(
    function() {
      parse('12\t34');
    },
    DiagnosticsError,
    '<TAB> should produce a syntax error as whitespace results in two tokens',
  );
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar treats <CR> as a whitespace character', t => {
  t.is(parse('\r1234'), 1234, '<cr> should be ignored');

  t.throws(
    function() {
      parse('12\r34');
    },
    DiagnosticsError,
    '<CR> should produce a syntax error as whitespace results in two tokens',
  );
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar treats <LF> as a whitespace character', t => {
  t.is(parse('\n1234'), 1234, '<LF> should be ignored');

  t.throws(
    function() {
      parse('12\n34');
    },
    DiagnosticsError,
    '<LF> should produce a syntax error as whitespace results in two tokens',
  );
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar treats <SP> as a whitespace character', t => {
  t.is(parse(' 1234'), 1234, '<SP> should be ignored');
  t.throws(
    function() {
      parse('12 34');
    },
    DiagnosticsError,
    '<SP> should produce a syntax error as whitespace results in two tokens',
  );
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('JSONStrings can be written using double quotes', t => {
  t.is(parse('"abc"'), 'abc', 'parse(\'"abc"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('A JSONString may not be delimited by single quotes', t => {
  t.throws(function() {
    parse("'abc'");
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('A JSONString may not be delimited by Uncode escaped quotes', t => {
  t.throws(function() {
    parse('\\u0022abc\\u0022');
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('A JSONStrings can contain no JSONStringCharacters (Empty JSONStrings)', t => {
  t.is(parse('""'), '', 'parse(\'""\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar does not allow a JSONStringCharacter to be any of the Unicode characters U+0000 thru U+0007', t => {
  t.throws(function() {
    parse('"\u0000\u0001\u0002\u0003\u0004\u0005\u0006\u0007"'); // invalid string characters should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar does not allow a JSONStringCharacter to be any of the Unicode characters U+0008 thru U+000F', t => {
  t.throws(function() {
    parse('"\u0008\u0009\u000a\u000b\u000c\u000d\u000e\u000f"'); // invalid string characters should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar does not allow a JSONStringCharacter to be any of the Unicode characters U+0010 thru U+0017', t => {
  t.throws(function() {
    parse('"\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017"'); // invalid string characters should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar does not allow a JSONStringCharacter to be any of the Unicode characters U+0018 thru U+001F', t => {
  t.throws(function() {
    parse('"\u0018\u0019\u001a\u001b\u001c\u001d\u001e\u001f"'); // invalid string characters should produce a syntax error
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('The JSON lexical grammar allows Unicode escape sequences in a JSONString', t => {
  t.is(parse('"\\u0058"'), 'X', 'parse(\'"\\u0058"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('A JSONStringCharacter UnicodeEscape may not have fewer than 4 hex characters', t => {
  t.throws(function() {
    parse('"\\u005"');
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('A JSONStringCharacter UnicodeEscape may not include any non=hex characters', t => {
  t.throws(function() {
    parse('"\\u0X50"');
  });
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows '/' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\/"'), '/', 'parse(\'"\\/"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows '' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\\\"'), '\\', 'parse(\'"\\\\"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows 'b' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\b"'), '\b', 'parse(\'"\\b"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows 'f' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\f"'), '\f', 'parse(\'"\\f"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows 'n' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\n"'), '\n', 'parse(\'"\\n"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows 'r' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\r"'), '\r', 'parse(\'"\\r"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test("The JSON lexical grammer allows 't' as a JSONEscapeCharacter after '' in a JSONString", t => {
  t.is(parse('"\\t"'), '\t', 'parse(\'"\\t"\'})');
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property value middles with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';

  for (const char of nullChars) {
    t.throws(function() {
      parse(`{ "name" : Jo'${char}hn } `);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property name is a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';

  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ ${char} : "John" }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property name starts with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ ${char}name : "John" }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property name ends with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';

  for (let char of nullChars) {
    t.throws(function() {
      parse(`{name${char} : "John" }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property name starts and ends with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{${char}name${char} : "John" }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property name middles with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ na${char}me : "John" }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property value is a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ "name" : ${char} }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property value starts with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ "name" : ${char}John }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property value ends with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ "name" : John${char} }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('parse - parsing an object where property value starts and ends with a null character', t => {
  let nullChars: Array<string> = [];
  nullChars[0] = '"\u0000"';
  nullChars[1] = '"\u0001"';
  nullChars[2] = '"\u0002"';
  nullChars[3] = '"\u0003"';
  nullChars[4] = '"\u0004"';
  nullChars[5] = '"\u0005"';
  nullChars[6] = '"\u0006"';
  nullChars[7] = '"\u0007"';
  nullChars[8] = '"\u0008"';
  nullChars[9] = '"\u0009"';
  nullChars[10] = '"\u000A"';
  nullChars[11] = '"\u000B"';
  nullChars[12] = '"\u000C"';
  nullChars[13] = '"\u000D"';
  nullChars[14] = '"\u000E"';
  nullChars[15] = '"\u000F"';
  nullChars[16] = '"\u0010"';
  nullChars[17] = '"\u0011"';
  nullChars[18] = '"\u0012"';
  nullChars[19] = '"\u0013"';
  nullChars[20] = '"\u0014"';
  nullChars[21] = '"\u0015"';
  nullChars[22] = '"\u0016"';
  nullChars[23] = '"\u0017"';
  nullChars[24] = '"\u0018"';
  nullChars[25] = '"\u0019"';
  nullChars[26] = '"\u001A"';
  nullChars[27] = '"\u001B"';
  nullChars[28] = '"\u001C"';
  nullChars[29] = '"\u001D"';
  nullChars[30] = '"\u001E"';
  nullChars[31] = '"\u001F"';
  for (let char of nullChars) {
    t.throws(function() {
      parse(`{ "name" : ${char}John${char} }`);
    }, DiagnosticsError);
  }
});

// Copyright (c) 2012 Ecma International.  All rights reserved.
test('Other category z spaces are not valid JSON whitespace as specified by the production JSONWhitespace.', t => {
  t.throws(
    function() {
      parse('\u16801');
    },
    DiagnosticsError,
    '\\u1680',
  );

  t.throws(
    function() {
      parse('\u180e1');
    },
    DiagnosticsError,
    '\\u180e',
  );

  t.throws(
    function() {
      parse('\u20001');
    },
    DiagnosticsError,
    '\\u2000',
  );

  t.throws(
    function() {
      parse('\u20011');
    },
    DiagnosticsError,
    '\\u2001',
  );

  t.throws(
    function() {
      parse('\u20021');
    },
    DiagnosticsError,
    '\\u2002',
  );

  t.throws(
    function() {
      parse('\u20031');
    },
    DiagnosticsError,
    '\\u2003',
  );

  t.throws(
    function() {
      parse('\u20041');
    },
    DiagnosticsError,
    '\\u2004',
  );

  t.throws(
    function() {
      parse('\u20051');
    },
    DiagnosticsError,
    '\\u2005',
  );

  t.throws(
    function() {
      parse('\u20061');
    },
    DiagnosticsError,
    '\\u2006',
  );

  t.throws(
    function() {
      parse('\u20071');
    },
    DiagnosticsError,
    '\\u2007',
  );

  t.throws(
    function() {
      parse('\u20081');
    },
    DiagnosticsError,
    '\\u2008',
  );

  t.throws(
    function() {
      parse('\u20091');
    },
    DiagnosticsError,
    '\\u2009',
  );

  t.throws(
    function() {
      parse('\u200a1');
    },
    DiagnosticsError,
    '\\u200a',
  );

  t.throws(
    function() {
      parse('\u202f1');
    },
    DiagnosticsError,
    '\\u202f',
  );

  t.throws(
    function() {
      parse('\u205f1');
    },
    DiagnosticsError,
    '\\u205f',
  );

  t.throws(
    function() {
      parse('\u30001');
    },
    DiagnosticsError,
    '\\u3000',
  );
});

// Copyright 2011 the Sputnik authors.  All rights reserved.
test('Tests that parse treats "__proto__" as a regular property name', () => {
  let x = parse('{"__proto__":[]}');

  if (Object.getPrototypeOf(x) !== Object.prototype) {
    throw new Error('#1: parse confused by "__proto__"');
  }

  // @ts-ignore
  if (!Array.isArray(x.__proto__)) {
    throw new Error('#2: parse did not set "__proto__" as a regular property');
  }
});
