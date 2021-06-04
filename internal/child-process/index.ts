// Node does not offer any promise-like methods for interacting with child_process other than being able to use
// util.promisify on exec and execFile
import childProcess = require("child_process");
import {AbsoluteFilePath} from "@internal/path";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticCategory,
	createSingleDiagnosticsError,
} from "@internal/diagnostics";
import {StaticMarkup, markup} from "@internal/markup";

interface ChildProcessOptions extends Omit<childProcess.SpawnOptions, "cwd"> {
	// Required `cwd`
	cwd: AbsoluteFilePath;
}

export class ChildProcess {
	constructor(command: string, args: string[], opts: ChildProcessOptions) {
		this.process = childProcess.spawn(
			command,
			args,
			{
				...opts,
				cwd: opts.cwd.join(),
			},
		);
		this.cwd = opts.cwd;
		this.command = command;
		this.args = args;
		this.output = [];

		const {stdout, stderr} = this.process;

		stdout?.on(
			"data",
			(chunk) => {
				this.output.push([0, chunk]);
			},
		);

		stderr?.on(
			"data",
			(chunk) => {
				this.output.push([1, chunk]);
			},
		);
	}

	public process: childProcess.ChildProcess;
	private command: string;
	private cwd: AbsoluteFilePath;
	private args: string[];
	private output: [0 | 1, string][];

	public getDisplayCommand(): string {
		return `${this.command} ${this.args.join(" ")}`;
	}

	public getOutput(out: boolean = true, err: boolean = true): string {
		if (!(out || err)) {
			return "";
		}

		return this.output.map(([code, chunk]) => {
			if (code === 0 && !out) {
				return "";
			}

			if (code === 1 && !err) {
				return "";
			}

			return chunk;
		}).join("");
	}

	public unexpected(
		message: StaticMarkup,
		category: DiagnosticCategory = DIAGNOSTIC_CATEGORIES["childProcess/failure"],
	) {
		throw createSingleDiagnosticsError({
			description: {
				category,
				message,
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Full command`,
					},
					{
						type: "command",
						command: this.getDisplayCommand(),
					},
					{
						type: "log",
						category: "info",
						text: markup`Output`,
					},
					{
						type: "code",
						language: "text",
						sourceText: this.getOutput(),
					},
				],
			},
			location: {
				language: "binary",
				path: this.cwd,
			},
		});
	}

	public async waitSuccess(): Promise<this> {
		const code = await this.wait();
		if (code !== 0) {
			throw this.unexpected(
				markup`Command <emphasis>${this.command}</emphasis> failed. Exited with code ${String(
					code,
				)}.`,
			);
		}
		return this;
	}

	public wait(): Promise<number | null> {
		return new Promise((resolve) => {
			this.process.on(
				"close",
				(code) => {
					resolve(code);
				},
			);
		});
	}
}

export function spawn(
	command: string,
	args: string[],
	opts: ChildProcessOptions,
): ChildProcess {
	return new ChildProcess(command, args, opts);
}
