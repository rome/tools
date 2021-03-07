import {test} from "rome";
import {catchDiagnosticsSync} from "@internal/diagnostics";
import {markupToAnsi, markupToPlainText} from "./format";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import {markup} from "../markup/escape";
import {joinMarkupLines} from "@internal/markup";
import {OneIndexed} from "@internal/numbers";

const SYNTAX_ERROR_TESTS = [
	markup`<view pointerChar="<emphasis" pointerLine="1" pointerStart="1" pointerEnd="3">foobar</view>`,
];

test(
	"should produce syntax errors",
	async (t) => {
		for (const input of SYNTAX_ERROR_TESTS) {
			const {diagnostics} = catchDiagnosticsSync(() => {
				markupToPlainText(
					input,
					{
						columns: new OneIndexed(400),
					},
				);
			});

			if (diagnostics === undefined) {
				throw new Error(`"${input}" should have thrown a syntax error`);
			} else {
				t.snapshot(
					await printDiagnosticsToString({
						diagnostics,
						suppressions: [],
					}),
				);
			}
		}
	},
);

test(
	"regression #971: format ranges spanning multiple lines",
	(t) => {
		t.inlineSnapshot(
			joinMarkupLines(
				markupToAnsi(
					markup`<indent><emphasis><error>✖ </error></emphasis><view><error>✖ Intercepted diagnostics
test.rjson:1:4 parse(rjson) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✖ Unclosed block comment
> 1 │ /* / error */
│     ^
2 │ {\"hey\": \"hi\"}
</error></view></indent>`,
				),
			),
			'\t\x1b[0m\x1b[1m\x1b[31m\u2716 \x1b[39m\x1b[22m\x1b[0m\x1b[0m\x1b[31m\u2716 Intercepted diagnostics\x1b[39m\x1b[0m\n\t\t\x1b[0m\x1b[31mtest.rjson:1:4 parse(rjson) \u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\u2501\x1b[39m\x1b[0m\n\t\t\x1b[0m\x1b[31m\u2716 Unclosed block comment\x1b[39m\x1b[0m\n\t\t\x1b[0m\x1b[31m> 1 \u2502 /* / error */\x1b[39m\x1b[0m\n\t\t\x1b[0m\x1b[31m\u2502     ^\x1b[39m\x1b[0m\n\t\t\x1b[0m\x1b[31m2 \u2502 {"hey": "hi"}\x1b[39m\x1b[0m\n',
		);
	},
);
