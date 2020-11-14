export function splitChars(str: string): string[] {
	return str.split(/(?:){1}/u);
}
