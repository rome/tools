import {test} from "rome";
import {createIntegrationTest} from "../../test-helpers";

test(
	"ServerRequest#watchFilesFromArgs extensions",
	createIntegrationTest(
		{},
		async (t, h) => {
			const req = await h.createRequest();
			const basenames: Array<string> = [];
			const sub = await req.watchFilesFromArgs(
				{
					extensions: ["txt"],
					ignoreArgumentMisses: true,
				},
				async ({paths}) => {
					for (const path of paths) {
						basenames.push(path.getBasename());
					}
				},
			);
			await h.writeFile("foo.js", "bar");
			await h.writeFile("foo.txt", "bar");
			await sub.unsubscribe();
			t.inlineSnapshot(basenames, "Array []");
		},
	),
);
