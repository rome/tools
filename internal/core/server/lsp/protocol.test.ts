import {TestHelper, test} from "rome";
import {LSPTransport} from "./protocol";
import {Reporter} from "@internal/cli-reporter";

function createTransportTest(
	callback: (transport: LSPTransport) => void,
): (t: TestHelper) => void {
	return function(t: TestHelper) {
		const reporter = new Reporter();

		const stream = reporter.attachCaptureStream();

		const transport = new LSPTransport(reporter);

		callback(transport);

		t.snapshot(stream.read());
	};
}

test(
	"LSPTransport handles emoji byte lengths",
	createTransportTest((transport) => {
		transport.append(
			[
				"Content-Length: 321",
				"",
				`{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"file:///Users/sebastianmckenzie/Scratch/rome-playground/test.ts","version":24},"contentChanges":[{"text":"//Can’t handle this comment\nconst foo = 'Or this “special” string';\nconst rocket = \"Or this🚀\";\n\nrocket;\nfoo;"}]}}`,
			].join("\r\n"),
		);
	}),
);
