import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";
import {ProjectDefinition} from "@internal/project";

test(
	"nested projects",
	createIntegrationTest(
		{
			projectConfig: {
				name: "foo",
				root: true,
			},
			files: {
				"bar/.config/rome.rjson": `name: "bar"`,
				"bar/test.ts": "",
			},
		},
		async (t, h) => {
			function check(project: ProjectDefinition) {
				t.is(project.config.name, "bar");
				if (project.parent === undefined) {
					throw new Error("Expected project parent");
				} else {
					t.is(project.parent.config.name, "foo");
				}
			}

			check(await h.server.projectManager.assertProject(h.cwd.append("bar")));
			check(
				await h.server.projectManager.assertProject(h.cwd.append("bar/test.ts")),
			);
		},
	),
);
