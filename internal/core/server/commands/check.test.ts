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
