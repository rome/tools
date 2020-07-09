const THEMATIC_BREAKS = new Set([
    "***",
    "---",
    "___"
]);

export const hasThematicBreak = (input: string): boolean => {
    return THEMATIC_BREAKS.has(input);
}
