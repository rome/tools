const regexCache: Map<number, RegExp> = new Map();

export function getIndentRegex(str: string): RegExp {
	const match = str.match(/^[ \t]*(?=\S)/gm);
	let count = 0;

	if (match !== null) {
		for (const str of match) {
			if (count === 0 || str.length < count) {
				count = str.length;
			}
		}
	}

	const cached = regexCache.get(count);
	if (cached !== undefined) {
		return cached;
	}

	const regex = new RegExp(`^[ \\t]{${count}}`, "gm");
	if (count < 15) {
		regexCache.set(count, regex);
	}
	return regex;
}
