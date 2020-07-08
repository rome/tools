import {test} from "rome";
import {createIntegrationTest} from "@romefrontend/core/integrationTestHelpers";
import {AbsoluteFilePath, createUnknownFilePath} from "@romefrontend/path";
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
		async (t, {cwd, server}) => {
			// Implicit extension
			t.inlineSnapshot(
				foundToRelativePath(
					cwd,
					await server.resolver.resolveEntryAssert({
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
					await server.resolver.resolveEntryAssert({
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
