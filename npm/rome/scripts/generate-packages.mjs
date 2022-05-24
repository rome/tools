import { resolve } from "path";
import { fileURLToPath } from "url";
import * as fs from "fs";

const ROMECLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const PACKAGES_ROOT = resolve(ROMECLI_ROOT, "..");
const REPO_ROOT = resolve(PACKAGES_ROOT, "..");
const MANIFEST_PATH = resolve(ROMECLI_ROOT, "package.json");

const rootManifest = JSON.parse(
	fs.readFileSync(MANIFEST_PATH).toString("utf-8"),
);

function generatePackage(platform, arch) {
	const packageName = `@rometools/cli-${platform}-${arch}`;
	const packageRoot = resolve(PACKAGES_ROOT, `cli-${platform}-${arch}`);

	// Remove the directory just in case it already exists (it's autogenerated
	// so there shouldn't be anything important there anyway)
	fs.rmSync(packageRoot, { recursive: true, force: true });

	// Create the package directory
	console.log("Create directory " + packageRoot);
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
	console.log("Create manifest " + manifestPath);
	fs.writeFileSync(manifestPath, manifest);

	// Copy the CLI binary
	const ext = platform === "win32" ? ".exe" : "";
	const binarySource = resolve(REPO_ROOT, `rome-${platform}-${arch}${ext}`);
	const binaryTarget = resolve(packageRoot, `rome${ext}`);

	console.log("Copy binary " + binaryTarget);
	fs.copyFileSync(binarySource, binaryTarget);
}

function writeManifest() {
	rootManifest["optionalDependencies"] =
		Object.fromEntries(
			PLATFORMS.flatMap(
				(platform) =>
					ARCHITECTURES.map(
						(arch) => [
							`@rometools/cli-${platform}-${arch}`,
							rootManifest.version,
						],
					),
			),
		);

	console.log("Update manifest " + MANIFEST_PATH);
	const content = JSON.stringify(rootManifest);
	fs.writeFileSync(MANIFEST_PATH, content);
}

const PLATFORMS = ["win32", "darwin", "linux"];
const ARCHITECTURES = ["x64", "arm64"];

for (const platform of PLATFORMS) {
	for (const arch of ARCHITECTURES) {
		generatePackage(platform, arch);
	}
}

writeManifest();
