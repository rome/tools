import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";

test(
	"--apply should create the .editorconfig file with correct extensions if it doesn't exist and add 'unknownVariable' to globals",
	createIntegrationTest(
		{
			disableProjectConfig: true,
			files: {
				"index.js": "unknownVariable; window;",
			},
		},
		async (t, {client}) => {
			await client.query({
				commandName: "init",
				commandFlags: {apply: true, allowDirty: true},
			});
		},
	),
);

test(
	"should not create the .editorconfig file if it exists",
	createIntegrationTest(
		{
			disableProjectConfig: true,
			files: {
				".editorconfig": "",
			},
		},
		async (t, {client}) => {
			await client.query({commandName: "init", commandFlags: {allowDirty: true}});
		},
	),
);

test(
	"should bail if a project already exists",
	createIntegrationTest(
		{
			disableProjectConfig: true,
			files: {},
		},
		async (t, {client}) => {
			await client.query({commandName: "init", commandFlags: {allowDirty: true}});
			await client.query({commandName: "init", commandFlags: {allowDirty: true}});
		},
	),
);

/*test(
	"should not allow project creation outside a repository",
	createIntegrationTest(
		{
			gitInitialize: false,
		},
		async (t, {client}) => {
			await client.query({commandName: "init"});
		},
	),
);

test(
	"should allow project creation inside a repository",
	createIntegrationTest(
		{
			gitInitialize: true,
		},
		async (t, {client}) => {
			await client.query({commandName: "init"});
		},
	),
);

test(
	"should not allow project creation inside a repository with uncommitted changes",
	createIntegrationTest(
		{
			gitInitialize: true,
		},
		async (t, {client, writeFile}) => {
			await writeFile("foo", "bar");
			await client.query({commandName: "init"});
		},
	),
);*/
