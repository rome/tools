import {ReporterStreamAttached} from "./types";
import {DEFAULT_TERMINAL_FEATURES} from "@internal/cli-environment";
import {TestHelper, test} from "rome";
import Reporter from "./Reporter";
import {markup} from "@internal/markup";

type StreamTestHelper = {
	reporter: Reporter;
	stream: ReporterStreamAttached;
	intermediateSnapshot(): void;
};

function createStreamTest(callback: (helper: StreamTestHelper) => void) {
	return (t: TestHelper) => {
		let buff = "";

		const reporter = new Reporter();

		const {stream} = reporter.addStream({
			format: "ansi",
			features: {
				...DEFAULT_TERMINAL_FEATURES,
				cursor: true,
			},
			write(chunk) {
				buff += chunk;
			},
		});

		let count = 0;

		function intermediateSnapshot(prefix: string = String(count++)) {
			t.namedSnapshot(`${prefix} raw`, buff);
			t.namedSnapshot(`${prefix} state`, stream.state);
		}

		callback({reporter, stream, intermediateSnapshot});
		intermediateSnapshot("final");
	};
}

test(
	"removes same line",
	createStreamTest(({reporter}) => {
		const lineSnapshot = reporter.getLineSnapshot();
		reporter.log(
			markup`remove me`,
			{
				// Wont have advanced to the next line
				noNewline: true,
			},
		);
		reporter.removeLine(lineSnapshot);
	}),
);

test(
	"removes higher line",
	createStreamTest(({reporter}) => {
		reporter.log(markup`1`);
		reporter.log(markup`2`);
		const lineSnapshot = reporter.getLineSnapshot();
		reporter.log(markup`5`);
		reporter.log(markup`3`);
		reporter.log(markup`4`);
		reporter.removeLine(lineSnapshot);
	}),
);

test(
	"replaces same line",
	createStreamTest(({reporter}) => {
		const lineSnapshot = reporter.getLineSnapshot();
		reporter.log(
			markup`remove me`,
			{
				// Wont have advanced to the next line
				noNewline: true,
			},
		);
		reporter.log(
			markup`ok i did it`,
			{
				replaceLineSnapshot: lineSnapshot,
			},
		);
	}),
);

test(
	"replaces higher line",
	createStreamTest(({reporter}) => {
		reporter.log(markup`1`);
		reporter.log(markup`2`);
		const lineSnapshot = reporter.getLineSnapshot();
		reporter.log(markup`5`);
		reporter.log(markup`4`);
		reporter.log(
			markup`3`,
			{
				replaceLineSnapshot: lineSnapshot,
			},
		);
	}),
);

test(
	"nextLineInsertLeadingNewline will insert newline on next log",
	createStreamTest(({reporter, intermediateSnapshot}) => {
		reporter.log(
			markup`1`,
			{
				preferNoNewline: true,
			},
		);
		intermediateSnapshot();
		reporter.log(markup`2`);
	}),
);
