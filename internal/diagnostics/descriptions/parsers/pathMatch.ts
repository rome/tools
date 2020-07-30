import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

export const pathMatch = createDiagnosticsCategory({
	INVALID_PATTERN_SEGMENT_PART: {message: markup`Invalid pattern segment part`},
	INVALID_PATH_SEGMENT: {message: markup`Invalid path segment`},
});
