import {test} from "rome";
import {convertPossibleNodeErrorToDiagnostic} from "./errors";
import {initErrorHooks} from "@romefrontend/v8";

// Always on in regular Rome processes
initErrorHooks();

test(
	"ENOENT conversion",
	(t) => {
		const nodeErr: NodeJS.ErrnoException = new Error();
		nodeErr.code = "ENOENT";
		nodeErr.path = "/foo";

		const betterErr = convertPossibleNodeErrorToDiagnostic(nodeErr);
		t.inlineSnapshot(
			betterErr.message,
			"<emphasis>/foo</emphasis> does not exist\n\n /foo internalError/fs  OUTDATED  \u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\n\n  \u2716 /foo does not exist\n\n  \u26a0 This diagnostic may be out of date as it relies on the following files that have been changed since the diagnostic was generated\n\n\n  - /foo\n  - ~/Projects/rome/packages/node/errors.test.ts\n  - /private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js\n\n\n   1. <anonymous> (~/Projects/rome/packages/node/errors.test.ts:106329:19)\n   2. <anonymous> (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:140375:17)\n   3. ___R$project$rome$$romefrontend$diagnostics$wrap_ts$catchDiagnostics (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:102359:23)\n   4. ___R$project$rome$$romefrontend$core$test$worker$TestWorkerRunner_ts$default.runTest (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:140374:32)\n   5. ___R$project$rome$$romefrontend$core$test$worker$TestWorkerRunner_ts$default.run (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:140439:25)\n   6. ___R$project$rome$$romefrontend$core$test$worker$TestWorker_ts$default.runTest (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:141429:24)\n   7. <anonymous> (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:141417:16)\n   8. ___R$project$rome$$romefrontend$events$BridgeEvent_ts$default.call (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:49296:6)\n   9. ___R$project$rome$$romefrontend$events$BridgeEvent_ts$default.dispatchRequest (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:49467:16)\n  10. ___R$project$rome$$romefrontend$core$common$bridges$TestWorkerBridge_ts$default.handleMessageRequest (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:49987:17)\n  11. ___R$project$rome$$romefrontend$core$common$bridges$TestWorkerBridge_ts$default.handleMessage (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:49939:10)\n  12. process.<anonymous> (/private/var/folders/qf/t00z772x7m90vb0pwlfpt0qw0000gp/T/rome-dev/index.js:50311:11)\n  13. process.emit (events.js:315:19)\n  14. emit (internal/child_process.js:906:11)\n  15. process.processTicksAndRejections (internal/process/task_queues.js:85:20)\n\n  \u26a0 This diagnostic was derived from an internal Rome error. Possible bug.\n\n",
		);
		t.true(
			betterErr.stack !== undefined &&
			!betterErr.stack.includes("changeMessage"),
		);
	},
);
