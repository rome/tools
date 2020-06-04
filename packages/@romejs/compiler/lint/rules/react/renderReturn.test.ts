import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"react render return",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						import React from "react";
						class Hello extends React.Component {
							render() {}
						}
					`,
					dedent`
						import React from "react";
						class Hello extends React.Component {
							render() {
								[1, 2, 3].map((num) => {
									return <div> Foo </div>
								});
							}
						}
					`,
					dedent`
						import React from "react";
						class Hello extends React.Component {
							render = () => { }
						}
					`,
				],

				valid: [
					dedent`
						import React from "react";
						class Hello extends React.Component {
							render() {
								return <div>Foo</div>;
							}
						}
					`,
					dedent`
						import React from "react";
						class Hello extends React.Component {
							render() {
								if (foo) {
									return <div>Foo</div>;
								} else {
									return void 0;
								}
							}
						}
					`,
					dedent`
						class Hello {
							render = () => { return <></> }
						}
					`,
					dedent`
						class Hello {
							render = () => (<></>)
						}
					`,
				],
			},
			{category: "lint/react/renderReturn"},
		);
	},
);
