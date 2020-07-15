import {test} from "rome";
import {testLint} from "../../utils/testing";

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
				filename: "file.tsx",
				category: "lint/jsx/pascalCase",
			},
		);
	},
);
