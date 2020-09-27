import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/markdown-parser
export const markdownParser = createDiagnosticsCategory({
	INVALID_SEQUENCE: {message: markup`Invalid sequence`},
	ONLY_TEXT_INSIDE_DEFINITIONS: {
		message: markup`Inside definition [] you are only allowed to use inline styles such as bold, emphasis and code spans.`,
	},
	TEXT_WITHOUT_TEXT: {message: markup`A token without text has been provided`},
});
