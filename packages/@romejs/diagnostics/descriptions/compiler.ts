import {createDiagnosticsCategory} from "./index";

// @romejs/compiler
export const compiler = createDiagnosticsCategory({
	CLASSES_UNSUPPORTED: {
		category: "compile/classes",
		message: "The classes transform doesn't know how to transform this",
	},
	JSX_NOT_XML: {
		category: "compile/jsx",
		message: "JSX is not XML",
	},
	CONST_ENUMS_UNSUPPORTED: {
		category: "compile/const-enums",
		message: "Const enums are not supported",
	},
});
