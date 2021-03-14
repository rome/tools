import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";
import {AbsoluteFilePath, createRelativePath} from "@internal/path";
import {ResolverQueryResponseFound} from "./Resolver";

function foundToRelativePath(
	cwd: AbsoluteFilePath,
	res: ResolverQueryResponseFound,
): string {
	return cwd.relative(res.path.assertAbsolute()).join();
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
						source: createRelativePath("./index"),
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
						source: createRelativePath("./index"),
						platform: "ios",
					}),
				),
				"index.ios.js",
			);
		},
	),
);
