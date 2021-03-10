import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/flexbox.json
export default [
	createPrefixVisitor({
		name: "display/flex",
		enter(path, targets) {
			return prefixCSSValue(
				path,
				"display",
				"flex",
				"flexbox",
				targets,
				(propertyName) =>
					propertyName === "-moz-flex" ? "-moz-box" : propertyName
				,
			);
		},
	}),
	createPrefixVisitor({
		name: "display/inline-flex",
		enter(path, targets) {
			return prefixCSSValue(
				path,
				"display",
				"inline-flex",
				"flexbox",
				targets,
				(value) => value === "-moz-inline-flex" ? "-moz-inline-box" : value,
			);
		},
	}),
];
