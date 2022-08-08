import { ExtensionContext, Uri, window, workspace } from "vscode";
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind,
} from "vscode-languageclient/node";
import { setContextValue } from "./utils";
import { Session } from "./session";
import { syntaxTree } from "./commands/syntaxTree";
import { Commands } from "./commands";

let client: LanguageClient;

const IN_ROME_PROJECT = "inRomeProject";

export async function activate(context: ExtensionContext) {
	const command =
		process.env.DEBUG_SERVER_PATH || (await getServerPath(context));
	if (!command) {
		await window.showErrorMessage(
			"The Rome extensions doesn't ship with prebuilt binaries for your platform yet. " +
				"You can still use it by cloning the rome/tools repo from GitHub to build the LSP " +
				"yourself and use it with this extension with the rome.lspBin setting",
		);
		return;
	}

	const serverOptions: ServerOptions = {
		command,
		transport: TransportKind.stdio,
	};

	const traceOutputChannel = window.createOutputChannel("Rome Trace");

	// only override serverOptions.options when developing extension,
	// this is convenient for debugging
	// Before, every time we modify the client package, we need to rebuild vscode extension and install, for now, we could use Launching Client or press F5 to open a separate debug window and doing some check, finally we could bundle the vscode and do some final check.
	// Passing such variable via `Launch.json`, you need not to add an extra environment variable or change the setting.json `rome.lspBin`,
	if (process.env.DEBUG_SERVER_PATH) {
		serverOptions.options = { env: { ...process.env } };
	}

	const clientOptions: LanguageClientOptions = {
		documentSelector: [
			{ scheme: "file", language: "javascript" },
			{ scheme: "file", language: "typescript" },
			{ scheme: "file", language: "javascriptreact" },
			{ scheme: "file", language: "typescriptreact" },
		],
		traceOutputChannel,
	};

	client = new LanguageClient("rome_lsp", "Rome", serverOptions, clientOptions);

	const session = new Session(context, client);

	// we are now in a rome project
	setContextValue(IN_ROME_PROJECT, true);

	session.registerCommand(Commands.SyntaxTree, syntaxTree(session));

	client.start();
}

type Architecture = "x64" | "arm64";

type PlatformTriplets = {
	[P in NodeJS.Platform]?: {
		[A in Architecture]: string;
	};
};

const PLATFORM_TRIPLETS: PlatformTriplets = {
	win32: { x64: "x86_64-pc-windows-msvc", arm64: "aarch64-pc-windows-msvc" },
	darwin: { x64: "x86_64-apple-darwin", arm64: "aarch64-apple-darwin" },
	linux: {
		x64: "x86_64-unknown-linux-gnu",
		arm64: "aarch64-unknown-linux-gnu",
	},
};

async function getServerPath(
	context: ExtensionContext,
): Promise<string | undefined> {
	const config = workspace.getConfiguration();
	const explicitPath = config.get("rome.lspBin");
	if (typeof explicitPath === "string" && explicitPath !== "") {
		return explicitPath;
	}

	const triplet = PLATFORM_TRIPLETS[process.platform]?.[process.arch];
	if (!triplet) {
		return undefined;
	}

	const binaryExt = triplet.includes("windows") ? ".exe" : "";
	const binaryName = `rome_lsp${binaryExt}`;

	const bundlePath = Uri.joinPath(context.extensionUri, "server", binaryName);
	const bundleExists = await fileExists(bundlePath);

	return bundleExists ? bundlePath.fsPath : undefined;
}

async function fileExists(path: Uri) {
	try {
		await workspace.fs.stat(path);
		return true;
	} catch (err) {
		if (err.code === "ENOENT") {
			return false;
		} else {
			throw err;
		}
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
