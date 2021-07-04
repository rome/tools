// Split by "." but ignore "\."
export function splitEscapedObjectPath(path: string): string[] {
	const result: string[] = [];
	let lastIndex = 0;
	for (let i = 0; i < path.length; i++) {
		if (path[i] === "." && i === 0) {
			lastIndex = 1;
		} else if (path[i] === "." && i === path.length - 1 && path[i - 1] !== "\\") {
			result.push(path.slice(lastIndex, i));
			lastIndex = i + 1;
		} else if (path[i] === "." && path[i - 1] !== "\\") {
			// ignore repeated dots
			if (path[i - 1] === ".") {
				lastIndex = i + 1;
			} else {
				result.push(path.slice(lastIndex, i));
				lastIndex = i + 1;
			}
		}
	}
	if (lastIndex < path.length) {
		result.push(path.slice(lastIndex));
	}

	return result;
}

// Replace "." with "\."
export function escapePath(path: string) {
	return path.replace(/\./g, "\\.");
}

// Replace "\." with "."
export function unescapePath(path: string) {
	return path.replace(/\\\./g, ".");
}
