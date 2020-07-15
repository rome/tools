import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"react require render return",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						import React from "react";
						class Foo extends React.Component {
							render() {}
						}
					`,
					dedent`
						import React from "react";
						class Foo extends React.Component {
							render() {
								[1, 2, 3].map((num) => {
									return <div> Foo </div>
								});
							}
						}
					`,
					dedent`
						import React from "react";
						class Foo extends React.Component {
							render = () => { }
						}
					`,
					dedent`
						import React, {Component} from "react";
						class Foo extends Component {
							render = () => { }
						}
					`,
					dedent`
						import React from "react";
						const Foo = class extends React.Component {
							render = () => { }
						}
					`,
					dedent`
						import React, {Component} from "react";
						const Foo = class extends Component {
							render = () => { }
						}
					`,
					dedent`
						import React from "react";
						const Foo = class extends React.PureComponent {
							render = () => { }
						}
					`,
					dedent`
						import React, {PureComponent} from "react";
						const Foo = class extends PureComponent {
							render = () => { }
						}
					`,
				],

				valid: [
					dedent`
						import React from "react";
						class Foo extends React.Component {
							render() {
								return <div>Foo</div>;
							}
						}
					`,
					dedent`
						import React from "react";
						class Foo extends React.Component {
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
						class Foo {
							render = () => { return <></> }
						}
					`,
					dedent`
						class Foo {
							render = () => (<></>)
						}
					`,
					dedent`
						class Foo extends Bar {
							render() { }
						}
					`,
				],
				filename: "file.tsx",
				category: "lint/react/requireRenderReturn",
			},
		);
	},
);
