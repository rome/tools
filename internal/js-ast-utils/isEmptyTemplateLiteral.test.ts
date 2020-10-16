import {test} from "rome";
import {jsTemplateElement, jsTemplateLiteral} from "@internal/ast";
import {isEmptyTemplateLiteral} from "@internal/js-ast-utils/isEmptyTemplateLiteral";

test(
	"returns whether the template literal is empty",
	async (t) => {
		t.false(
			isEmptyTemplateLiteral(
				jsTemplateLiteral.create({
					expressions: [],
					quasis: [
						jsTemplateElement.create({
							cooked: "foo",
							raw: "bar",
						}),
					],
				}),
			),
		);

		t.false(
			isEmptyTemplateLiteral(
				jsTemplateLiteral.create({
					expressions: [],
					quasis: [
						jsTemplateElement.create({
							cooked: "",
							raw: "",
						}),
						jsTemplateElement.create({
							cooked: "",
							raw: "",
						}),
					],
				}),
			),
		);

		t.true(
			isEmptyTemplateLiteral(
				jsTemplateLiteral.create({
					expressions: [],
					quasis: [
						jsTemplateElement.create({
							cooked: "",
							raw: "test",
						}),
					],
				}),
			),
		);
	},
);
