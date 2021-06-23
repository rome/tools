import {TOMLParser, TOMLValue, TOMLArray, TOMLObject, Tokens} from "./types";
import {descriptions} from "@internal/diagnostics";
import {isPlainObject} from "@internal/typescript-helpers";
import {isValidWordKey} from "./tokenizer";

function parseTableHeader(parser: TOMLParser) {
  parser.expectToken("OpenSquareBracket")

  const isArrayElement = parser.eatToken("OpenSquareBracket");
  const keys = parseKeys(parser);

  const [obj, key] = getPath(parser.meta.root, keys);
  const newTarget = {};

  if (isArrayElement) {
    // TODO subtables
    let arr: TOMLArray;

    let existing = obj[key];
    if (existing === undefined) {
      arr = [];
    } else if (Array.isArray(existing)) {
      arr = existing;
    } else {
      // TODO?
      throw parser.unexpected();
    }

    arr.push(newTarget);
    obj[key] = arr;
  } else {
    obj[key] = newTarget;
  }
  parser.setState({target: newTarget});

  if (isArrayElement) {
    parser.expectToken("CloseSquareBracket");
  }
  parser.expectToken("CloseSquareBracket");
}

export function parseRoot(parser: TOMLParser): TOMLObject {
	while (true) {
    while (parser.matchToken("OpenSquareBracket")) {
      parseTableHeader(parser);
    }

    if (parser.matchToken("EOF")) {
      break;
    }

    // TODO ensure newlines between key/values

    const keys = parseKeys(parser);

    if (!parser.eatToken("Equals")) {
      throw parser.unexpected({
        description: descriptions.TOML.NO_VALUE_FOR_KEY(keys.join(".")),
      });
    }

    const value = parseValue(parser);
		setObjectValue(parser.state.target, keys, value);
	}

	return parser.meta.root;
}

function getPath(obj: TOMLObject, keys: string[]): [TOMLObject, string] {
  let target = obj;

  // Make sure all keys except the last are objects
  for (let i = 0; i < keys.length - 2; i++) {
    const key = keys[i];
    const value = target[key];

    if (value === undefined) {
      const obj = {};
      target[key] = obj;
      target = obj;
    } else if (isPlainObject(value)) {
      target = value;
    } else {
      // TODO error
    }
  }

  const key = keys[keys.length - 1];
  return [target, key];
}

function setObjectValue(obj: TOMLObject, keys: string[], value: TOMLValue) {
  const [target, key] = getPath(obj, keys);

  if (key in target) {
    // TODO defined multiple times
  } else {
    target[key] = value;
  }
}

function parseInlineTable(parser: TOMLParser): TOMLObject {
  const obj: TOMLObject = {};

  let trailingComma: undefined | Tokens["Comma"];

	while (!parser.matchToken("EOF") && !parser.matchToken("CloseCurlyBrace")) {
    const keys = parseKeys(parser);

    parser.expectToken("Equals");

    const value = parseValue(parser);

    setObjectValue(obj, keys, value);

    trailingComma = parser.eatToken("Comma");
  }

  if (trailingComma !== undefined) {
    // TODO custom error
    throw parser.unexpected({
      token: trailingComma,
    });
  }

	parser.expectToken("CloseCurlyBrace");

	return obj;
}

function isArrayElementSeparator(parser: TOMLParser): boolean {
  if (parser.eatToken("Comma")) {
    return true;
  }

  if (parser.matchToken("CloseSquareBracket")) {
    return true;
  }

  return false;
}

function parseArray(parser: TOMLParser): TOMLArray {
	const arr: TOMLArray = [];

	while (!parser.matchToken("EOF") && !parser.matchToken("CloseSquareBracket")) {
		const value = parseValue(parser);
		arr.push(value);

		if (!isArrayElementSeparator(parser)) {
      // TODO custom message
      throw parser.unexpected();
    }
	}

	parser.expectToken("CloseSquareBracket");

	return arr;
}

