import { ExtensionContext, workspace, Uri, window } from 'vscode';

import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	const command = await getServerPath(context);
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

	const clientOptions: LanguageClientOptions = {
		documentSelector: [
			{ scheme: "file", language: "javascript" },
			{ scheme: "file", language: "typescript" },
		],
	};

	client = new LanguageClient("rome_lsp", "Rome", serverOptions, clientOptions);

	client.start();
}

type Architecture = 'x64' | 'arm64';

type PlatformTriplets = {
	[P in NodeJS.Platform]?: {
		[A in Architecture]: string;
	};
};

const PLATFORM_TRIPLETS: PlatformTriplets = {
	win32: { x64: "x86_64-pc-windows-msvc", arm64: "aarch64-pc-windows-msvc" },
	darwin: { x64: "x86_64-apple-darwin", arm64: "aarch64-apple-darwin" },
	linux: { x64: "x86_64-unknown-linux-gnu", arm64: "aarch64-unknown-linux-gnu" },
};

async function getServerPath(context: ExtensionContext): Promise<
	string | undefined
> {
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
