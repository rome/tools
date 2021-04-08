import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/codec-config
export const toml = createDiagnosticsCategory({
	UNCLOSED_STRING: {message: markup`Unclosed string`},
});
