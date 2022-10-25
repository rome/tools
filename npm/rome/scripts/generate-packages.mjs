import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import * as fs from "node:fs";

const ROMECLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_ROOT = resolve(ROMECLI_ROOT, "..");
const REPO_ROOT = resolve(PACKAGES_ROOT, "..");
const MANIFEST_PATH = resolve(ROMECLI_ROOT, "package.json");

const rootManifest = JSON.parse(
	fs.readFileSync(MANIFEST_PATH).toString("utf-8"),
);

function generateNativePackage(platform, arch) {
	const packageName = `@rometools/cli-${platform}-${arch}`;
	const packageRoot = resolve(PACKAGES_ROOT, `cli-${platform}-${arch}`);

	// Remove the directory just in case it already exists (it's autogenerated
	// so there shouldn't be anything important there anyway)
	fs.rmSync(packageRoot, { recursive: true, force: true });

	// Create the package directory
	console.log(`Create directory ${packageRoot}`);
	fs.mkdirSync(packageRoot);

	// Generate the package.json manifest
	const { version } = rootManifest;

	const manifest = JSON.stringify({
		name: packageName,
		version,
		os: [platform],
		cpu: [arch],
	});

	const manifestPath = resolve(packageRoot, "package.json");
	console.log(`Create manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, manifest);

	// Copy the CLI binary
	const ext = platform === "win32" ? ".exe" : "";
	const binarySource = resolve(REPO_ROOT, `rome-${platform}-${arch}${ext}`);
	const binaryTarget = resolve(packageRoot, `rome${ext}`);

	console.log(`Copy binary ${binaryTarget}`);
	fs.copyFileSync(binarySource, binaryTarget);
}

function updateWasmPackage(target) {
	const packageName = `@rometools/wasm-${target}`;
	const packageRoot = resolve(PACKAGES_ROOT, `wasm-${target}`);

	const manifestPath = resolve(packageRoot, "package.json");
	const manifest = JSON.parse(fs.readFileSync(manifestPath).toString("utf-8"));

	const { version } = rootManifest;
	manifest["name"] = packageName;
	manifest["version"] = version;

	console.log(`Update manifest ${manifestPath}`);
	fs.writeFileSync(manifestPath, JSON.stringify(manifest));
}

function writeManifest(packagePath) {
	const manifestPath = resolve(PACKAGES_ROOT, packagePath, "package.json");

	const manifestData = JSON.parse(
		fs.readFileSync(manifestPath).toString("utf-8"),
	);

	const nativePackages = PLATFORMS.flatMap((platform) =>
		ARCHITECTURES.map((arch) => [
			`@rometools/cli-${platform}-${arch}`,
			rootManifest.version,
		]),
	);

	manifestData["version"] = rootManifest.version;
	manifestData["optionalDependencies"] = Object.fromEntries(
		nativePackages,
	);

	console.log(`Update manifest ${manifestPath}`);
	const content = JSON.stringify(manifestData);
	fs.writeFileSync(manifestPath, content);
}

const PLATFORMS = ["win32", "darwin", "linux"];
const ARCHITECTURES = ["x64", "arm64"];
const WASM_TARGETS = ["bundler", "nodejs", "web"];

for (const target of WASM_TARGETS) {
	updateWasmPackage(target);
}

for (const platform of PLATFORMS) {
	for (const arch of ARCHITECTURES) {
		generateNativePackage(platform, arch);
	}
}

writeManifest("rome");
writeManifest("backend-jsonrpc");
