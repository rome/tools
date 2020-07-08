const selfClosingTagNames: Set<string> = new Set([
	"area",
	"base",
	"br",
	"embed",
	"hr",
	"img",
	"input",
	"link",
	"meta",
	"param",
	"source",
	"track",
	"wbr",
	"command",
	"keygen",
	"menuitem",
]);

export function isSelfClosingTagName(tagName: string): boolean {
	return selfClosingTagNames.has(tagName);
}
