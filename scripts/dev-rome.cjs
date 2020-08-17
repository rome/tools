// @ts-check
const child = require("child_process");
const path = require("path");
const net = require("net");
const fs = require("fs");
const os = require("os");
const tty = require("tty");

//# Utils

/**
 * @param str {string}
 * @returns {string}
 */
function red(str) {
	return `\u001b[31m${str}\u001b[39m`;
}

/**
 * @param str {string}
 * @returns {void}
 */
function heading(str) {
	console.log(`\u001b[7m ${str} \u001b[27m`);
}

/**
 * @param loc {string}
 * @returns {void}
 */
function unlink(loc) {
	if (!fs.existsSync(loc)) {
		return;
	}

	const stats = fs.lstatSync(loc);
	if (stats.isFile()) {
		fs.unlinkSync(loc);
	} else if (stats.isDirectory()) {
		for (const filename of fs.readdirSync(loc)) {
			unlink(path.join(loc, filename));
		}
		fs.rmdirSync(loc);
	}
}

/**
 * @param count {number}
 * @returns {void}
 */
function clearLines(count) {
	const {stdout} = process;

	// Clear lines output by trunk build
	for (let i = 0; i < count; i++) {
		// Cursor to beginning of line
		stdout.write(`${ANSI_ESCAPE}1G`);

		// Cursor up
		stdout.write(`${ANSI_ESCAPE}1A`);

		// Erase line
		stdout.write(`${ANSI_ESCAPE}2K`);
	}
}

//# Validate Node version

// Format of node.version is "v12.6.0" so we want to slice off the v
const versionParts = process.version.slice(1).split(".");
const major = Number(versionParts[0]);

// Keep this updated alongside engines in package.json
const EXPECTED_MAJOR = 12;

if (major < EXPECTED_MAJOR) {
	console.error(
		red(`Rome requires Node >=v${EXPECTED_MAJOR} but got ${process.version}`),
	);
	process.exit(1);
}

//# Constants

const ANSI_ESCAPE = "\x1b[";
const packageJson = require("../package.json");
const projectRoot = path.join(__dirname, "..");
const tempDevBuildFolder = path.join(os.tmpdir(), "rome-dev");

//# Init

async function isDevDaemonRunning() {
	// Path and version logic copied from internal/core/common/constants.ts
	// If there is a running daemon then we shouldn't build and just use the existing bundle
	// We'll log to let the developer know what's going on
	const version = `${packageJson.version}-dev`;
	const socketPath = path.join(os.tmpdir(), `rome-${version}.sock`);

	return new Promise((resolve) => {
		const socket = net.createConnection(
			{
				path: socketPath,
			},
			() => {
				resolve(true);
				socket.end();
			},
		);

		socket.on(
			"error",
			() => {
				resolve(false);
			},
		);
	});
}
async function buildTrunk() {
	if (await isDevDaemonRunning()) {
		console.log(
			"\x1b[1m\x1b[33m!!!! A dev daemon is currently running. Skipping new build. !!!!\x1b[39m\x1b[22m",
		);
		console.log(
			"\x1b[34mIf you want to run new code and stop the daemon you can stop the daemon with:\x1b[39m",
		);
		console.log("\x1b[2m$ ./rome stop\x1b[22m");
		console.log();
		return;
	}

	unlink(tempDevBuildFolder);
	fs.mkdirSync(tempDevBuildFolder);

	let lines = 0;

	heading("Building trunk");
	lines++;

	return new Promise((resolve) => {
		let args = [
			path.join(__dirname, "vendor/rome.cjs"),
			"bundle",
			path.join(projectRoot, "internal/cli/bin/rome.ts"),
			tempDevBuildFolder,
			"--quiet",
		];

		if (process.stdout instanceof tty.WriteStream) {
			args = [
				...args,
				"--output-columns",
				String(process.stdout.columns),
				"--output-tty",
				"--output-color-depth",
				String(process.stdout.getColorDepth()),
			];
		}

		const proc = child.spawn(process.execPath, args);

		proc.stdout.pipe(process.stdout);
		proc.stderr.pipe(process.stderr);

		/**
		 * @param chunk {Buffer}
		 */
		function countLines(chunk) {
			const match = chunk.toString().match(/\n/g);
			if (match != null) {
				lines += match.length;
			}
		}

		proc.stdout.on("data", countLines);
		proc.stderr.on("data", countLines);

		proc.on(
			"close",
			(code) => {
				if (code === 0) {
					clearLines(lines);
					resolve();
				} else {
					console.error(`Trunk build failure. Exit code ${code}`);
					process.exit(1);
				}
			},
		);
	});
}

async function execDev() {
	const args = [
		"--inspect-publish-uid=http",
		path.join(tempDevBuildFolder, "index.js"),
		...process.argv.slice(2),
	];
	if (process.env.ROME_DEV_DEBUG === "1") {
		args.unshift("--inspect", "--inspect-brk");
	}

	const res = child.spawnSync(
		process.execPath,
		[...process.execArgv, ...args],
		{
			stdio: "inherit",
			env: {
				...process.env,
				ROME_DEV: "1",
			},
		},
	);
	if (res.status !== 0) {
		process.exit(1);
	}
}

async function main() {
	await buildTrunk();
	await execDev();
}

main().then(() => {
	process.exit(0);
}).catch((err) => {
	console.error(err.stack);
	process.exit(1);
});
