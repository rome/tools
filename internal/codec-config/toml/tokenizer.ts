import {createParser, isntLineBreak, isDigit, createReadCallback} from "@internal/parser-core";
import {TOMLParser, TOMLParserTypes} from "./types";
import {descriptions} from "@internal/diagnostics";
import {ZeroIndexed} from "@internal/numbers";
import {unescapeString} from "@internal/string-escape";

// Remove underscores from 'a string, this is used for numeric separators eg. 100_000
function removeUnderscores(
	parser: TOMLParser,
	index: ZeroIndexed,
	raw: string,
): string {
	let str = "";

	for (let i = 0; i < raw.length; i++) {
		const char = raw[i];

		if (char === "_") {
      // TODO require number on either side
			continue;
		} else {
			str += char;
		}
	}

	return str;
}

// Used for Number token validation, allow underscore as a separator
function isNumberChar(char: string): boolean {
	return isDigit(char) || char === "_";
}

const isSingleStringValueChar = createReadCallback("'");
const isDoubleStringValueChar = createReadCallback('"');
const isMultilineSingleStringValueChar = createReadCallback("'''");
const isMultilineDoubleStringValueChar = createReadCallback('"""');

export function isValidWordKey(char: string) {
	return char !== undefined && /^[A-Za-z0-9_\-]+$/.test(char);
}

export const tomlParser = createParser<TOMLParserTypes>({
  diagnosticLanguage: "toml",
  ignoreWhitespaceTokens: true,
	getInitialState: (parser) => ({
		target: parser.meta.root,
	}),
	tokenize(parser, tokenizer) {
		const char = tokenizer.get();

		switch (char) {
      // Skip comments completely from tokenization
      // TODO we will need to track these at some point for serialization
      case "#": {
        tokenizer.read(isntLineBreak);
				return parser.lookaheadToken(tokenizer.index);
      }

			case "'":
			case '"':
        return tokenizeString(parser, char, tokenizer);
    }

    if (tokenizer.consume("[")) {
      return tokenizer.finishToken("OpenSquareBracket");
    }

    if (tokenizer.consume("]")) {
      return tokenizer.finishToken("CloseSquareBracket");
    }

    if (tokenizer.consume(":")) {
      return tokenizer.finishToken("Colon");
    }

    if (tokenizer.consume("=")) {
      return tokenizer.finishToken("Equals");
    }

    if (tokenizer.consume(".")) {
      return tokenizer.finishToken("Dot");
    }

    if (tokenizer.consume("+")) {
      return tokenizer.finishToken("Plus");
    }

    if (tokenizer.consume("-")) {
      return tokenizer.finishToken("Minus");
    }

    if (tokenizer.consume("{")) {
      return tokenizer.finishToken("OpenCurlyBrace");
    }

    if (tokenizer.consume("}")) {
      return tokenizer.finishToken("CloseCurlyBrace");
    }

    if (tokenizer.consume(",")) {
      return tokenizer.finishToken("Comma");
    }

    if (isDigit(char)) {
      return tokenizeDigit(parser, tokenizer);
    }

		if (isValidWordKey(char)) {
			const value = tokenizer.read(isValidWordKey);
			return tokenizer.finishValueToken("Word", value);
		} else {
      tokenizer.take(1);

			// Invalid but we'll reverify it with allowedCharacterForKey later
			return tokenizer.finishValueToken("Word", char);
		}
	},
});


function tokenizeString(parser: TOMLParser, char: string, tokenizer: TOMLParser["tokenizer"]) {
  tokenizer.consume(char);

  let isMultiline = tokenizer.consume(char.repeat(2));

  let valueCallback = char === '"' ? isDoubleStringValueChar : isSingleStringValueChar;
  if (isMultiline) {
    valueCallback = char === '"' ? isMultilineDoubleStringValueChar : isMultilineSingleStringValueChar;
  }

  const valueStart = tokenizer.index;
  let value = tokenizer.read(valueCallback);

  if (tokenizer.isEOF()) {
    throw parser.unexpected({
      description: descriptions.TOML.UNCLOSED_STRING,
      start: tokenizer.getPosition(),
    });
  }

  value = unescapeString(
    value,
    {
      mode: isMultiline ? "toml-multiline" : "toml-singleline",
      unexpected(metadata, strIndex) {
        throw parser.unexpected({
          description: metadata,
          start: parser.getPositionFromIndex(valueStart.add(strIndex)),
        });
      }
    },
  );

  if (isMultiline) {
    tokenizer.assert(char.repeat(3));
  } else {
    tokenizer.assert(char);
  }

  return tokenizer.finishValueToken("String", value);
}

