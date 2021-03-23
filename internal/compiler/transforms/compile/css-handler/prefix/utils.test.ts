import {test} from "rome";
import {
	findPropertyIndex,
	findPropertyValueIndex,
	nodeHasPrefixedProperty,
	nodeHasPrefixedPropertyValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {cssBlock, cssDeclaration, cssIdentifier} from "@internal/ast";

test(
	"css prefix utils",
	(t) => {
		t.false(
			nodeHasPrefixedProperty(
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
				"transition",
				"webkit",
			),
		);

		t.true(
			nodeHasPrefixedProperty(
				cssBlock.create({
					value: [
						cssDeclaration.create({
							important: false,
							name: "transition",
							value: [],
						}),
						cssDeclaration.create({
							important: false,
							name: "-webkit-transition",
							value: [],
						}),
					],
				}),
				"transition",
				"webkit",
			),
		);

		t.is(
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
			2,
		);

		t.false(
			nodeHasPrefixedPropertyValue(
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
				"flex",
				"webkit",
			),
		);

		t.true(
			nodeHasPrefixedPropertyValue(
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
									value: "-webkit-flex",
								}),
							],
						}),
					],
				}),
				"display",
				"flex",
				"webkit",
			),
		);

		t.is(
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
			2,
		);
	},
);
