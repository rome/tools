import {createIntegrationTest} from "@internal/test-helpers";
import {test} from "rome";

test(
	"smoke",
	createIntegrationTest(
		{
			files: {
				"index.js": "unknownVariable",
			},
		},
		async (t, {client}) => {
			await client.query({commandName: "check"});
		},
	),
);

test(
	"smoke save",
	createIntegrationTest(
		{
			files: {
				"index.js": "if (unformatted) {swag}",
			},
		},
		async (t, {client}) => {
			await client.query({commandName: "check", commandFlags: {apply: true}});
		},
	),
);

test(
	"only checks files matching arguments",
	createIntegrationTest(
		{
			files: {
				"foo.js": "let foo",
				"bar.js": "let bar",
			},
		},
		async (t, {client}) => {
			const res = await client.query({
				commandName: "check",
				args: ["foo.js"],
				commandFlags: {apply: true},
			});

			t.true(res.type === "DIAGNOSTICS");
			if (res.type === "DIAGNOSTICS") {
				t.true(res.diagnostics.length === 1);
			}
		},
	),
);
