import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";
import WorkerQueue from "@internal/core/server/WorkerQueue";
import {createAbsoluteFilePath} from "@internal/path";

test("prepare with unknown file path", createIntegrationTest({}, async (t, h) => {
	const queue = new WorkerQueue(h.server);
	await queue.prepare([createAbsoluteFilePath("/doesntexist")]);
}));
