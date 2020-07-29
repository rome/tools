/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper, test} from "rome";
import Parser, {ParserOptions} from "./Parser";
import {Reporter} from "@romefrontend/cli-reporter";
import {Consumer} from "@romefrontend/consume";
import {
	DiagnosticsProcessor,
	catchDiagnostics,
} from "@romefrontend/diagnostics";
import {printDiagnostics} from "@romefrontend/cli-diagnostics";
import {markup} from "@romefrontend/markup";

async function testParser<T>(
	t: TestHelper,
	{
		defineFlags,
		args,
		preInit,
		postInit,
		options: opts,
	}: {
		defineFlags: (consumer: Consumer) => T;
		args: Array<string>;
		preInit?: (parser: Parser<T>) => void;
		postInit?: (parser: Parser<T>, flags: T) => void;
		options?: Partial<ParserOptions<T>>;
	},
) {
	const reporter = new Reporter();
	const stream = reporter.attachCaptureStream();

	const parser = new Parser(
		reporter,
		{
			...opts,
			programName: "test",
			defineFlags,
		},
		args,
	);

	const {diagnostics} = await catchDiagnostics(async () => {
		if (preInit !== undefined) {
			preInit(parser);
		}
		const flags = await parser.init();
		t.namedSnapshot("flags", flags);
		if (postInit !== undefined) {
			postInit(parser, flags);
		}
	});

	if (diagnostics !== undefined) {
		await printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				processor: new DiagnosticsProcessor(),
				reporter,
			},
		});
	}

	t.namedSnapshot("output", stream.read());

	const helpStream = reporter.attachCaptureStream();
	await parser.showHelp();
	t.namedSnapshot("help", helpStream.read());
}

test(
	"does not allow shorthands",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: () => {},
				args: ["-f"],
			},
		);
	},
);

test(
	"does not allow camel case",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: () => {},
				args: ["--fooBar"],
			},
		);
	},
);

test(
	"flag value equals",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get("name").asString(),
					};
				},
				args: ["--name=sebastian"],
			},
		);
	},
);

test(
	"required flag",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get("name").asString(),
					};
				},
				args: ["--name", "sebastian"],
			},
		);
	},
);

test(
	"required flag omitted",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get("name").asString(),
					};
				},
				args: [],
			},
		);
	},
);

test(
	"optional flag",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get("name").asStringOrVoid(),
					};
				},
				args: ["--name", "sebastian"],
			},
		);
	},
);

test(
	"optional flag omitted",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get("name").asStringOrVoid(),
					};
				},
				args: [],
			},
		);
	},
);

test(
	"flag with description",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get(
							"name",
							{
								description: markup`the name of the coolest person in the world`,
							},
						).asStringOrVoid(),
					};
				},
				args: ["--name", "sebastian"],
			},
		);
	},
);

test(
	"optional boolean flag",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get("run").asBooleanOrVoid(),
					};
				},
				args: ["--run"],
			},
		);
	},
);

test(
	"optional boolean flag omitted",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get("run").asBooleanOrVoid(),
					};
				},
				args: ["--run"],
			},
		);
	},
);

test(
	"required boolean flag",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get("run").asBoolean(),
					};
				},
				args: ["--run"],
			},
		);
	},
);

test(
	"required boolean flag omitted",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get("run").asBoolean(),
					};
				},
				args: ["--run"],
			},
		);
	},
);

test(
	"flip boolean flag",
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get("run").asBoolean(),
					};
				},
				args: ["--no-run"],
			},
		);
	},
);

test(
	"command required with missing command",
	async (t) => {
		await testParser(
			t,
			{
				options: {
					commandRequired: true,
				},
				defineFlags: () => {
					return {};
				},
				args: [],
			},
		);
	},
);

test(
	"command required with no command but with --help flag",
	async (t) => {
		await testParser(
			t,
			{
				options: {
					commandRequired: true,
					noProcessExit: true,
				},
				defineFlags: () => {
					return {};
				},
				args: ["--help"],
				preInit(p) {
					p.addCommand({
						name: "foo",
						callback() {},
					});
					p.addCommand({
						name: "bar",
						callback() {},
					});
				},
			},
		);
	},
);

test(
	"command required with wrong command",
	async (t) => {
		await testParser(
			t,
			{
				options: {
					commandRequired: true,
				},
				defineFlags: () => {
					return {};
				},
				args: ["foo"],
				preInit(p) {
					p.addCommand({
						name: "foobar",
						callback() {},
					});
				},
			},
		);
	},
);

test(
	"command required with wrong command and suggestion",
	async (t) => {
		await testParser(
			t,
			{
				options: {
					commandRequired: true,
					commandSuggestions: {
						foo: {
							commandName: "foobar",
							description: markup`A much cooler command`,
						},
					},
				},
				defineFlags: () => {
					return {};
				},
				args: ["foo"],
			},
		);
	},
);
