import {createIntegrationTest} from "@romejs/core/integrationTestHelpers";
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
			await client.query({commandName: "lint"});
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
			await client.query({commandName: "lint", commandFlags: {save: true}});
		},
	),
);
