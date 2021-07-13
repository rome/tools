const HAS_BACKSLASH = /\\/;

// Used as a fast check to skip expensive string unescape logic
export function hasEscapes(str: string): boolean {
	return HAS_BACKSLASH.test(str);
}
