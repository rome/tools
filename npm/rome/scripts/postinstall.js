const { platform, arch } = process;

const PLATFORMS = {
	win32: {
		x64: "@rometools/cli-win32-x64/rome.exe",
		arm64: "@rometools/cli-win32-arm64/rome.exe",
	},
	darwin: {
		x64: "@rometools/cli-darwin-x64/rome",
		arm64: "@rometools/cli-darwin-arm64/rome",
	},
	linux: {
		x64: "@rometools/cli-linux-x64/rome",
		arm64: "@rometools/cli-linux-arm64/rome",
	},
};

const binName = PLATFORMS?.[platform]?.[arch];
if (binName) {
	let binPath;
	try {
		binPath = require.resolve(binName);
	} catch {
		console.warn(
			`The Rome CLI postinstall script failed to resolve the binary file "${binName}". Running Rome from the npm package will probably not work correctly.`,
		);
	}
} else {
	console.warn(
		"The Rome CLI package doesn't ship with prebuilt binaries for your platform yet. " +
			"You can still use the CLI by cloning the rome/tools repo from GitHub, " +
			"and follow the instructions there to build the CLI for your platform.",
	);
}
