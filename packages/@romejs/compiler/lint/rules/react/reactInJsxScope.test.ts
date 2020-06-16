import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"react in jsx scope",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						export function HelloWorldComponent() {
							return <div>
									Hello World!!!
							</div>;
						}
					`,
				],
				valid: [
					dedent`
						import React from "react";
						export function HelloWorldComponent() {
							return <div>
									Hello World!!!
							</div>;
						}
					`,
				],
			},
			{category: "lint/react/reactInJsxScope"},
		);
	},
);