function tokenizeDate(parser: TOMLParser, tokenizer: TOMLParser["tokenizer"], year: number) {
  tokenizer.assert("-");

  // Get month
  const monthStart = tokenizer.index;
  const month = tokenizer.read(isDigit);
  if (month.length !== 2) {
    throw parser.unexpected({
      startIndex: monthStart,
      endIndex: tokenizer.index,
    });
  }
  tokenizer.assert("-");

  // Get day
  const dayStart = tokenizer.index;
  const day = tokenizer.read(isDigit);
  if (day.length !== 2) {
    throw parser.unexpected({
      startIndex: dayStart,
      endIndex: tokenizer.index,
    });
  }

  return tokenizer.finishComplexToken("Date", {
    year,
    month: Number(month),
    day: Number(day),
  });
}

function tokenizeTime(parser: TOMLParser, tokenizer: TOMLParser["tokenizer"], hours: number) {
  tokenizer.assert(":");

  // Get minutes
  const minutesStart = tokenizer.index;
  const minutes = tokenizer.read(isDigit);
  if (minutes.length !== 2) {
    throw parser.unexpected({
      startIndex: minutesStart,
      endIndex: tokenizer.index,
    });
  }
  tokenizer.assert(":");

  // Get seconds
  const secondsStart = tokenizer.index;
  let seconds = tokenizer.read(isDigit);
  if (seconds.length !== 2) {
    throw parser.unexpected({
      startIndex: secondsStart,
      endIndex: tokenizer.index,
    });
  }

  // Get fractional seconds
  if (tokenizer.consume(".")) {
    seconds += ".";

    let fractionalSeconds = tokenizer.read(isDigit);
    if (fractionalSeconds.length === 0) {
      throw parser.unexpected({
        index: tokenizer.index,
      });
    } else {
      seconds += fractionalSeconds;
    }
  }

  return tokenizer.finishComplexToken("Time", {
    hours,
    minutes: Number(minutes),
    seconds: Number(seconds),
  });
}

function tokenizeDigit(parser: TOMLParser, tokenizer: TOMLParser["tokenizer"]) {
  const start = tokenizer.index;
  let raw = tokenizer.read(isNumberChar);
  let num = removeUnderscores(parser, start, raw);
  let isFloat = false;

  if (raw.length === 4 && num.length === 4 && tokenizer.startsWith("-")) {
    return tokenizeDate(parser, tokenizer, Number(num));
  }

  if (raw.length === 2 && raw.length === 2 && tokenizer.startsWith(":")) {
    return tokenizeTime(parser, tokenizer, Number(num));
  }

  if (tokenizer.consume(".")) {
    isFloat = true;
    num += ".";

    const start = tokenizer.index;
    const raw = tokenizer.read(isNumberChar);
    if (raw.length === 0) {
      // TODO custom error
      throw parser.unexpected({
        index: tokenizer.index,
      });
    } else {
      num += removeUnderscores(parser, start, raw);
    }
  }

  if (tokenizer.consume("e") || tokenizer.consume("E")) {
    isFloat = true;
    num += "e";

    if (tokenizer.startsWith("+") || tokenizer.startsWith("-")) {
      num += tokenizer.take(1);
    }

    const start = tokenizer.index;
    const raw = tokenizer.read(isNumberChar);
    if (raw.length === 0) {
      // TODO specific error
      throw parser.unexpected({
        index: tokenizer.index,
      });
    } else {
      num += removeUnderscores(parser, start, raw);
    }
  }

  if (isFloat) {
    return tokenizer.finishValueToken("Float", num);
  } else {
    return tokenizer.finishValueToken("Int", num);
  }
}
