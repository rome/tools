import {test} from "rome";
import {catchDiagnosticsSync} from "@romefrontend/diagnostics";
import {markupToPlainText} from "./format";
import {printDiagnosticsToString} from "@romefrontend/cli-diagnostics";
import {ob1Coerce1} from "@romefrontend/ob1";

const SYNTAX_ERROR_TESTS = [
	`<view pointerChar="<emphasis" pointerLine="1" pointerStart="1" pointerEnd="3">foobar</view>`,
];

test(
	"should produce syntax errors",
	async (t) => {
		for (const input of SYNTAX_ERROR_TESTS) {
			const {diagnostics} = catchDiagnosticsSync(() => {
				markupToPlainText(
					input,
					{
						columns: ob1Coerce1(400),
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
