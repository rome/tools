import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

export const userConfig = createDiagnosticsCategory({
	VSCODE_THEME_NOT_FOUND: {message: markup`VSCode theme not found`},
});
