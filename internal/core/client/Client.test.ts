import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";

test(
	"Client#generateRageSummary",
	createIntegrationTest(
		{},
		async (t, helper) => {
			await t.notThrowsAsync(async () => {
				await helper.client.generateRageSummary();
			});
		},
	),
);
