import {test} from "rome";
import {tryStaticEvaluation} from "./tryStaticEvaluation";
import {template} from "./template";
import {Scope} from "@internal/compiler";
import {
	jsReferenceIdentifier,
	jsTemplateElement,
	jsTemplateLiteral,
} from "@internal/ast";

const TEST_SCOPE = new Scope({
	kind: "program",
	node: undefined,
	parentScope: undefined,
	rootScope: undefined,
});

test(
	"successfully evaluates unary expressions that aren't boolean negation",
	(t) => {
		t.false(tryStaticEvaluation(template.expression`-5`, TEST_SCOPE).bailed);
		t.is(tryStaticEvaluation(template.expression`-5`, TEST_SCOPE).value, -5);

		t.false(tryStaticEvaluation(template.expression`+5`, TEST_SCOPE).bailed);
		t.is(tryStaticEvaluation(template.expression`+5`, TEST_SCOPE).value, 5);

		t.false(tryStaticEvaluation(template.expression`~10`, TEST_SCOPE).bailed);
		t.is(tryStaticEvaluation(template.expression`~10`, TEST_SCOPE).value, -11);

		t.false(tryStaticEvaluation(template.expression`!true`, TEST_SCOPE).bailed);
		t.is(
			tryStaticEvaluation(template.expression`!true`, TEST_SCOPE).value,
			false,
		);
	},
);

test(
	"successfully evaluates binary expressions",
	(t) => {
		t.is(tryStaticEvaluation(template.expression`2 + 5;`, TEST_SCOPE).value, 7);

		t.is(
			tryStaticEvaluation(template.expression`10 * 5;`, TEST_SCOPE).value,
			50,
		);

		t.is(
			tryStaticEvaluation(template.expression`20 / 2;`, TEST_SCOPE).value,
			10,
		);

		t.is(tryStaticEvaluation(template.expression`13 % 5;`, TEST_SCOPE).value, 3);

		t.is(
			tryStaticEvaluation(template.expression`10 | 6;`, TEST_SCOPE).value,
			14,
		);

		t.is(
			tryStaticEvaluation(template.expression`20 & 20;`, TEST_SCOPE).value,
			20,
		);

		t.is(tryStaticEvaluation(template.expression`20 & 2;`, TEST_SCOPE).value, 0);

		t.is(
			tryStaticEvaluation(template.expression`60 >> 2;`, TEST_SCOPE).value,
			15,
		);

		t.is(
			tryStaticEvaluation(template.expression`100 >>> 3;`, TEST_SCOPE).value,
			12,
		);

		t.is(
			tryStaticEvaluation(template.expression`100 << 2;`, TEST_SCOPE).value,
			400,
		);

		t.is(tryStaticEvaluation(template.expression`10 ^ 2;`, TEST_SCOPE).value, 8);
	},
);

test(
	"bails for non-matching binary operators",
	(t) => {
		t.true(
			tryStaticEvaluation(
				template.expression`listlike instanceof array;`,
				TEST_SCOPE,
			).bailed,
		);

		t.true(
			tryStaticEvaluation(template.expression`prop in obj;`, TEST_SCOPE).bailed,
		);
	},
);

test(
	"evaluates template literals",
	(t) => {
		const node = jsTemplateLiteral.create({
			quasis: [
				jsTemplateElement.create({cooked: "hello ", raw: "hello "}),
				jsTemplateElement.create({cooked: " world", raw: " world"}),
			],
			expressions: [template.expression`2+2;`],
		});
		t.false(tryStaticEvaluation(node, TEST_SCOPE).bailed);
		t.is(tryStaticEvaluation(node, TEST_SCOPE).value, "hello 4 world");
	},
);

test(
	"evaluated reference identifiers",
	(t) => {
		t.false(
			tryStaticEvaluation(
				jsReferenceIdentifier.create({name: "undefined"}),
				TEST_SCOPE,
			).bailed,
		);
		t.is(
			tryStaticEvaluation(
				jsReferenceIdentifier.create({name: "undefined"}),
				TEST_SCOPE,
			).value,
			undefined,
		);
	},
);
