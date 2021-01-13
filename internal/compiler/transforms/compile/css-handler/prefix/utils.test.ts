import {test} from "rome";
import {nodeHasPrefix, nodeValueHasPrefix} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {cssBlock, cssDeclaration, cssString} from "@internal/ast";

test(
	"css prefix utils",
	(t) => {
		t.false(nodeHasPrefix(cssBlock.create({
			value: [
				cssDeclaration.create({
					important: false,
					name: "transition",
					value: []
				}),
				cssDeclaration.create({
					important: false,
					name: "-moz-transition",
					value: []
				})
			],
		}), "-webkit-"));

		t.true(nodeHasPrefix(cssBlock.create({
			value: [
				cssDeclaration.create({
					important: false,
					name: "transition",
					value: []
				}),
				cssDeclaration.create({
					important: false,
					name: "-webkit-transition",
					value: []
				})
			],
		}), "-webkit-"));


		t.false(nodeValueHasPrefix(cssBlock.create({
			value: [
				cssDeclaration.create({
					important: false,
					name: "display",
					value: [
						cssString.create({
							value: "flex"
						})
					]
				}),
				cssDeclaration.create({
					important: false,
					name: "display",
					value: [
						cssString.create({
							value: "-moz-flex"
						})
					]
				})
			],
		}), "display", "-webkit-"));


		t.true(nodeValueHasPrefix(cssBlock.create({
			value: [
				cssDeclaration.create({
					important: false,
					name: "display",
					value: [
						cssString.create({
							value: "flex"
						})
					]
				}),
				cssDeclaration.create({
					important: false,
					name: "display",
					value: [
						cssString.create({
							value: "-webkit-flex"
						})
					]
				})
			],
		}), "display", "-webkit-"));
	}
)