function parseNumberWord(parser: TOMLParser, token: Tokens["Word"]): number {
  switch (token.value) {
    case "nan": {
      parser.nextToken();
      return NaN;
    }

    case "inf": {
      parser.nextToken();
      return Infinity;
    }

    default:
      // TODO custom error
      throw parser.unexpected();
  }
}

function parseWord(parser: TOMLParser, token: Tokens["Word"]): TOMLValue {
  switch (token.value) {
    case "true": {
      parser.nextToken();
      return true;
    }

    case "false": {
      parser.nextToken();
      return false;
    }

    default:
      return parseNumberWord(parser, token);
  }
}

function parseNumber(parser: TOMLParser): number {
  const token = parser.getToken();

  switch (token.type) {
    case "Float":
    case "Int": {
      parser.nextToken();
      return Number(token.value);
    }

    case "Plus":
      // Excessive plus, should have been removed
      throw parser.unexpected();

    case "Minus":
      // Excessive minus, should have been removed
      throw parser.unexpected();

    default:
      throw parser.unexpected();
  }
}

function parseValue(parser: TOMLParser): TOMLValue {
	const token = parser.getToken();

	switch (token.type) {
		case "String": {
			parser.nextToken();
			return token.value;
		}

		case "OpenSquareBracket": {
			parser.nextToken();
			return parseArray(parser);
    }

		case "OpenCurlyBrace": {
			parser.nextToken();
			return parseInlineTable(parser);
    }

    case "Word":
      return parseWord(parser, token);

    case "Minus": {
      parser.nextToken();
      return -parseNumber(parser);
    }

    case "Plus": {
      parser.nextToken();
      return +parseNumber(parser);
    }

    case "Int":
    case "Float":
      return parseNumber(parser);

    // TODO hex
    // 0xDEADBEEF
    // 0xdeadbeef
    // 0xdead_beef

    // TODO binary
    // 0b11010110

    // TODO octal
    // 0o01234567
    // 0o755

    // TODO offset datetime
    // 1979-05-27T07:32:00Z
    // 1979-05-27T00:32:00-07:00
    // 1979-05-27T00:32:00.999999-07:00

    // TODO local datetime
    // 1979-05-27T07:32:00
    // 1979-05-27T00:32:00.999999

    // TODO local date
    // 1979-05-27

    // TODO local time
    // 07:32:00
    // 00:32:00.999999

		default: {
			throw parser.unexpected();
		}
	}
}

function parseKey(parser: TOMLParser): string {
  if (parser.matchToken("String")) {
    return parser.expectToken("String").value;
  }

  const parts: string[] = [];

  while (true) {
    const token = parser.getToken();

    switch (token.type) {
      case "String":
        // TODO custom message
        throw parser.unexpected();

      case "Int": {
        parts.push(token.value);
        parser.nextToken();
        break;
      }

      case "Word": {
        const key = token.value;

        if (isValidWordKey(key)) {
          parser.nextToken();
          parts.push(key);
        } else {
          throw parser.unexpected({
            token,
            description: descriptions.TOML.INVALID_KEY_CHAR(key),
          });
        }
        break;
      }

      default:
        throw parser.unexpected();
    }

    if (!isAtValidKeyPart(parser)) {
      break;
    }
  }

  return parts.join("");
}

function isAtValidKeyPart(parser: TOMLParser): boolean {
  return parser.matchToken("Word") || parser.matchToken("String") || parser.matchToken("Int");
}

function parseKeys(parser: TOMLParser): string[] {
  let keys: string[] = [];

  while (true) {
    keys.push(parseKey(parser));

    const trailingDot = parser.eatToken("Dot");

    if (!isAtValidKeyPart(parser)) {
      if (trailingDot !== undefined) {
        // TODO custom message
        throw parser.unexpected({
          token: trailingDot,
        });
      }
      break;
    }
  }

  return keys;
}
