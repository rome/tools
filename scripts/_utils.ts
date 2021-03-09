import {AbsoluteFilePath, createAbsoluteFilePath} from "@internal/path";
import {Reporter} from "@internal/cli-reporter";
import {createMockWorker} from "@internal/test-helpers";
import {formatAST} from "@internal/formatter";
import {valueToNode} from "@internal/js-ast-utils";
import {markup} from "@internal/markup";
import {regex} from "@internal/string-escape";
import {json} from "@internal/codec-config";
import crypto = require("crypto");
import child = require("child_process");

export const reporter = Reporter.fromProcess();
export const integrationWorker = createMockWorker();

export const ROOT = createAbsoluteFilePath(__dirname).getParent();
export const INTERNAL = ROOT.append("internal");
export const PUBLIC_PACKAGES = ROOT.append("public-packages");

let forceGenerated = false;

const COMMENT_START = /(?:\/\*|<!--|#)/;
const COMMENT_END = /(?:\*\/|-->|#)/;

export async function modifyGeneratedFile(
	{path, scriptName, id = "main"}: {
		path: AbsoluteFilePath;
		scriptName?: string;
		id?: string;
	},
	callback: () => Promise<{
		lines: string[];
		prepend?: boolean;
		hash?: string;
	}>,
): Promise<void> {
	const {prepend, lines, hash: customHashContent} = await callback();

	// Build expected inner generated
	let generated = await formatFile(
		path,
		lines.map((line) => line.trimRight()).join("\n"),
	);
	generated = generated.trim();

	// Read file
	let file = await path.readFileText();

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

	const existingGeneratedInner = file.slice(startInnerIndex, endInnerIndex);
	let contentStart = file.slice(0, startIndex);
	let contentEnd = file.slice(endIndex, file.length);

	if (prepend) {
		generated += existingGeneratedInner;
	}

	// Check if the generated file has the same hash
	const commentHash = startMatch ? startMatch[1] : "";
	const expectedHash = hash(customHashContent || generated);
	const generatedHash = hash(existingGeneratedInner);
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
		delimiter: determineDelimiter(path),
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
	scriptName: undefined | string;
	id: string;
	hash: string;
	delimiter: CommentDelimiter;
};

type CommentDelimiter = "SLASH" | "ARROW" | "HASH";

function determineDelimiter(path: AbsoluteFilePath): CommentDelimiter {
	if (
		path.hasExtension("ts") ||
		path.hasExtension("js") ||
		path.hasExtension("rjson")
	) {
		return "SLASH";
	}
	if (
		path.hasExtension("sh") ||
		path.hasExtension("yml") ||
		path.hasExtension("yaml") ||
		path.hasExtension("toml")
	) {
		return "HASH";
	}
	return "ARROW";
}

function createGeneratedCommentInstructions(scriptName: undefined | string): string {
	let instructions = `Everything below is automatically generated. DO NOT MODIFY.`;
	if (scriptName !== undefined) {
		instructions += `Run \`./rome run scripts/${scriptName}\` to update.`;
	}
	return instructions;
}

function createGeneratedStartComment(opts: CommentOptions): string {
	const {hash, id, delimiter, scriptName} = opts;
	return wrapComment(
		`GENERATED:START(hash:${hash},id:${id}) ${createGeneratedCommentInstructions(
			scriptName,
		)}`,
		delimiter,
	);
}

function createGeneratedEndComment({id, delimiter}: CommentOptions): string {
	return wrapComment(`GENERATED:END(id:${id})`, delimiter);
}

function wrapComment(value: string, delimiter: CommentDelimiter): string {
	let comment = "";

	switch (delimiter) {
		case "SLASH": {
			comment += "/* ";
			break;
		}
		case "ARROW": {
			comment += "<!-- ";
			break;
		}
		case "HASH": {
			comment += "# ";
			break;
		}
	}
	comment += value;

	switch (delimiter) {
		case "SLASH": {
			comment += " */";
			break;
		}
		case "ARROW": {
			comment += " -->";
			break;
		}
		case "HASH": {
			comment += " #";
			break;
		}
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
	await path.writeFile(sourceText);
	reporter.success(markup`Wrote <emphasis>${path}</emphasis>`);
}

export function waitChildProcess(
	proc: child.ChildProcess,
): Promise<child.ChildProcess> {
	return new Promise((resolve) => {
		proc.on(
			"close",
			(code) => {
				if (code === 0) {
					resolve(proc);
				} else {
					reporter.error(
						markup`Subprocess exit with code ${String(proc.exitCode)}`,
					);
					process.exit(proc.exitCode || 0);
				}
			},
		);
	});
}

export async function exec(
	cmd: string,
	args: string[],
	opts: child.SpawnOptions = {},
): Promise<void> {
	reporter.command(`${cmd} ${args.join(" ")}`);

	await waitChildProcess(
		child.spawn(
			cmd,
			args,
			{
				stdio: "inherit",
				...opts,
			},
		),
	);
}

export async function execDev(args: string[]): Promise<void> {
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

export async function updateVersion(newVersion: string): Promise<void> {
	const path = ROOT.append("package.json");

	const manifest = json.consumeValue(await path.readFileTextMeta());
	manifest.set("version", newVersion);

	const formatted = json.stringify(manifest.asUnknown()) + "\n";
	await path.writeFile(formatted);


	reporter.success(
		markup`Updated <code>version</code> to <emphasis>${newVersion}</emphasis> in ${path}`,
	);
}

async function getSubDirectories(
	files: Iterable<AbsoluteFilePath>,
): Promise<string[]> {
	const subDirs: string[] = [];

	for await (const file of files) {
		if ((await file.lstat()).isDirectory()) {
			subDirs.push(file.getBasename());
		}
	}

	return subDirs;
}

export async function getLanguages(): Promise<string[]> {
	const astPath = INTERNAL.append("ast");
	const astDir = await astPath.readDirectory();

	return getSubDirectories(astDir);
}

export async function getLanguageCategories(language: string): Promise<string[]> {
	const languagePath = INTERNAL.append("ast", language);
	const languageDir = await languagePath.readDirectory();

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

export function valueToCode(value: unknown): string {
	return formatAST(valueToNode(value)).code;
}
