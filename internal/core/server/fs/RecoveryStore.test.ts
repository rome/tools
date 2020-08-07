import {createIntegrationTest} from "@internal/test-helpers";
import {test} from "rome";

const BAD_FILE = "\n\nfoobar();";
const GOOD_FILE = "foobar();\n";

test(
	"recover pop",
	createIntegrationTest(
		{
			files: {
				"index.js": BAD_FILE,
			},
		},
		async (t, {client, server, readFile}) => {
			// Fix files
			await client.query({commandName: "check", commandFlags: {apply: true}});

			// Verify that it's been fixed
			t.is(await readFile("index.js"), GOOD_FILE);

			// Verify that we created a store
			t.true((await server.recoveryStore.getAllStores()).stores.length === 1);

			// Run `recover apply`
			await client.query({
				commandName: "recover pop",
				silent: true,
			});

			// Verify that it's been reverted
			t.is(await readFile("index.js"), BAD_FILE);
		},
	),
);
