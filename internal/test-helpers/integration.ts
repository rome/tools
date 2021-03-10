import {TestHelper} from "rome";
import {
	Client,
	ClientFlags,
	FileReference,
	Server,
	ServerBridge,
	Worker,
	WorkerBridge,
} from "@internal/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	RelativePath,
	TEMP_PATH,
	UIDPath,
	createAbsoluteFilePath,
	createPath,
	createRelativePath,
	createUIDPath,
} from "@internal/path";
import {JSONObject, json} from "@internal/codec-config";
import {DEFAULT_TERMINAL_FEATURES, Stdout} from "@internal/cli-environment";
import {Dict} from "@internal/typescript-helpers";
import {DEFAULT_USER_CONFIG, UserConfig} from "../core/common/userConfig";
import ServerRequest from "../core/server/ServerRequest";
import {partialServerQueryRequestToFull} from "../core/server/Server";
import {PartialServerQueryRequest} from "../core/common/bridges/ServerBridge";
import {ProjectConfig, createDefaultProjectConfig} from "@internal/project";
import {Fixture, FixtureFile, createFixtureTests} from "@internal/test-helpers";
import {removeCarriageReturn} from "@internal/string-utils";
import {
	getFileHandlerExtensions,
	getFileHandlerFromPathAssert,
} from "../core/common/file-handlers";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";
import crypto = require("crypto");
import stream = require("stream");
import {ExtensionHandler} from "../core/common/file-handlers/types";
import {
	DiagnosticsProcessor,
	interceptDiagnostics,
} from "@internal/diagnostics";
import {
	convertToMarkupFromRandomString,
	joinMarkupLines,
	normalizeMarkup,
} from "@internal/markup";
import {markupToPlainText} from "@internal/cli-layout";
import child = require("child_process");
import util = require("util");
import {Reporter} from "@internal/cli-reporter";
import {BridgeClient} from "@internal/events";
import {decodeUTF8} from "@internal/binary";

const exec = util.promisify(child.exec);

type IntegrationTestHelper = {
	cwd: AbsoluteFilePath;
	bridge: BridgeClient<typeof ServerBridge>;
	client: Client;
	server: Server;
	readFile: (relative: RelativePath | string) => Promise<string>;
	writeFile: (relative: RelativePath | string, content: string) => Promise<void>;
	createRequest: (query?: PartialServerQueryRequest) => Promise<ServerRequest>;
};

type IntegrationTestOptions = {
	gitInitialize?: boolean;
	disableProjectConfig?: boolean;
	userConfig?: UserConfig;
	files?: Dict<string>;
	projectConfig?: JSONObject;
	flags?: Partial<ClientFlags>;
};

export async function generateTempDirectory(
	prefix: string = "rome",
): Promise<AbsoluteFilePath> {
	const key = crypto.randomBytes(16).toString("base64");
	const path = TEMP_PATH.append(`${prefix}-${key}`);
	if (await path.exists()) {
		// Extremely rare collision which is only possible if we haven't cleaned up
		return generateTempDirectory(prefix);
	} else {
		await path.createDirectory();
		return path;
	}
}

export type IntegrationWorker = {
	worker: Worker;
	addProject: (config: ProjectConfig) => number;
	performFileOperation: <T>(
		opts: IntegrationWorkerFileRefOptions,
		callback: (ref: FileReference) => Promise<T>,
	) => Promise<T>;
};

type IntegrationWorkerFileRefOptions = {
	sourceText?: string;
	real?: string | AbsoluteFilePath;
	project?: number;
	once?: boolean;
	uid: string | UIDPath;
};

export function findFixtureInput(
	{files, dir}: Fixture,
	projectConfig: undefined | ProjectConfig,
): {
	input: FixtureFile;
	handler: ExtensionHandler;
} {
	const triedExts = [];
	for (const ext of getFileHandlerExtensions(projectConfig)) {
		const input = files.get(`input.${ext}`);
		if (input !== undefined) {
			return {
				input,
				handler: getFileHandlerFromPathAssert(
					createPath(`input.${ext}`),
					projectConfig,
				).handler,
			};
		}

		triedExts.push(ext);
	}

	throw new Error(
		`The fixture ${dir} did not have an input.(${triedExts.join("|")})`,
	);
}

let cachedIntegrationWorker: undefined | IntegrationWorker;

