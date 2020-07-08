import {createDiagnosticsCategory} from "./index";

export const v8 = createDiagnosticsCategory({
	SYNTAX_ERROR: (message: string) => ({message, category: "v8/syntaxError"}),
});
