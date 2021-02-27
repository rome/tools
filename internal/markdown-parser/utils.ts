import {ZeroIndexed} from "@internal/numbers";
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

const INLINE_HOT_CHARACTERS = new Set(["*", "_", "`", "[", "]", "(", ")", "\n"]);

export function hasThematicBreak(input: string): boolean {
	return THEMATIC_BREAKS.has(input);
}

interface CalculateFlanking {
	startIndex: ZeroIndexed;
	input: string;
	endIndex: ZeroIndexed;
}

export function canBeLeftFlankingDelimiter(
	{input, endIndex, startIndex}: CalculateFlanking,
): boolean {
	const nextCharIsNotWhiteSpace = input[endIndex.valueOf() + 1] !== " ";
	const previousCharIsWhiteSpace = input[startIndex.valueOf() - 1] === " ";

	const nextCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[endIndex.valueOf() + 1],
	);
	const previousCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[startIndex.valueOf() - 1],
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
	const nextCharIsWhiteSpace = input[endIndex.valueOf() + 1] === " ";
	const previousCharIsNotWhiteSpace = input[startIndex.valueOf() - 1] !== " ";

	const nextCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[endIndex.valueOf() + 1],
	);
	const previousCharIsPunctuation = PUNCTUATION_CHARACTERS.has(
		input[startIndex.valueOf() - 1],
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
	index: ZeroIndexed,
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

export function isListItem(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	const nextChar = input[index.valueOf() + 1];
	const nextNextChar = input[index.valueOf() + 2];
	const previousChar = input[index.valueOf() - 1];
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
	index: ZeroIndexed,
	input: string,
	charToCheck: string,
): boolean {
	const nextChar = input[index.valueOf() + 1];
	const nextNextChar = input[index.valueOf() + 2];
	return (
		char === charToCheck &&
		nextChar === charToCheck &&
		nextNextChar === charToCheck
	);
}

export function readUntilEndOfParagraph(char: string): boolean {
	return char !== "\n";
}