export function createMockWorker(force: boolean = false): IntegrationWorker {
	if (!force && cachedIntegrationWorker !== undefined) {
		return cachedIntegrationWorker;
	}

	// This wont actually be used, it's just for setting up subscriptions
	const bridges = WorkerBridge.createFromLocal({optionalResource: true});

	const worker = new Worker({
		type: "file-processor",
		id: 0,
		env: {},
		dedicated: false,
		userConfig: DEFAULT_USER_CONFIG,
		cacheWriteDisabled: true,
		cacheReadDisabled: true,
		inspectorPort: undefined,
		bridge: bridges.client,
	});

	let projectIdCounter = 0;

	async function performFileOperation<T>(
		opts: IntegrationWorkerFileRefOptions,
		callback: (ref: FileReference) => Promise<T>,
	): Promise<T> {
		const {
			project = defaultProjectId,
			sourceText,
		} = opts;

		let relative = createRelativePath(
			typeof opts.uid === "string" ? opts.uid : opts.uid.format(),
		);

		if (opts.real === undefined && opts.sourceText === undefined) {
			throw new Error("real and sourceText cannot be undefined");
		}

		let real: AbsoluteFilePath;
		if (opts.real === undefined) {
			real = createAbsoluteFilePath(`/project-${project}`).append(relative);
		} else if (typeof opts.real === "string") {
			real = createAbsoluteFilePath(opts.real);
		} else {
			real = opts.real;
		}

		const uid =
			typeof opts.uid === "string" ? createUIDPath(opts.uid) : opts.uid;

		const ref: FileReference = {
			project,
			uid,
			real,
		};

		if (sourceText !== undefined) {
			worker.updateBuffer(
				ref,
				{
					mtimeNs: BigInt(Date.now()) * 1000000n,
					content: sourceText,
				},
			);
		}

		try {
			return await interceptDiagnostics(
				async () => {
					return await callback(ref);
				},
				(processor) => {
					if (sourceText !== undefined) {
						processor.normalizer.setInlineSourceText(ref.uid, sourceText);
					}
				},
			);
		} finally {
			worker.clearBuffer(ref);
		}
	}

	function addProject(config: ProjectConfig): number {
		let id = projectIdCounter++;
		worker.updateProjects(
			new Map([
				[
					id,
					{
						config,
						configCacheKeys: {},
						configPath: createAbsoluteFilePath(`/project-${id}/package.json`),
						directory: createAbsoluteFilePath(`/project-${id}`),
					},
				],
			]),
		);
		return id;
	}

	const defaultProjectId = addProject(createDefaultProjectConfig());

	const int: IntegrationWorker = {
		addProject,
		performFileOperation,
		worker,
	};
	if (!force) {
		cachedIntegrationWorker = int;
	}
	return int;
}

export async function declareParserTests() {
	const {worker, performFileOperation} = createMockWorker();

	return createFixtureTests(async (fixture, t) => {
		const {options} = fixture;
		const {input} = findFixtureInput(fixture, undefined);

		const sourceTypeJS = options.get("sourceTypeJS").asStringSetOrVoid([
			"script",
			"module",
		]);
		const inputContent = removeCarriageReturn(decodeUTF8(input.content));

		const {ast} = await performFileOperation(
			{
				uid: input.relative.join(),
				sourceText: inputContent,
			},
			async (ref) => {
				return await worker.parse(
					ref,
					{
						cache: false,
						sourceTypeJS,
						allowCorrupt: true,
					},
				);
			},
		);

		// Inline diagnostics
		const processor = new DiagnosticsProcessor();
		processor.normalizer.setInlineSourceText(ast.path, inputContent);
		processor.addDiagnostics(ast.diagnostics);
		const diagnostics = processor.getDiagnostics();

		const outputFile = input.absolute.getParent().append(
			input.absolute.getExtensionlessBasename(),
		).join();
		t.namedSnapshot("ast", ast, undefined, {filename: outputFile});

		const printedDiagnostics = await printDiagnosticsToString({
			diagnostics,
			suppressions: [],
		});
		t.namedSnapshot(
			"diagnostics",
			printedDiagnostics,
			undefined,
			{filename: outputFile},
		);

		if (diagnostics.length === 0) {
			if (options.has("throws")) {
				// TODO: throw new Error(`Expected diagnostics but didn't receive any\n${printedDiagnostics}`);
			}
		} else if (!options.has("throws")) {
			// TODO: throw new Error(`Received diagnostics when we didn't expect any\n${printedDiagnostics}`);
		}
	});
}

