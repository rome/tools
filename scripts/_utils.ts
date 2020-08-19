import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@internal/path";
import {
	lstat,
	readDirectory,
	readFileText,
	writeFile as writeFileReal,
} from "@internal/fs";
import {Reporter} from "@internal/cli-reporter";
import {createMockWorker} from "@internal/test-helpers";

import crypto = require("crypto");
import child = require("child_process");
import {markup} from "@internal/markup";
import {regex} from "@internal/string-escape";

export const reporter = Reporter.fromProcess();
export const integrationWorker = createMockWorker();

export const ROOT = findRoot();
export const INTERNAL = ROOT.append("internal");
export const PUBLIC_PACKAGES = ROOT.append("public-packages");

// This is only necessary because we have poor support for __dirname in the bundler
function findRoot(): AbsoluteFilePath {
	let pickNext = false;

	for (const path of createAbsoluteFilePath(__dirname).getChain()) {
		if (pickNext) {
			return path;
		}

		if (path.getBasename() === "scripts") {
			pickNext = true;
		}
	}

	throw new Error("Could not find the root");
}

let forceGenerated = false;

const COMMENT_START = /(?:\/\*|<!--)/;
const COMMENT_END = /(?:\*\/|-->)/;

export async function modifyGeneratedFile(
	{path, scriptName, id = "main"}: {
		path: AbsoluteFilePath;
		scriptName: string;
		id?: string;
	},
	callback: () => Promise<{
		lines: Array<string>;
		hash?: string;
	}>,
): Promise<void> {
	const {lines, hash: customHashContent} = await callback();

	// Build expected inner generated
	let generated = await formatFile(
		path,
		lines.map((line) => line.trimRight()).join("\n"),
	);
	generated = generated.trim();

	// Read file
	let file = await readFileText(path);

	const startRegex = regex`${COMMENT_START} GENERATED:START\(hash:(.*?),id:${id}\) ${createGeneratedCommentInstructions(
		scriptName,
	)} ${COMMENT_END}(\n|\r\n)`;
	const startMatch = file.match(startRegex);
	const startIndex = startMatch?.index ?? file.length;
	const startInnerIndex = startIndex + (startMatch ? startMatch[0].length : 0);

	const endRegex = regex`${COMMENT_START} GENERATED:END\(id:${id}\) ${COMMENT_END}(\n|\r\n)`;
	const endMatch = file.match(endRegex);
	const endInnerIndex = endMatch?.index ?? file.length;
	const endIndex = endInnerIndex + (endMatch ? endMatch[0].length : 0);

	const generatedInner = file.slice(startInnerIndex, endInnerIndex);
	let contentStart = file.slice(0, startIndex);
	let contentEnd = file.slice(endIndex, file.length);

	// Check if the generated file has the same hash
	const commentHash = startMatch ? startMatch[1] : "";
	const expectedHash = hash(customHashContent || generated);
	const generatedHash = hash(generatedInner);
	let isSame = false;
	if (expectedHash === commentHash && generatedHash === commentHash) {
		isSame = true;
	}
	if (customHashContent && expectedHash === commentHash) {
		isSame = true;
	}
	if (forceGenerated) {
		isSame = false;
	}

	// The file is up to date if the comment hash and hash of the inner generated matches
	if (isSame) {
		reporter.warn(
			markup`Generated <emphasis>${path}</emphasis><dim>(hash:${expectedHash},id:${id})</dim> is the same.`,
		);
		return;
	}

	// Append comments
	const commentOpts: CommentOptions = {
		isJS: path.hasExtension("ts") || path.hasExtension("js"),
		hash: expectedHash,
		scriptName,
		id,
	};

	let final = contentStart.trimRight();
	if (final !== "") {
		final += "\n\n";
	}
	final += createGeneratedStartComment(commentOpts) + "\n";
	final += generated + "\n";
	final += createGeneratedEndComment(commentOpts);
	final += "\n\n";
	final += contentEnd.trimLeft();
	final = final.trimRight() + "\n";
	await writeFile(path, final);
}

export function setForceGenerated(force: boolean) {
	forceGenerated = force;
}

