/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {VirtualModulesMap} from "./VirtualModules";

export const modules: VirtualModulesMap = new Map();

/* GENERATED:START(hash:62fae5f175b94703729e1cc6f8b17087fd7f0d3c,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/virtual-modules` to update. */
modules.set(
	"rome",
	new Map([
		[
			"index.ts",
			{
				"mtime": 1_596_092_146_931.4539,
				"content": '/**\n * Copyright (c) Facebook, Inc. and its affiliates.\n *\n * This source code is licensed under the MIT license found in the\n * LICENSE file in the root directory of this source tree.\n */\n\nexport {TestHelper, test, testOptions} from "./test";\n',
			},
		],
		[
			"package.json",
			{
				"mtime": 1_596_913_605_922.6692,
				"content": '{\n  "name": "@internal/virtual-rome",\n  "type": "module",\n  "private": true,\n  "main": "index.ts"\n}\n',
			},
		],
		[
			"test.ts",
			{
				"mtime": 1_596_599_768_028.357,
				"content": '/**\n * Copyright (c) Facebook, Inc. and its affiliates.\n *\n * This source code is licensed under the MIT license found in the\n * LICENSE file in the root directory of this source tree.\n */\n\nimport {AsyncVoidCallback, JSONPropertyValue, VoidCallback} from "./types";\n\nexport type ExpectedError = undefined | string | RegExp | Function;\n\nexport type TestSnapshotOptions = {\n\tfilename?: string;\n\tlanguage?: string;\n};\n\n// These diagnostics are subsets of the official diagnostics\n// We can potentially normalize these and ensure backwards compatibility with the official diagnostics\n\nexport type TestDiagnosticLogCategory = "none" | "info" | "warn" | "error";\n\nexport type TestDiagnosticAdviceInspect = {\n\ttype: "inspect";\n\tdata: JSONPropertyValue;\n};\n\nexport type TestDiagnosticAdviceList = {\n\ttype: "list";\n\tlist: Array<string>;\n};\n\nexport type TestDiagnosticAdviceCode = {\n\ttype: "code";\n\tsourceText: string;\n};\n\nexport type TestDiagnosticAdviceLog = {\n\ttype: "log";\n\tcategory: TestDiagnosticLogCategory;\n\ttext: string;\n};\n\nexport type TestDiagnosticAdviceItem =\n\t| TestDiagnosticAdviceInspect\n\t| TestDiagnosticAdviceCode\n\t| TestDiagnosticAdviceLog\n\t| TestDiagnosticAdviceList;\n\nexport interface TestHelper {\n\taddToAdvice(\n\t\titem: TestDiagnosticAdviceItem | (() => TestDiagnosticAdviceItem),\n\t): void;\n\tclearAdvice(): void;\n\tonTeardown(callback: AsyncVoidCallback): void;\n\tclearTimeout(): void;\n\textendTimeout(time: number): void;\n\tsetTimeout(time: number): void;\n\tcheckTimeout(): void;\n\ttruthy(value: unknown, message?: string): void;\n\tfalsy(value: unknown, message?: string): void;\n\ttrue(value: unknown, message?: string): void;\n\tfalse(value: unknown, message?: string): void;\n\tis(received: unknown, expected: unknown, message?: string): void;\n\tnot(received: unknown, expected: unknown, message?: string): void;\n\tlooksLike(received: unknown, expected: unknown, message?: string): void;\n\tnotLooksLike(received: unknown, expected: unknown, message?: string): void;\n\tthrows(\n\t\tthrower: VoidCallback,\n\t\texpected?: ExpectedError,\n\t\tmessage?: string,\n\t): void;\n\tthrowsAsync(\n\t\tthrower: AsyncVoidCallback,\n\t\texpected?: ExpectedError,\n\t\tmessage?: string,\n\t): Promise<void>;\n\tnotThrows(nonThrower: VoidCallback, message?: string): void;\n\tnotThrowsAsync(\n\t\tnonThrower: AsyncVoidCallback,\n\t\tmessage?: string,\n\t): Promise<void>;\n\tregex(contents: string, regex: RegExp, message?: string): void;\n\tnotRegex(contents: string, regex: RegExp, message?: string): void;\n\tsnapshot(\n\t\texpected: unknown,\n\t\tmessage?: string,\n\t\topts?: TestSnapshotOptions,\n\t): string;\n\tinlineSnapshot(received: unknown, expected?: string | boolean | number): void;\n\tnamedSnapshot(\n\t\tname: string,\n\t\texpected: unknown,\n\t\tmessage?: string,\n\t\topts?: TestSnapshotOptions,\n\t): string;\n}\n\nexport type TestName = string | Array<string>;\n\ndeclare const __ROME__TEST_OPTIONS__: GlobalTestOptions;\n\nexport type GlobalTestOptions =\n\t| undefined\n\t| {\n\t\t\tdirname?: string;\n\t\t\tregister?: (err: Error, opts: TestOptions, callback: TestCallback) => void;\n\t\t};\n\ntype NamelessTestOptions = {\n\ttimeout?: number;\n\tonly?: boolean;\n};\n\nexport type TestCallback = (t: TestHelper) => void | undefined | Promise<void>;\n\nexport type TestOptions = NamelessTestOptions & {\n\tname: TestName;\n};\n\ntype TestArg = TestName | NamelessTestOptions | TestCallback | undefined;\n\nexport const testOptions: NonNullable<GlobalTestOptions> =\n\ttypeof __ROME__TEST_OPTIONS__ === "undefined" ? {} : __ROME__TEST_OPTIONS__;\n\nfunction registerTest(\n\tcallsiteError: Error,\n\topts: TestOptions,\n\tcallback: TestCallback,\n) {\n\tconst register = testOptions.register;\n\n\tif (typeof register !== "function") {\n\t\tthrow new Error("Test harness does not exist");\n\t}\n\n\tregister(callsiteError, opts, callback);\n}\n\nfunction isOptionsObject(arg: TestArg): arg is NamelessTestOptions {\n\treturn typeof arg === "object" && arg != null && !Array.isArray(arg);\n}\n\nfunction splitArgs(\n\targs: TestRegisterFunctionArgs,\n): {\n\toptions: TestOptions;\n\tcallback: TestCallback;\n} {\n\tconst name = args.shift();\n\tif (typeof name !== "string" && !Array.isArray(name)) {\n\t\tthrow new Error("Expected test name to be a string or an array of strings");\n\t}\n\n\tconst callback = args.pop();\n\tif (typeof callback !== "function") {\n\t\tthrow new Error("Expected options callback");\n\t}\n\n\tconst options = args.pop();\n\tif (options !== undefined && !isOptionsObject(options)) {\n\t\tthrow new Error("Expected options object");\n\t}\n\n\tif (args.length > 0) {\n\t\tthrow new Error("Expected to have exhausted test register arguments");\n\t}\n\n\treturn {\n\t\toptions: {\n\t\t\t...options,\n\t\t\tname,\n\t\t},\n\t\tcallback,\n\t};\n}\n\ntype TestRegisterFunctionArgs =\n\t| [TestName, TestCallback]\n\t| [TestName, NamelessTestOptions, TestCallback];\n\ntype TestRegisterFunction = (...args: TestRegisterFunctionArgs) => void;\n\nexport const test: TestRegisterFunction & {\n\tonly: TestRegisterFunction;\n} = function(...args: TestRegisterFunctionArgs) {\n\tconst {options, callback} = splitArgs(args);\n\tregisterTest(new Error(), options, callback);\n};\n\ntest.only = function(...args: TestRegisterFunctionArgs) {\n\tconst {options, callback} = splitArgs(args);\n\tregisterTest(\n\t\tnew Error(),\n\t\t{\n\t\t\t...options,\n\t\t\tonly: true,\n\t\t},\n\t\tcallback,\n\t);\n};\n',
			},
		],
		[
			"types.ts",
			{
				"mtime": 1_596_271_135_840.0615,
				"content": "/**\n * Copyright (c) Facebook, Inc. and its affiliates.\n *\n * This source code is licensed under the MIT license found in the\n * LICENSE file in the root directory of this source tree.\n */\n\n// These are copied from internal/codec-json/types.ts\nexport type JSONValue =\n\t| null\n\t| string\n\t| number\n\t| boolean\n\t| JSONObject\n\t| JSONArray;\n\nexport type JSONPropertyValue = undefined | void | JSONValue;\n\nexport type JSONObject = {\n\t[x: string]: JSONPropertyValue;\n};\n\nexport type JSONArray = Array<JSONValue>;\n\nexport type VoidCallback = () => void | undefined;\n\nexport type AsyncVoidCallback = () =>\n\t| void\n\t| undefined\n\t| Promise<void | undefined>;\n",
			},
		],
	]),
);
/* GENERATED:END(id:main) */
