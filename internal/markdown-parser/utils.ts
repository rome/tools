const THEMATIC_BREAKS = new Set(["***", "---", "___"]);

export function hasThematicBreak(input: string): boolean {
	return THEMATIC_BREAKS.has(input);
}
