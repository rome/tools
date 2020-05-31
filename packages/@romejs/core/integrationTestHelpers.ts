import {TestHelper} from "rome";
import {Client, Master, MasterBridge} from ".";
import {AbsoluteFilePath, RelativeFilePath, TEMP_PATH} from "@romejs/path";
import {ClientFlags} from "./common/types/client";
import {JSONObject, stringifyJSON} from "@romejs/codec-json";
import {
	createDirectory,
	exists,
	lstat,
	readDirectory,
	readFileText,
	removeDirectory,
	writeFile,
} from "@romejs/fs";
import {Dict} from "@romejs/typescript-helpers";
import {UserConfig} from "./common/userConfig";
import crypto = require("crypto");
import stream = require("stream");

type IntegrationTestHelper = {
	cwd: AbsoluteFilePath;
	bridge: MasterBridge;
	client: Client;
	master: Master;
	writeFile: (
		relative: RelativeFilePath | string,
		content: string,
	) => Promise<void>;
};

type IntegrationTestOptions = {
	userConfig?: UserConfig;
	files?: Dict<string>;
	projectConfig?: JSONObject;
	flags?: Partial<ClientFlags>;
};

async function generateTempFolder(): Promise<AbsoluteFilePath> {
	const key = crypto.randomBytes(16).toString("base64");
	const path = TEMP_PATH.append(`rome-integration-${key}`);
	if (await exists(path)) {
		// Extremely rare collision which is only possible if we haven't cleaned up
		return generateTempFolder();
	} else {
		await createDirectory(path);
		return path;
	}
}

export function createIntegrationTest(
	opts: IntegrationTestOptions,
	callback: (t: TestHelper, helper: IntegrationTestHelper) => Promise<void>,
): (t: TestHelper) => Promise<void> {
	return async function(t: TestHelper) {
		const temp = await generateTempFolder();

		const projectPath = temp.append("project");
		await createDirectory(projectPath);

		const virtualModulesPath = temp.append("virtual");
		await createDirectory(virtualModulesPath);

		const cachePath = temp.append("cache");
		await createDirectory(cachePath);

		const remotePath = temp.append("remote");
		await createDirectory(remotePath);

		const userConfig: UserConfig = {
			cachePath,
			runtimeModulesPath: virtualModulesPath,
		};

		try {
			const {flags, projectConfig = {}, files = {}} = opts;

			projectConfig.files = Object.assign({}, project.files, {
				vendorPath: "../remote",
			});

			if (files["rome.json"] === undefined && files["rome.rjson"] === undefined) {
				files["rome.json"] = stringifyJSON(projectConfig);
			}

			for (let basename in files) {
				const path = projectPath.append(basename);
				await createDirectory(path.getParent());

				const content = files[basename];
				await writeFile(path, content);
			}

			let console = "";
			const stdout = new stream.Writable({
				write(chunk, encoding, callback) {
					console += chunk;
					callback();
				},
			});

			const client = new Client({
				globalErrorHandlers: false,
				flags: {
					cwd: projectPath,
					...flags,
				},
				stdin: undefined,
				stdout,
				stderr: stdout,
			});

			let logs = "";
			client.bridgeAttachedEvent.subscribe(async () => {
				await client.subscribeLogs(
					true,
					(chunk) => {
						logs += chunk;
					},
				);
			});

			try {
				const {master, bridge} = await client.startInternalMaster({
					inbandOnly: true,
					forceCacheEnabled: true,
					loggerOptions: {
						cwd: projectPath,
						excludePid: true,
					},
					userConfig,
				});

				await callback(
					t,
					{
						cwd: projectPath,
						bridge,
						client,
						master,
						async writeFile(
							relative: RelativeFilePath | string,
							content: string,
						): Promise<void> {
							const absolute = projectPath.append(relative);
							await writeFile(absolute, content);
						},
					},
				);
			} finally {
				await client.end();

				// Console
				t.namedSnapshot("console", console);

				// Logs
				//t.namedSnapshot("logs", logs);
				logs;

				// Files
				const files: Array<string> = [];
				let queue: Array<AbsoluteFilePath> = [projectPath];
				while (queue.length > 0) {
					const path = queue.pop()!;
					const stat = await lstat(path);

					if (stat.isDirectory()) {
						queue = [...queue, ...(await readDirectory(path))];
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
					filesSnapshot += (await readFileText(projectPath.append(basename))).trim();
					filesSnapshot += "\n";
				}

				t.namedSnapshot("files", filesSnapshot);
			}
		} finally {
			await removeDirectory(temp);
		}
	};
}
