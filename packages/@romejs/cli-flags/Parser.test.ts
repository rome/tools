/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TestHelper, test} from 'rome';
import Parser from './Parser';
import {Reporter} from '@romejs/cli-reporter';
import {Consumer} from '@romejs/consume';
import {catchDiagnostics} from '@romejs/diagnostics';
import {printDiagnostics} from '@romejs/cli-diagnostics';

async function testParser<T>(
	t: TestHelper,
	{
		defineFlags,
		args,
		callback,
	}: {
		defineFlags: (consumer: Consumer) => T;
		args: Array<string>;
		callback?: (parser: Parser<T>, flags: T) => void;
	},
) {
	const reporter = new Reporter();
	const stream = reporter.attachCaptureStream();

	const parser = new Parser(
		reporter,
		{
			programName: 'test',
			defineFlags,
		},
		args,
	);

	const {diagnostics} = await catchDiagnostics(async () => {
		const flags = await parser.init();
		t.namedSnapshot('flags', flags);
		if (callback !== undefined) {
			callback(parser, flags);
		}
	});

	if (diagnostics !== undefined) {
		printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				reporter,
			},
		});
	}

	t.namedSnapshot('output', stream.read());

	const helpStream = reporter.attachCaptureStream();
	await parser.showHelp();
	t.namedSnapshot('help', helpStream.read());
}

test(
	'does not allow shorthands',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: () => {},
				args: ['-f'],
			},
		);
	},
);

test(
	'does not allow camel case',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: () => {},
				args: ['--fooBar'],
			},
		);
	},
);

test(
	'flag value equals',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get('name').asString(),
					};
				},
				args: ['--name=sebastian'],
			},
		);
	},
);

test(
	'required flag',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get('name').asString(),
					};
				},
				args: ['--name', 'sebastian'],
			},
		);
	},
);

test(
	'required flag omitted',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get('name').asString(),
					};
				},
				args: [],
			},
		);
	},
);

test(
	'optional flag',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get('name').asStringOrVoid(),
					};
				},
				args: ['--name', 'sebastian'],
			},
		);
	},
);

test(
	'optional flag omitted',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get('name').asStringOrVoid(),
					};
				},
				args: [],
			},
		);
	},
);

test(
	'flag with description',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						name: c.get(
							'name',
							{
								description: 'the name of the coolest person in the world',
							},
						).asStringOrVoid(),
					};
				},
				args: ['--name', 'sebastian'],
			},
		);
	},
);

test(
	'optional boolean flag',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get('run').asBooleanOrVoid(),
					};
				},
				args: ['--run'],
			},
		);
	},
);

test(
	'optional boolean flag omitted',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get('run').asBooleanOrVoid(),
					};
				},
				args: ['--run'],
			},
		);
	},
);

test(
	'required boolean flag',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get('run').asBoolean(),
					};
				},
				args: ['--run'],
			},
		);
	},
);

test(
	'required boolean flag omitted',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get('run').asBoolean(),
					};
				},
				args: ['--run'],
			},
		);
	},
);

test(
	'flip boolean flag',
	async (t) => {
		await testParser(
			t,
			{
				defineFlags: (c) => {
					return {
						run: c.get('run').asBoolean(),
					};
				},
				args: ['--no-run'],
			},
		);
	},
);
