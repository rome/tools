const fs = require("fs");
const child_process = require("child_process");
const path = require("path");
const { dir } = require("console");

const TMP_DIRECTORY = path.resolve("./target");

function buildRome() {
	console.log("Build Rome...");

	child_process.execSync("cargo build --bin rome --release");

	return path.resolve("../target/release/rome");
}

const BENCHMARKS = {
	formatter: {
		webpack: {
			repository: "https://github.com/webpack/webpack.git",
			sourceDirectories: {
				lib: ["js"],
				examples: ["js"],
				declarations: ["ts"],
				benchmark: ["js"],
			},
		},
		prettier: {
			repository: "https://github.com/prettier/prettier.git",
			sourceDirectories: {
				src: ["js"],
				scripts: ["js", "mjs"],
			},
		},
	},
};

function benchmarkFormatter(rome) {
	console.log("");
	console.log("Benchmark formatter...");
	console.log("―".repeat(80));
	console.log("");

	for (const [name, configuration] of Object.entries(BENCHMARKS.formatter)) {
		console.log(`[${name}]`);

		if (fs.existsSync(path.join(TMP_DIRECTORY, name))) {
			console.log("Updating");
			runScript("git reset --hard @{u}");
			runScript("git clean -df");
			runScript("git pull --depth=1");
		} else {
			console.log("Clone project...");

			runScript(`git clone --depth=1 ${configuration.repository}`, {
				stdio: "inherit",
			});
		}

		const prettierPaths = Object.entries(configuration.sourceDirectories)
			.flatMap(([directory, extensions]) => {
				return extensions.map(
					(extension) => `\"${path.join(directory, `**/*.${extension}`)}\"`,
				);
			})
			.join(" ");

		const prettierCommand = `\"${resolvePrettier()}\" ${prettierPaths} --write`;

		const romeCommand = `${rome} format ${Object.keys(
			configuration.sourceDirectories,
		)
			.map((path) => `\"${path}\"`)
			.join(" ")} --write`;

		const romeSingleCoreCommand = `${setEnvScript(
			"RAYON_NUM_THREADS",
			"1",
		)}; ${romeCommand}`;

		// Run 2 warmups to make sure the files are formatted correctly
		const hyperfineCommand = `hyperfine -w 2 -n Prettier "${prettierCommand}" -n Rome "${romeCommand}" --shell=${shellOption()} -n "Rome (1 thread)" "${romeSingleCoreCommand}"`;
		console.log(hyperfineCommand);

		child_process.execSync(hyperfineCommand, {
			cwd: path.join(TMP_DIRECTORY, name),
			stdio: "inherit",
		});
	}
}

function resolvePrettier() {
	switch (process.platform) {
		case "win32":
			// Use the powershell binary or Prettier spawns a new cmd
			return path.resolve("./node_modules/.bin/prettier.ps1");
		default:
			return path.resolve("./node_modules/.bin/prettier");
	}
}

function shellOption() {
	switch (process.platform) {
		case "win32":
			// Use Powershell so that it is possible to set an environment variable for a single command (ugh!)
			return "pwsh";
		default:
			return "default";
	}
}

function setEnvScript(name, value) {
	switch (process.platform) {
		case "win32": {
			return `$Env:${name}=${value}`;
		}
		default:
			return `set ${name}=\"${value}\"`;
	}
}

function runScript(command, options) {
	child_process.execSync(command, {
		cwd: TMP_DIRECTORY,
		...options,
	});
}

function run() {
	fs.mkdirSync("target", { recursive: true });

	const rome = buildRome();

	benchmarkFormatter(rome);
}

run();