export function createIntegrationTest(
	opts: IntegrationTestOptions,
	callback: (t: TestHelper, helper: IntegrationTestHelper) => Promise<void>,
): (t: TestHelper) => Promise<void> {
	return async function(t: TestHelper) {
		return;
		t.setTimeout(10_000);

		const temp = await generateTempDirectory("rome-integration");

		const projectPath = temp.append("project");
		await projectPath.createDirectory();

		if (opts.gitInitialize) {
			await exec("git init", {cwd: projectPath.join()});
		}

		const virtualModulesPath = temp.append("virtual");
		await virtualModulesPath.createDirectory();

		const cachePath = temp.append("cache");
		await cachePath.createDirectory();

		const remotePath = temp.append("remote");
		await remotePath.createDirectory();

		const recoveryPath = temp.append("recovery");
		await recoveryPath.createDirectory();

		const userConfig: UserConfig = {
			configPath: undefined,
			recoveryPath,
			cacheDirectory: cachePath,
			syntaxTheme: undefined,
		};

		try {
			const {flags, projectConfig = {}, files = {}} = opts;

			// Properly configure `vendorPath` so it doesn't point to /tmp
			projectConfig.files = Object.assign(
				{},
				projectConfig.files,
				{
					vendorPath: "../remote",
				},
			);

			// Add serialized project config. We skip this if there's already a project config files entry to allow
			// some flexibility if we want invalid project config tests.
			if (
				!opts.disableProjectConfig &&
				files[".config/rome.json"] === undefined
			) {
				files[".config/rome.json"] = json.stringify(projectConfig) + "\n";
			}

			// Materialize files
			for (let basename in files) {
				const path = projectPath.append(basename);
				await path.getParent().createDirectory();

				const content = files[basename];
				await path.writeFile(content);
			}

			// Use this reporter for markup rendering
			const reporter = new Reporter("IntegrationTest");
			const clientStream = reporter.attachCaptureStream();

			// Mock and capture stdout
			const stdout: Stdout = new stream.Writable({
				write(chunk, encoding, callback) {
					const str = chunk.toString();
					const markup = convertToMarkupFromRandomString(str);

					// Strip filelink text that could have absolute paths
					const stripped = normalizeMarkup(
						markup,
						{
							stripFilelinkText: true,
							cwd: temp,
						},
					);

					reporter.log(stripped.text, {noNewline: true});
					callback();
				},
			});

			// Create a Client. The abstraction used by the CLI.
			const client = new Client({
				dedicated: false,
				userConfig,
				terminalFeatures: {
					...DEFAULT_TERMINAL_FEATURES,
					format: "markup",
				},
				flags: {
					realCwd: projectPath,
					cwd: projectPath,
					...flags,
				},
				stdin: process.stdin,
				stdout,
				stderr: stdout,
			});

			// Capture client logs
			let logs = "";
			await client.subscribeLogs(
				"all",
				true,
				(chunk) => {
					const textChunk = joinMarkupLines(
						markupToPlainText(convertToMarkupFromRandomString(chunk)),
					);
					logs += textChunk;
				},
			);

			t.addToAdvice({
				type: "log",
				category: "info",
				text: "Server logs",
			});

			t.addToAdvice(() => ({
				type: "code",
				sourceText: logs,
			}));

			try {
				// Start the server inside of the process
				const {server, bridge, serverClient} = await client.startInternalServer({
					// Only one worker running inside of this process. Don't fork workers.
					inbandOnly: true,
					// Force cache to be enabled (which will be at our generated directory specified above)
					// This will ignore any ROME_CACHE env variable specified by scripts/dev-rome
					forceCacheEnabled: true,
					userConfig,
				});

				const intTestHelper: IntegrationTestHelper = {
					cwd: projectPath,
					bridge,
					client,
					server,
					async readFile(relative: RelativePath | string): Promise<string> {
						const absolute = projectPath.append(relative);
						return absolute.readFileText();
					},
					async writeFile(
						relative: RelativePath | string,
						content: string,
					): Promise<void> {
						const absolute = projectPath.append(relative);
						await server.recoveryStore.writeFiles(
							new AbsoluteFilePathMap([
								[
									absolute,
									{
										type: "WRITE",
										content,
										mtimeNs: server.memoryFs.maybeGetMtimeNs(absolute),
									},
								],
							]),
						);
					},
					async createRequest(
						query: PartialServerQueryRequest = {commandName: "noop"},
					) {
						return new ServerRequest({
							client: serverClient,
							query: partialServerQueryRequestToFull(query),
							server,
						});
					},
				};

				await callback(t, intTestHelper);
			} finally {
				await client.end();

				// Console
				t.namedSnapshot("console", clientStream.read());

				// Files
				const files: string[] = [];
				let queue: AbsoluteFilePath[] = [projectPath];
				while (queue.length > 0) {
					const path = queue.pop()!;
					const stat = await path.lstat();

					if (stat.isDirectory()) {
						if (path.getBasename() === ".git") {
							// Don't output the entire .git directory
							queue = [...queue, path.append("HEAD")];
						} else {
							queue = [...queue, ...(await path.readDirectory())];
						}
					} else {
						files.push(projectPath.relative(path).join());
					}
				}

				let filesSnapshot = "";
				for (const basename of files.sort()) {
					if (filesSnapshot !== "") {
						filesSnapshot += "\n";
					}

					filesSnapshot += `# ${basename}\n`;
					filesSnapshot += await projectPath.append(basename).readFileText();
					filesSnapshot += "\n";
				}

				t.namedSnapshot("files", filesSnapshot);
			}
		} finally {
			// Clean up after ourselves. Will be called whether the tests fails or is successful
			await temp.removeDirectory();
		}
	};
}
