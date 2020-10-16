const NEWLINE = /\r\n|[\n\r\u2028\u2029]/;

export function splitLines(src: string): string[] {
	return src.split(NEWLINE);
}
