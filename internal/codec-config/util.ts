// Words can't start with a digit
import {isAlpha, isDigit} from "@internal/parser-core";

export function isWordStartChar(char: string): boolean {
	return isAlpha(char) || char === "_" || char === "$";
}

// But a digit can appear inside of a word
export function isWordChar(char: string): boolean {
	return isWordStartChar(char) || isDigit(char);
}

// Check if an input string is a valid word, this is used by the stringifier to
// determine if a property key should be quoted
export function isValidWord(word: string): boolean {
	if (word.length === 0 || !isWordStartChar(word[0])) {
		return false;
	}

	for (const char of word) {
		if (!isWordChar(char)) {
			return false;
		}
	}

	return true;
}
