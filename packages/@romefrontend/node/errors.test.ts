import {test} from "rome";
import {convertPossibleNodeError} from "./errors";
import {initErrorHooks} from "@romefrontend/v8";

// Always on in Rome processes
initErrorHooks();

test(
	"ENOENT conversion",
	(t) => {
		const nodeErr: NodeJS.ErrnoException = new Error();
		nodeErr.code = "ENOENT";
		nodeErr.path = "/foo";

		const betterErr = convertPossibleNodeError(nodeErr);
		t.inlineSnapshot(betterErr.message, "'/foo' does not exist");
		t.true(
			betterErr.stack !== undefined &&
			!betterErr.stack.includes("changeMessage"),
		);
	},
);
