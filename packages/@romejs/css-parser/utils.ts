import {Number0, ob1Add, ob1Get, ob1Inc} from "@romejs/ob1";
import {isAlpha, isDigit, isHexDigit} from "@romejs/parser-core";

export const Symbols = {
	CarriageReturn: "\r",
	Control: 0x80,
	FormFeed: "\f",
	LineFeed: "\n",
	MaxValue: 0x10ffff,
	Replace: "\ufffd",
	Space: " ",
	SurrogateMax: 0xdfff,
	SurrogateMin: 0xd800,
	Tab: "\t",
};

export function consumeBadURL(index: Number0, input: string): [Number0] {
	while (ob1Get(index) < input.length) {
		if (getChar(index, input) === ")") {
			return [ob1Inc(index)];
		}

		if (isValidEscape(getChar(index, input), getChar(index, input, 1))) {
			[index] = consumeEscaped(index, input);
		} else {
			index = ob1Inc(index);
		}
	}
	return [index];
}

export function consumeChar(index: Number0, input: string): [Number0, string] {
	return [ob1Inc(index), getChar(index, input)];
}

export function consumeEscaped(index: Number0, input: string): [Number0, string] {
	let value = "";
	index = ob1Add(index, 2);
	const lastChar = getChar(index, input, -1);

	if (isHexDigit(lastChar)) {
		const maxOffset = Math.min(input.length, ob1Get(index) + 5);
		let hexString = lastChar;
		let utf8Value = "";

		while (ob1Get(index) < maxOffset) {
			if (!isHexDigit(getChar(index, input))) {
				break;
			}
			hexString += getChar(index, input);
			index = ob1Inc(index);
		}
		const hexNumber = parseInt(hexString, 16);
		if (
			hexNumber === 0 ||
			hexNumber > Symbols.MaxValue ||
			(hexNumber >= Symbols.SurrogateMin && hexNumber <= Symbols.SurrogateMax)
		) {
			utf8Value = Symbols.Replace;
		} else {
			utf8Value = hexToUtf8(hexString);
		}
		value += utf8Value;

		if (isWhitespace(input[ob1Get(index)])) {
			index = ob1Add(index, getNewlineLength(index, input));
		}
	}

	return [index, value];
}

export function consumeName(index: Number0, input: string): [Number0, string] {
	let value = "";

	while (ob1Get(index) < input.length) {
		const char1 = getChar(index, input);
		const char2 = getChar(index, input, 1);

		if (isName(char1)) {
			value += char1;
			index = ob1Inc(index);
			continue;
		}

		if (isValidEscape(char1, char2)) {
			const [newIndex, newValue] = consumeEscaped(index, input);
			value += newValue;
			index = newIndex;
			continue;
		}

		break;
	}

	return [index, value];
}

export function consumeNumber(
	index: Number0,
	input: string,
): [Number0, number, string] {
	const char = getChar(index, input);
	let value = "";
	let type = "integer";

	if (char === "+" || char === "-") {
		value += char;
		index = ob1Inc(index);
	}

	while (isDigit(getChar(index, input))) {
		value += getChar(index, input);
		index = ob1Inc(index);
	}

	if (getChar(index, input) === "." && isDigit(getChar(index, input, 1))) {
		value += getChar(index, input);
		index = ob1Inc(index);

		value += getChar(index, input);
		index = ob1Inc(index);

		type = "number";

		while (isDigit(getChar(index, input))) {
			value += getChar(index, input);
			index = ob1Inc(index);
		}
	}

	const char1 = getChar(index, input);
	const char2 = getChar(index, input, 1);
	const char3 = getChar(index, input, 2);

	if (char1 === "E" || char1 === "e") {
		if (isDigit(char2)) {
			value += getChar(index, input);
			index = ob1Inc(index);

			value += getChar(index, input);
			index = ob1Inc(index);
		} else if ((char2 === "-" || char2 === "+") && isDigit(char3)) {
			value += getChar(index, input);
			index = ob1Inc(index);

			value += getChar(index, input);
			index = ob1Inc(index);

			value += getChar(index, input);
			index = ob1Inc(index);

			while (isDigit(getChar(index, input))) {
				value += getChar(index, input);
				index = ob1Inc(index);
			}
		}
	}

	return [index, parseFloat(value), type];
}

export function getChar(index: Number0, input: string, offset = 0): string {
	const targetIndex = ob1Get(index) + offset;
	return input[targetIndex];
}

export function getCodePoint(char: string): number {
	if (!char) {
		return -1;
	}

	if (char.length === 1) {
		const point = char.codePointAt(0);
		if (point !== undefined) {
			return point;
		}
	}

	throw new Error("Input was not 1 character long");
}

export function getNewlineLength(index: Number0, input: string): number {
	if (
		getChar(index, input) === Symbols.CarriageReturn &&
		getChar(index, input, 1) === Symbols.LineFeed
	) {
		return 2;
	}

	return 1;
}

export function hexToUtf8(hex: string): string {
	const match = hex.match(/.{1,2}/g);
	return match ? decodeURIComponent(`%${match.join("%")}`) : Symbols.Replace;
}

export function isIdentifierStart(
	char1: string,
	char2: string,
	char3: string,
): boolean {
	if (char1 === "-") {
		return isNameStart(char2) || char2 === "-" || isValidEscape(char2, char3);
	}

	if (isNameStart(char1)) {
		return true;
	}

	if (char1 === "\\") {
		return isValidEscape(char1, char2);
	}

	return false;
}

export function isName(char: string): boolean {
	return isNameStart(char) || isDigit(char) || char === "-";
}

export function isNameStart(char: string): boolean {
	return isAlpha(char) || isNonAscii(char) || char === "_";
}

export function isNewline(char: string): boolean {
	return (
		char === Symbols.LineFeed ||
		char === Symbols.CarriageReturn ||
		char === Symbols.FormFeed
	);
}

export function isNonAscii(char: string): boolean {
	return getCodePoint(char) >= Symbols.Control;
}

export function isNumberStart(
	char1: string,
	char2: string,
	char3: string,
): boolean {
	if (char1 === "+" || char1 === "-") {
		if (isDigit(char2)) {
			return true;
		}
		return char2 === "." && isDigit(char3);
	}

	if (char1 === ".") {
		return isDigit(char2);
	}

	return isDigit(char1);
}

export function isValidEscape(char1: string, char2?: string): boolean {
	if (char1 !== "\\" || !char2 || isNewline(char2)) {
		return false;
	}

	return true;
}

export function isWhitespace(char: string): boolean {
	return isNewline(char) || char === Symbols.Space || char === Symbols.Tab;
}
