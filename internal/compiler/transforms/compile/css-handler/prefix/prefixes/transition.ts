import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-transitions.json
export default createPrefixVisitor({
	name: "transition",
	enter(path, targets) {
		return prefixCSSProperty(path, "transition", "css-transitions", targets);
	},
});
