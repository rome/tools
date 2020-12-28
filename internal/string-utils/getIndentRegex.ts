export function getIndentRegex(str: string): RegExp {
	const match = str.match(/^[ \t]*(?=\S)/gm);
	let count = 0;

	if (match !== null) {
		count = Math.min(...match.map((x) => x.length));
	}

	return new RegExp(`^[ \\t]{${count}}`, "gm");
}
