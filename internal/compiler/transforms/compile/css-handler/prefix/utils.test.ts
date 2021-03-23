import {test} from "rome";
import {
	findPropertyIndex,
	findPropertyValueIndex,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {cssBlock, cssDeclaration, cssIdentifier} from "@internal/ast";

test(
	"css prefix utils",
	(t) => {
		t.inlineSnapshot(
			findPropertyIndex(
				cssBlock.create({
					value: [
						cssDeclaration.create({
							important: false,
							name: "transition",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "-webkit-transform",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "-moz-transition",
							value: [],
						}),
					],
				}),
				"-webkit-transition",
			),
			"[-1, undefined]",
		);

		t.inlineSnapshot(
			findPropertyIndex(
				cssBlock.create({
					value: [
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "color",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "transition",
							value: [],
						}),
					],
				}),
				"transition",
			),
			'[2, CSSDeclaration {name: "transition", value: [], important: false}]',
		);

		t.inlineSnapshot(
			findPropertyValueIndex(
				cssBlock.create({
					value: [
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [
								cssIdentifier.create({
									value: "flex",
								}),
							],
						}),
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [
								cssIdentifier.create({
									value: "-moz-flex",
								}),
							],
						}),
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [
								cssIdentifier.create({
									value: "-webkit-block",
								}),
							],
						}),
					],
				}),
				"display",
				"-webkit-flex",
			),
			"[-1, undefined]",
		);

		t.inlineSnapshot(
			findPropertyValueIndex(
				cssBlock.create({
					value: [
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [
								cssIdentifier.create({
									value: "-webkit-flex",
								}),
							],
						}),
						cssDeclaration.create({
							important: false,
							name: "display",
							value: [
								cssIdentifier.create({
									value: "flex",
								}),
							],
						}),
					],
				}),
				"display",
				"flex",
			),
			'[2, CSSDeclaration {name: "display", value: [CSSIdentifier {value: "flex"}], important: false}]',
		);
	},
);
