import {test} from "rome";
import {createIntegrationTest} from "@romejs/core/integrationTestHelpers";
import {AbsoluteFilePath, createUnknownFilePath} from "@romejs/path";
import {ResolverQueryResponseFound} from "./Resolver";

function foundToRelativePath(
	cwd: AbsoluteFilePath,
	res: ResolverQueryResponseFound,
): string {
	return cwd.relative(res.path).join();
}

test(
	"smoke",
	createIntegrationTest(
		{
			files: {
				"index.js": "",
				"index.ios.js": "",
			},
		},
		async (t, {cwd, master}) => {
			// Implicit extension
			t.inlineSnapshot(
				foundToRelativePath(
					cwd,
					await master.resolver.resolveEntryAssert({
						origin: cwd,
						source: createUnknownFilePath("./index"),
					}),
				),
				"index.js",
			);

			// Platform
			t.inlineSnapshot(
				foundToRelativePath(
					cwd,
					await master.resolver.resolveEntryAssert({
						origin: cwd,
						source: createUnknownFilePath("./index"),
						platform: "ios",
					}),
				),
				"index.ios.js",
			);
		},
	),
);