type CommentOptions = {
	scriptName: string;
	id: string;
	hash: string;
	isJS: boolean;
};

function createGeneratedCommentInstructions(scriptName: string): string {
	return `Everything below is automatically generated. DO NOT MODIFY. Run \`./rome run scripts/${scriptName}\` to update.`;
}

function createGeneratedStartComment(opts: CommentOptions): string {
	const {hash, id, isJS, scriptName} = opts;
	return wrapComment(
		`GENERATED:START(hash:${hash},id:${id}) ${createGeneratedCommentInstructions(
			scriptName,
		)}`,
		isJS,
	);
}

function createGeneratedEndComment({id, isJS}: CommentOptions): string {
	return wrapComment(`GENERATED:END(id:${id})`, isJS);
}

function wrapComment(value: string, isJS: boolean): string {
	let comment = "";
	if (isJS) {
		comment += "/* ";
	} else {
		comment += "<!-- ";
	}
	comment += value;
	if (isJS) {
		comment += " */";
	} else {
		comment += " -->";
	}
	return comment;
}

function hash(content: string): string {
	return crypto.createHash("sha1").update(content).digest("hex");
}

async function formatFile(
	path: AbsoluteFilePath,
	sourceText: string,
): Promise<string> {
	// Not currently supported
	if (path.hasExtension("md")) {
		return sourceText;
	}

	return await integrationWorker.performFileOperation(
		{
			real: path,
			uid: ROOT.relative(path).join(),
			sourceText,
		},
		async (ref) => {
			const res = await integrationWorker.worker.api.format(ref, {}, {});
			if (res === undefined) {
				return sourceText;
			} else {
				return res.formatted;
			}
		},
	);
}

export async function writeFile(path: AbsoluteFilePath, sourceText: string) {
	sourceText = await formatFile(path, sourceText);

	// Windows: `content` will always have `\r` stripped so add it back
	if (process.platform === "win32") {
		sourceText = sourceText.replace(/\n/g, "\r\n");
	}

	// Write
	await writeFileReal(path, sourceText);
	reporter.success(markup`Wrote <emphasis>${path}</emphasis>`);
}

export function waitChildProcess(
	proc: child.ChildProcess,
): Promise<child.ChildProcess> {
	return new Promise((resolve) => {
		proc.on(
			"close",
			() => {
				resolve(proc);
			},
		);
	});
}

export async function exec(
	cmd: string,
	args: Array<string>,
	opts: child.SpawnOptions = {},
): Promise<void> {
	reporter.command(`${cmd} ${args.join(" ")}`);

	const proc = await waitChildProcess(
		child.spawn(
			cmd,
			args,
			{
				stdio: "inherit",
				...opts,
			},
		),
	);

	if (proc.exitCode !== 0) {
		reporter.error(markup`Exit code ${String(proc.exitCode)}`);
		process.exit(proc.exitCode || 0);
	}
}

export async function execDev(args: Array<string>): Promise<void> {
	await waitChildProcess(
		child.spawn(
			process.execPath,
			[ROOT.append("scripts/dev-rome.cjs").join(), ...args],
			{
				stdio: "inherit",
			},
		),
	);
}

async function getSubDirectories(
	files: AbsoluteFilePathSet,
): Promise<Array<string>> {
	const subDirs: Array<string> = [];

	for await (const file of files) {
		if ((await lstat(file)).isDirectory()) {
			subDirs.push(file.getBasename());
		}
	}

	return subDirs;
}

export async function getLanguages(): Promise<Array<string>> {
	const astPath = INTERNAL.append("ast");
	const astDir = await readDirectory(astPath);

	return getSubDirectories(astDir);
}

export async function getLanguageCategories(
	language: string,
): Promise<Array<string>> {
	const languagePath = INTERNAL.append("ast", language);
	const languageDir = await readDirectory(languagePath);

	return getSubDirectories(languageDir);
}

export async function languageExists(language: string): Promise<boolean> {
	const languages = await getLanguages();

	return languages.includes(language);
}

export async function languageCategoryExists(
	language: string,
	category: string,
): Promise<boolean> {
	const categories = await getLanguageCategories(language);

	return categories.includes(category);
}
