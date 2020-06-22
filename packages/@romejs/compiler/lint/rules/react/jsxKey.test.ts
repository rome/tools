/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {dedent} from "@romejs/string-utils";
import {testLint} from "../../utils/testing";

test(
	"react jsx key",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"const a = [<div />, <div />]",
					"const a = [1, 2].map(x => <div>{x}</div>);",
					"React.Children.map(children, x => <div>{x}</div>);",
					"Children.map(children, x => <div>{x}</div>);",
					dedent`
						const a = [1, 2].map(x => {
							return <div>{x}</div>;
						});
					`,
					dedent`
						React.Children.map(children, x => {
							return <div>{x}</div>;
						});
					`,
					dedent`
						Children.map(children, x => {
							return <div>{x}</div>;
						});
					`,
					dedent`
						const a = [1, 2].map(function(x) {
							return <div>{x}</div>;
						});
					`,
					dedent`
						React.Children.map(children, function(x) {
							return <div>{x}</div>;
						});
					`,
					dedent`
						Children.map(children, function(x) {
							return <div>{x}</div>;
						});
					`,
				],
				valid: [
					'const a = [<div key="a" />, <div key={"b"} />]',
					"const a = [1, 2].map(x => <div key={x}>{x}</div>)",
					"React.Children.map(children, x => <div key={x}>{x}</div>)",
					"Children.map(children, x => <div key={x}>{x}</div>)",
					dedent`
						const a = [1, 2].map(x => {
							return <div key={x}>{x}</div>;
						});
					`,
					dedent`
						React.Children.map(children, x => {
							return <div key={x}>{x}</div>;
						});
					`,
					dedent`
						Children.map(children, x => {
							return <div key={x}>{x}</div>;
						});
					`,
					dedent`
						const a = [1, 2].map(function(x) {
							return <div key={x}>{x}</div>;
						});
					`,
					dedent`
						React.Children.map(children, function(x) {
							return <div key={x}>{x}</div>;
						});
					`,
					dedent`
						Children.map(children, function(x) {
							return <div key={x}>{x}</div>;
						});
					`,
				],
			},
			{category: "lint/react/jsxKey"},
		);
	},
);
