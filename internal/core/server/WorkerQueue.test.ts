import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";
import WorkerQueue from "./WorkerQueue";
import {createAbsoluteFilePath} from "@internal/path";

test(
	"prepare with unknown file path",
	createIntegrationTest(
		{},
		async (t, h) => {
			const queue = new WorkerQueue(
				h.server,
				{
					async callback() {},
				},
			);
			await queue.prepare([createAbsoluteFilePath("/doesntexist")]);
		},
	),
);
