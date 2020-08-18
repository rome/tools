// Node does not offer any promise-like methods for interacting with child_process other than being able to use
// util.promisify on exec and execFile
import childProcess = require("child_process");
import {AbsoluteFilePath} from "@internal/path";
import {dedent} from "@internal/string-utils";

interface ChildProcessOptions extends Omit<childProcess.SpawnOptions, "cwd"> {
	cwd?: AbsoluteFilePath;
}

export class ChildProcess {
	constructor(command: string, args: Array<string>, opts: ChildProcessOptions) {
		this.process = childProcess.spawn(
			command,
			args,
			{
				...opts,
				cwd: opts.cwd === undefined ? undefined : opts.cwd.join(),
			},
		);
		this.command = command;
		this.args = args;
		this.output = [];

		const {stdout, stderr} = this.process;

		if (stdout != null) {
			stdout.on(
				"data",
				(chunk) => {
					this.output.push([0, chunk]);
				},
			);
		}

		if (stderr != null) {
			stderr.on(
				"data",
				(chunk) => {
					this.output.push([1, chunk]);
				},
			);
		}
	}

	public process: childProcess.ChildProcess;
	private command: string;
	private args: Array<string>;
	private output: Array<[0 | 1, string]>;

	public getDisplayCommand(): string {
		return `${this.command} ${this.args.join(" ")}`;
	}

	public getOutput(out: boolean = true, err: boolean = true): string {
		if (!out && !err) {
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

	public unexpected(message: string) {
		throw new Error(
			dedent`
			${message}
			Command: ${this.getDisplayCommand()}
			stderr: ${this.getOutput(false, true)}
		`,
		);
	}

	public async waitSuccess(): Promise<this> {
		const code = await this.wait();
		if (code !== 0) {
			throw this.unexpected(`Command failed. Exited with code ${code}`);
		}
		return this;
	}

	public wait(): Promise<number> {
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
	args: Array<string>,
	opts: ChildProcessOptions = {},
): ChildProcess {
	return new ChildProcess(command, args, opts);
}
