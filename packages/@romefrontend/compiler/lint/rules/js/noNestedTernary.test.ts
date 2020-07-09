import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"js no nested ternary",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"let thing = foo ? bar : baz === qux ? quxx : foobar;",
					"let thing = foo ? bar ? boo : foo : baz ? boo : foo;",
					"foo ? baz === qux ? quxx() : foobar() : bar();",
				],
				valid: [
					"let thing = foo ? bar : foobar;",
					"let thing = foo ? bar || boo : foo || bar;",
					"let thing = foo ? bar && boo : foo && bar;",
					"let thing = foo || baz ? bar || boo : foo || bar;",
					"let thing = foo && baz ? bar || boo : foo && bar;",
					"let thing = foo || baz ? bar || boo : foo && bar;",
					`
						if (foo) {
							thing = bar;
						} else if (baz === qux) {
							thing = quxx;
						} else {
							thing = foobar;
						}
					`,
				],
			},
			{category: "lint/js/noNestedTernary"},
		);
	},
);
