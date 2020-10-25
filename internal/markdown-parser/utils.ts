import {Number0, ob1Get0} from "@internal/ob1";
import {MarkdownParser} from "@internal/markdown-parser/index";
import {isDigit} from "@internal/parser-core";

const THEMATIC_BREAKS = new Set(["***", "---", "___"]);

const PUNCTUATION_CHARACTERS = new Set([
	"!",
	'"',
	"#",
	"$",
	"%",
	"&",
	"'",
	"(",
	")",
	"*",
	"+",
	",",
	"-",
	".",
	"/",
	"!",
	":",
	";",
	"<",
	"=",
	">",
	"?",
	"@",
	"[",
	"\\",
	"]",
	"^",
	"_",
	"`",
	"{",
	"|",
	"}",
	"~",
]);

const INLINE_HOT_CHARACTERS = new Set([
	"*",
	"_",
	"`",
	"[",
	"]",
	"(",
	")",
	"\n",
	"|",
]);

export function hasThematicBreak(input: string): boolean {
	return THEMATIC_BREAKS.has(input);
}

interface CalculateFlanking {
	startIndex: Number0;
	input: string;
	endIndex: Number0;
}

export function canBeLeftFlankingDelimiter(
	{input, endIndex, startIndex}: CalculateFlanking,
): boolean {
	const nextCharIsNotWhiteSpace = input[ob1Get0(endIndex) + 1] !== " ";
	const previousCharIsWhiteSpace = input[ob1Get0(startIndex) - 1] === " ";

	const nextCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[ob1Get0(endIndex) + 1],
	);
	const previousCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[ob1Get0(startIndex) - 1],
	);

	// https://github.github.com/gfm/#left-flanking-delimiter-run
	return (
		// 1. not followed by whitespace char
		nextCharIsNotWhiteSpace &&
		// 2a. not followed by a punctuation char
		(!nextCharIsPunctuation ||
		// 2b. followed by a punctuation char and  preceded by white space or punctuation char
		(nextCharIsPunctuation &&
		(previousCharIsWhiteSpace || previousCharIsPunctuation)))
	);
}

export function canBeRightFlankingDelimiter(
	{input, endIndex, startIndex}: CalculateFlanking,
): boolean {
	const nextCharIsWhiteSpace = input[ob1Get0(endIndex) + 1] === " ";
	const previousCharIsNotWhiteSpace = input[ob1Get0(startIndex) - 1] !== " ";

	const nextCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[ob1Get0(endIndex) + 1],
	);
	const previousCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[ob1Get0(startIndex) - 1],
	);

	// https://github.github.com/gfm/#right-flanking-delimiter-run
	return (
		// 1.  not preceded by whitespace
		previousCharIsNotWhiteSpace &&
		// 2a. not preceded by a punctuation char
		(!previousCharIsPunctuation ||
		// 2b. preceded by punctuation char and followed by white space or punctuation
		(previousCharIsPunctuation &&
		(nextCharIsWhiteSpace || nextCharIsPunctuation)))
	);
}

export function isntInlineCharacter(char: string) {
	return !INLINE_HOT_CHARACTERS.has(char);
}

export function isBlockToken(parser: MarkdownParser) {
	return (
		parser.matchToken("NewLine") ||
		parser.matchToken("Code") ||
		parser.matchToken("Break") ||
		parser.matchToken("HeadingLevel") ||
		parser.matchToken("ListItem")
	);
}

export function hasBlockTokens(
	char: string,
	index: Number0,
	input: string,
): boolean {
	return (
		isListItem(char, index, input) ||
		isBlock(char, index, input, "-") ||
		isBlock(char, index, input, "_") ||
		isBlock(char, index, input, "*") ||
		isBlock(char, index, input, "`")
	);
}

export function isListItem(char: string, index: Number0, input: string): boolean {
	const nextChar = input[ob1Get0(index) + 1];
	const nextNextChar = input[ob1Get0(index) + 2];
	const previousChar = input[ob1Get0(index) - 1];
	if (isDigit(char)) {
		return nextChar === "." && nextNextChar === " ";
	}
	if (char === "*" || char === "-") {
		return nextChar === " " && previousChar === "\n";
	}
	return false;
}

export function isBlock(
	char: string,
	index: Number0,
	input: string,
	charToCheck: string,
): boolean {
	const nextChar = input[ob1Get0(index) + 1];
	const nextNextChar = input[ob1Get0(index) + 2];
	return (
		char === charToCheck &&
		nextChar === charToCheck &&
		nextNextChar === charToCheck
	);
}

export function readUntilEndOfParagraph(char: string): boolean {
	return char !== "\n";
}
