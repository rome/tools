import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react jsx pascal case",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<Foo_component />", "<FOO_COMPONENT />"],
				valid: [
					"<Foo />",
					"<foo_bar />",
					"<fooBar />",
					"<foo_COMPONENT />",
					"<foo />",
					"<div />",
					"<FooComponent />",
				],
			},
			{category: "lint/react/jsxPascalCase"},
		);
	},
);
