export function allowedCharacterForKey(char: string) {
	return char !== undefined && /[A-Za-z]|[0-9]|_|-/.test(char);
}
