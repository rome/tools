import {
	FormatterSettings,
	Printed,
	QuoteStyle,
	Workspace,
	WorkspaceSettings,
} from "./workspace";
import { FormatError } from "./error";

const TEMPORARY_CONTENT = "temporary_content";

function createWorkspaceSettingsFromFormatOptions({
	quoteStyle,
	indentStyle,
	indentWidth,
	lineWidth,
}: Format): WorkspaceSettings {
	const settings: WorkspaceSettings = {
		formatter: {
			indentStyle,
			indentWidth,
			lineWidth,
		},
		linter: undefined,
		javascript: undefined,
	};

	if (quoteStyle) {
		settings.javascript = {
			formatter: {
				quoteStyle: quoteStyle,
			},
		};
	}

	return settings;
}

/**
 * Options passed to format JavaScript files
 */
interface Format extends FormatterSettings {
	quoteStyle: QuoteStyle | undefined;
}

/**
 * The main entry point of the API. Create an instance of this class, and then
 * you can use the exported APIs.
 */
class Rome {
	private workspace: Workspace;

	constructor() {
		this.workspace = new Workspace();
	}

	/**
	 *
	 * @param {FormatterSettings} params
	 */
	format_options(params: Format) {
		const settings = createWorkspaceSettingsFromFormatOptions(params);
		this.workspace.update_settings({
			settings,
		});
	}

	format(content: String, options: Format | undefined): Printed | undefined {
		if (options) {
			this.format_options(options);
		}

		this.workspace.open_file({
			path: TEMPORARY_CONTENT,
			content,
		});
		try {
			return this.workspace.format_file({
				path: TEMPORARY_CONTENT,
			});
		} catch (err) {
			if (err instanceof FormatError) {
				console.error(err.missing_content());
			} else {
				console.log("Unknown error");
				console.log(err);
			}
			return undefined;
		}
	}
}

/**
 * Formats some content given the input options. The options are optionals.
 * @param {String} content
 * @param {Format=?} options
 * @returns {Printed | undefined}
 */
function format(
	content: String,
	options: Format | undefined,
): Printed | undefined {
	const workspace = new Workspace();
	if (options) {
		const settings = createWorkspaceSettingsFromFormatOptions(options);
		workspace.update_settings({ settings });
	}

	workspace.open_file({
		path: TEMPORARY_CONTENT,
		content,
	});
	try {
		return workspace.format_file({
			path: TEMPORARY_CONTENT,
		});
	} catch (err) {
		if (err instanceof FormatError) {
			console.error(err.missing_content());
		} else {
			console.log("Unknown error");
			console.log(err);
		}
		return undefined;
	}
}

export type { Format, Printed };
export { Rome, format };
