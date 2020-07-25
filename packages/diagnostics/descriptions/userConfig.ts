import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/cli-layout";

export const userConfig = createDiagnosticsCategory({
	VSCODE_THEME_NOT_FOUND: {message: markup`VSCode theme not found`},
});
