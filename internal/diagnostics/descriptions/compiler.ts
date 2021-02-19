import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

// @internal/compiler
export const compiler = createDiagnosticsCategory({
	CLASSES_UNSUPPORTED: {
		category: DIAGNOSTIC_CATEGORIES["compile/classes"],
		message: markup`The classes transform doesn't know how to transform this`,
	},
	JSX_NOT_XML: {
		category: DIAGNOSTIC_CATEGORIES["compile/jsx"],
		message: markup`JSX is not XML`,
	},
	CONST_ENUMS_UNSUPPORTED: {
		category: DIAGNOSTIC_CATEGORIES["compile/const-enums"],
		message: markup`Const enums are not supported`,
	},
	ENUM_COMPUTED_VALUES_UNSUPPORTED: {
		category: DIAGNOSTIC_CATEGORIES["compile/nonnumeric-enum-values"],
		message: markup`Only numeric enums can have computed members`,
	},
});
