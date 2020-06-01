import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react jsx no literals",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<Foo> bar </Foo>",
					"<Foo> 'bar' </Foo>",
					"<Foo> {'bar'} baz {'bar'} </Foo>",
					"<Foo prop='bar'></Foo>",
				],
				valid: [
					"<Foo>  </Foo>",
					"<Foo> {'bar'} </Foo>",
					"<Foo> {bar('baz')} </Foo>",
					"<Foo prop={'baz'}> {bar('baz')} </Foo>",
				],
			},
			{category: "lint/react/jsxNoLiterals"},
		);
	},
);
