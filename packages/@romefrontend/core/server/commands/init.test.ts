import {test} from "rome";
import {createIntegrationTest} from "@romefrontend/test-helpers";
import {dedent} from "@romefrontend/string-utils";

test(
	"should create the .editorconfig file with correct extensions if it doesn't exist and add 'unknownVariable' to globals",
	createIntegrationTest(
		{
			files: {
				"index.js": "unknownVariable",
				"config.yml": dedent`
                something: foo
                  - else; bar
                `,
			},
		},
		async (t, {client}) => {
			await client.query({commandName: "init"});
		},
	),
);

test(
	"should not create the .editorconfig file if it exists",
	createIntegrationTest(
		{
			files: {
				".editorconfig": "",
			},
		},
		async (t, {client}) => {
			await client.query({commandName: "init"});
		},
	),
);
