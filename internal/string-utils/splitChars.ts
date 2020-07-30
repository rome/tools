export function splitChars(str: string): Array<string> {
	return str.split(/(?:){1}/u);
}
