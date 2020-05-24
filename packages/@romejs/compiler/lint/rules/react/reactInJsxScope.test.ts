import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react in jsx scope",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
				export function HelloWorldComponent() {
					return <div>
							Hello World!!!
					</div>;
				}
				`,
				// VALID
				`
				import React from "react";
				export function HelloWorldComponent() {
					return <div>
							Hello World!!!
					</div>;
				}
				`,
			],
			{category: "lint/react/reactInJsxScope"},
		);
	},
);
