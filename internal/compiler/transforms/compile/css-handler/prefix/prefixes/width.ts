import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/intrinsic-width.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L535

const properties = [
	"width",
	"min-width",
	"max-width",
	"height",
	"min-height",
	"max-height",
	"inline-size",
	"min-inline-size",
	"max-inline-size",
	"block-size",
	"min-block-size",
	"max-block-size",
	// TODO: implement prefixes for grid related props too
	//"grid",
	//"grid-template",
	//"grid-template-rows",
	//"grid-template-columns",
	//"grid-auto-columns",
	//"grid-auto-rows",
];

const values = [
	"min-content",
	"max-content",
	"fill",
	//"fill-available",
	//"stretch",
	"fit-content",
];

function fillRenamer(value: string) {
	switch (value) {
		case "-moz-fill":
			return "-moz-available";
		case "-webkit-fill":
			return "-webkit-fill-available";
	}

	return value;
}

export default properties.flatMap((propertyName) =>
	values.map((value) =>
		createPrefixVisitor({
			name: `${propertyName}/${value}`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value,
					browserFeaturesKey: "intrinsic-width",
					rename: fillRenamer,
				});
			},
		})
	)
);
