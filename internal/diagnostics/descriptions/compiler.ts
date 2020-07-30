import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/compiler
export const compiler = createDiagnosticsCategory({
	CLASSES_UNSUPPORTED: {
		category: "compile/classes",
		message: markup`The classes transform doesn't know how to transform this`,
	},
	JSX_NOT_XML: {
		category: "compile/jsx",
		message: markup`JSX is not XML`,
	},
	CONST_ENUMS_UNSUPPORTED: {
		category: "compile/const-enums",
		message: markup`Const enums are not supported`,
	},
	ENUM_COMPUTED_VALUES_UNSUPPORTED: {
		category: "compile/nonnumeric-enum-values",
		message: markup`Only numeric enums can have computed members`,
	},
});
