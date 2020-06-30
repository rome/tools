import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romejs/string-utils";

test(
	"react filename extension [.js]",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class MyComponent extends React.Component {
							render() {
 						  		return <div class="one" />;
							}
						}
					`,
				],
				valid: [""],
			},
			{category: "lint/react/filenameExtension", path: "foo.js"},
		);

		await testLint(
			t,
			{
				invalid: [
					dedent`
						class MyComponent extends React.Component {
							render() {
 						  		return <div class="one" />;
							}
						}
					`,
				],
				valid: [""],
			},
			{category: "lint/react/filenameExtension", path: "foo.ts"},
		);

		await testLint(
			t,
			{
				invalid: [],
				valid: [
					dedent`
						class MyComponent extends React.Component {
							render() {
 						  		return <div class="second" />;
							}
						}
					`,
				],
			},
			{category: "lint/react/filenameExtension", path: "foo.jsx"},
		);

		await testLint(
			t,
			{
				invalid: [],
				valid: [
					dedent`
						class MyComponent extends React.Component {
							render() {
 						  		return <div class="second" />;
							}
						}
					`,
				],
			},
			{category: "lint/react/filenameExtension", path: "foo.tsx"},
		);
	},
);

// test(
// 	"react filename extension [.jsx]",
// 	async (t) => {
//
// 	},
// );
