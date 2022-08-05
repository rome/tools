// rome-ignore lint(js/noUnusedVariables): currently bugged
export class FormatError extends Error {
	constructor() {
		// Needs to pass both `message` and `options` to install the "cause" property.
		super("FormatError");
	}

	missing_content() {
		return `${this.message}: there's no content`;
	}
}
