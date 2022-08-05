import { FormatError } from "./error";

export enum FeatureName {
	Format,
	Lint,
}

export enum IndentStyle {
	Tab = "tab",
	Space = "space",
}

export enum QuoteStyle {
	Double,
	Single,
}

/**
 * Settings applied to the formatter as a whole
 */
export interface FormatterSettings {
	lineWidth: number | undefined;
	indentStyle: IndentStyle | undefined;
	indentWidth: number | undefined;
}

export interface JavaScriptFormatterSettings {
	quoteStyle: QuoteStyle | undefined;
}

export interface JavaScriptSettings {
	formatter: JavaScriptFormatterSettings | undefined;
}

/**
 * The new range, in case a range was formatted
 */
export type Range = [number, number];

export interface Printed {
	code: String;
	range: Range | undefined;
}

export interface LinterSettings {}

export interface WorkspaceSettings {
	formatter: FormatterSettings | undefined;
	linter: LinterSettings | undefined;
	javascript: JavaScriptSettings | undefined;
}

interface SupportsFeatureParams {
	path: String;
	feature: FeatureName;
}

interface UpdateSettingsParams {
	settings: WorkspaceSettings;
}

interface OpenFileParams {
	path: String;
	content: String;
}

interface GetSyntaxTreeParams {}

interface ChangeFileParams {}

interface CloseFileParams {}

interface PullDiagnosticsParams {}

interface PullActionsParams {}

interface FormatFileParams {
	path: String;
}

interface FormatRangeParams {
	path: string;
	range: Range;
}

interface FormatContentParams {}

interface FixFileParams {}

interface RenameParams {}

// rome-ignore lint(js/noUnusedVariables): currently bugged
export class Workspace {
	private settings: WorkspaceSettings;
	private files: Map<String, String>;

	constructor() {
		// Rome will handle the defaults.
		this.settings = {
			formatter: undefined,
			linter: undefined,
			javascript: undefined,
		};
		this.files = new Map();
	}

	// rome-ignore lint(js/noUnusedVariables): future implementation
	supports_feature(params: SupportsFeatureParams): boolean {
		return false;
	}

	/**
	 * Update the global settings for this workspace
	 *
	 * @param {UpdateSettingsParams} params
	 */
	update_settings(params: UpdateSettingsParams) {
		this.settings = {
			formatter: {
				lineWidth: params.settings.formatter?.lineWidth,
				indentWidth: params.settings.formatter?.indentWidth,
				indentStyle: params.settings.formatter?.indentStyle,
			},
			linter: {
				...params.settings.linter,
			},
			javascript: {
				formatter: {
					quoteStyle: params.settings.javascript?.formatter?.quoteStyle,
				},
			},
		};
	}

	// Add a new file to the workspace
	open_file({ path, content }: OpenFileParams) {
		this.files.set(path, content);
	}

	// Return a textual, debug representation of the syntax tree for a given document
	// rome-ignore lint(js/noUnusedVariables): future implementation
	get_syntax_tree(params: GetSyntaxTreeParams) {}

	// Change the content of an open file
	// rome-ignore lint(js/noUnusedVariables): future implementation
	change_file(params: ChangeFileParams) {}

	// Remove a file from the workspace
	// rome-ignore lint(js/noUnusedVariables): future implementation
	close_file(params: CloseFileParams) {}

	// Retrieves the list of diagnostics associated to a file
	// rome-ignore lint(js/noUnusedVariables): future implementation
	pull_diagnostics(params: PullDiagnosticsParams) {}

	/**
	 * Retrieves the list of code actions available for a given cursor
	 * position within a file
	 *
	 * @param {PullActionsParams} params
	 */
	// rome-ignore lint(js/noUnusedVariables): future implementation
	pull_actions(params: PullActionsParams) {}

	/**
	 * Runs the given file through the formatter using the provided options
	 * and returns the resulting source code
	 *
	 * @param {FormatFileParams} path
	 */
	format_file({ path }: FormatFileParams): Printed {
		let content = this.files.get(path);
		if (content === undefined) {
			throw new FormatError();
		}
		// TODO: here we should call the backend to format content
		return {
			code: "", // content returned by the backend call
			range: undefined,
		};
	}

	// Runs a range of an open document through the formatter
	// rome-ignore lint(js/noUnusedVariables): future implementation
	format_range({ path, range }: FormatRangeParams): Printed {
		let content = this.files.get(path);
		if (content === undefined) {
			throw new FormatError();
		}

		// TODO: here we should call the backend to format the range of a content
		return {
			code: "", // content returned by the backend call
			range: [0, 0],
		};
	}

	// Return the content of the file with all safe code actions applied
	// rome-ignore lint(js/noUnusedVariables): future implementation
	fix_file(params: FixFileParams) {}

	// Return the content of the file after renaming a symbol
	// rome-ignore lint(js/noUnusedVariables): future implementation
	rename(params: RenameParams) {}
}
