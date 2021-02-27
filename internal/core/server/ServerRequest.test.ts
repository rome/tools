import {test} from "rome";
import {createIntegrationTest} from "../../test-helpers";

test(
	"ServerRequest#glob watch extensions",
	createIntegrationTest(
		{},
		async (t, h) => {
			const req = await h.createRequest();
			const basenames: string[] = [];
			const globber = await req.glob({
				extensions: ["txt"],
				ignoreArgumentMisses: true,
			});
			const sub = await globber.watch(async ({paths}) => {
				for (const path of paths) {
					basenames.push(path.getBasename());
				}
			});
			await h.writeFile("foo.js", "bar");
			await h.writeFile("foo.txt", "bar");
			await sub.release();
			t.inlineSnapshot(basenames, 'Array [\n\t"foo.txt"\n\t"foo.txt"\n]');
		},
	),
);

test(
	"ServerRequest#glob watch evicted project updates",
	createIntegrationTest(
		{
			files: {
				"index.ts": 'import "unknown-module";',
			},
		},
		async (t, h) => {
			const req = await h.createRequest({commandName: "check"});

			const events: string[] = [];

			const beforeProject = await h.server.projectManager.assertProject(h.cwd);
			const globber = await req.glob({});
			const sub = await globber.watch(async ({initial, paths}) => {
				for (const path of paths) {
					events.push(
						`initial: ${initial}, path: ${h.cwd.relative(path).join()}`,
					);
				}
			});

			await h.writeFile(".config/rome.rjson", "");

			await sub.release();

			const afterProject = await h.server.projectManager.assertProject(h.cwd);
			t.not(beforeProject.id, afterProject.id);
		},
	),
);
