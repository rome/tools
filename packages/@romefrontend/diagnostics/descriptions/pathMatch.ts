import {createDiagnosticsCategory} from "./index";

export const pathMatch = createDiagnosticsCategory({
	INVALID_PATTERN_SEGMENT_PART: "Invalid pattern segment part",
	INVALID_PATH_SEGMENT: "Invalid path segment",
});
