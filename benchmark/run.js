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
	linter: {
		eslint: {
			repository: "https://github.com/eslint/eslint.git",
			sourceDirectories: ["lib", "messages", "tests/lib", "tools"],
		},
		webpack: {
			repository: "https://github.com/webpack/webpack.git",
			sourceDirectories: ["lib"],
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

		let projectDirectory = cloneProject(name, configuration.repository);

		const prettierPaths = Object.entries(configuration.sourceDirectories)
			.flatMap(([directory, extensions]) => {
				return extensions.map(
					(extension) => `'${path.join(directory, `**/*.${extension}`)}'`,
				);
			})
			.join(" ");

		const prettierCommand = `\"${resolvePrettier()}\" ${prettierPaths} --write`;

		const romeCommand = `${rome} format ${Object.keys(
			configuration.sourceDirectories,
		)
			.map((path) => `'${path}'`)
			.join(" ")} --write`;

		const romeSingleCoreCommand = withEnvVariable(
			"RAYON_NUM_THREADS",
			"1",
			romeCommand,
		);

		// Run 2 warmups to make sure the files are formatted correctly
		const hyperfineCommand = `hyperfine -w 2 -n Prettier "${prettierCommand}" -n Rome "${romeCommand}" --shell=${shellOption()} -n "Rome (1 thread)" "${romeSingleCoreCommand}"`;
		console.log(hyperfineCommand);

		child_process.execSync(hyperfineCommand, {
			cwd: projectDirectory,
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

function benchmarkLinter(rome) {
	console.log("");
	console.log("Benchmark linter...");
	console.log("―".repeat(80));
	console.log("");

	for (const [name, configuration] of Object.entries(BENCHMARKS.linter)) {
		console.log(`[${name}]`);

		let projectDirectory = cloneProject(name, configuration.repository);

		const eslintConfig = fs.readFileSync("./bench.eslint.js");
		fs.writeFileSync(
			path.join(projectDirectory, "bench.eslintrc.js"),
			eslintConfig,
		);
		const romeConfig = fs.readFileSync("./bench.rome.json");
		fs.writeFileSync(path.join(projectDirectory, "rome.json"), romeConfig);

		const eslintPaths = configuration.sourceDirectories
			.map((directory) => `'${directory}/**'`)
			.join(" ");

		const eslintCommand = `${resolveESlint()} --no-ignore -c bench.eslintrc.js ${eslintPaths}`;

		const romePaths = configuration.sourceDirectories
			.map((directory) => `'${directory}'`)
			.join(" ");

		const romeCommand = `${rome} check ${romePaths}`;

		const romeSingleCoreCommand = withEnvVariable(
			"RAYON_NUM_THREADS",
			"1",
			romeCommand,
		);

		// Run 2 warmups to make sure the files are formatted correctly
		const hyperfineCommand = `hyperfine -i -w 2 -n ESLint "${eslintCommand}" -n Rome "${romeCommand}" --shell=${shellOption()} -n "Rome (1 thread)" "${romeSingleCoreCommand}"`;
		console.log(hyperfineCommand);

		child_process.execSync(hyperfineCommand, {
			cwd: projectDirectory,
			stdio: "inherit",
		});
	}
}

function resolveESlint() {
	return path.resolve("./node_modules/.bin/eslint");
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

function withEnvVariable(name, value, command) {
	switch (process.platform) {
		case "win32": {
			return `$Env:${name}=${value}; ${command}`;
		}
		default:
			return `${name}=\"${value}\" ${command}`;
	}
}

function withDirectory(cwd) {
	return {
		run(command, options) {
			child_process.execSync(command, {
				cwd,
				...options,
			});
		},
	};
}

function cloneProject(name, repository) {
	let projectDirectory = path.join(TMP_DIRECTORY, name);

	let inProjectDirectory = withDirectory(projectDirectory);

	if (fs.existsSync(projectDirectory)) {
		console.log("Updating");
		inProjectDirectory.run("git reset --hard @{u}");
		inProjectDirectory.run("git clean -df");
		inProjectDirectory.run("git pull --depth=1");
	} else {
		console.log("Clone project...");

		withDirectory(TMP_DIRECTORY).run(`git clone --depth=1 ${repository}`, {
			stdio: "inherit",
		});
	}

	return projectDirectory;
}

function run() {
	fs.mkdirSync("target", { recursive: true });

	const rome = buildRome();

	benchmarkFormatter(rome);
	benchmarkLinter(rome);
}

run();
