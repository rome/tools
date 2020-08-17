/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from "@internal/cli-reporter";
import {SerializeCLIOptions, serializeCLIFlags} from "./serializeCLIFlags";
import {
	ConsumePath,
	ConsumePropertyDefinition,
	ConsumeSourceLocationRequestTarget,
	Consumer,
	consume,
} from "@internal/consume";
import {
	dedent,
	findClosestStringMatch,
	naturalCompare,
	toCamelCase,
	toKebabCase,
} from "@internal/string-utils";
import {AbsoluteFilePath, HOME_PATH, createUnknownPath} from "@internal/path";
import {Dict} from "@internal/typescript-helpers";
import {
	AnyMarkups,
	StaticMarkup,
	concatMarkup,
	markup,
	readMarkup,
} from "@internal/markup";
import {
	Diagnostic,
	DiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {exists, readFileText, writeFile} from "@internal/fs";
import {prettyFormatEager} from "@internal/pretty-format";
import highlightShell from "@internal/markup-syntax-highlight/highlightShell";
import {RSERObject} from "@internal/codec-binary-serial";
import {ExtendedMap} from "@internal/collections";

export type Examples = Array<{
	description: StaticMarkup;
	command: string;
}>;

type FlagsConsumer = {
	flags: Consumer;
	defaultFlags: Dict<FlagValue>;
	rawFlags: Dict<FlagValue>;
};

type CommandOptions<T extends RSERObject> = {
	name: string;
	category?: string;
	description?: StaticMarkup;
	usage?: string;
	examples?: Examples;
	ignoreFlags?: Array<string>;
	hidden?: boolean;
	defineFlags?: (consumer: Consumer) => T;
	callback: (flags: T) => void | Promise<void>;
};

type AnyCommandOptions = CommandOptions<RSERObject>;

type ArgDeclaration = {
	definition: ConsumePropertyDefinition;
	name: string;
	command: undefined | string;
};

type DefinedCommand = {
	flags: RSERObject;
	command: AnyCommandOptions;
};

export type ParserOptions<T> = {
	reporter: Reporter;
	programName: string;
	cwd: AbsoluteFilePath;
	args: Array<string>;
	defineFlags: (consumer: Consumer) => T;

	examples?: Examples;
	usage?: string;
	description?: StaticMarkup;
	version?: string;
	ignoreFlags?: Array<string>;
	noProcessExit?: boolean;
	commandRequired?: boolean;
	commandSuggestions?: Dict<{
		commandName: string;
		description: StaticMarkup;
	}>;
	shellCompletionDirectory?: AbsoluteFilePath;
};

function splitCommandName(cmd: string): Array<string> {
	return cmd.split(" ");
}

// Whether we can display this value in help
function isDisplayableHelpValue(value: unknown): value is string | number {
	return typeof value === "string" || typeof value === "number";
}

type _FlagValue = undefined | number | string | boolean;

export type FlagValue = _FlagValue | Array<_FlagValue>;

type SupportedCompletionShells = "bash" | "fish";

export default class Parser<T> {
	constructor(opts: ParserOptions<T>) {
		this.reporter = opts.reporter;
		this.opts = opts;

		this.shorthandFlags = new Set();
		this.incorrectCaseFlags = new Set();
		this.declaredFlags = new Map();
		this.defaultFlags = new Map();
		this.flags = new Map();
		this.args = [];

		// These are used to track where we should insert an argument for a boolean flag value
		this.flagToArgIndex = new ExtendedMap("flagToArgIndex");
		this.flagToArgOffset = 0;

		this.consumeRawArgs(opts.args);

		this.commands = new Map();
		this.ranCommand = undefined;
		this.currentCommand = undefined;
	}

	private reporter: Reporter;
	private opts: ParserOptions<T>;
	private incorrectCaseFlags: Set<string>;
	private shorthandFlags: Set<string>;
	private flags: Map<string, FlagValue>;
	private defaultFlags: Map<string, unknown>;
	private declaredFlags: Map<string, ArgDeclaration>;
	private flagToArgIndex: ExtendedMap<string, number>;
	private flagToArgOffset: number;
	private currentCommand: undefined | string;
	private ranCommand: undefined | AnyCommandOptions;
	private commands: Map<string, AnyCommandOptions>;
	public args: Array<string>;

	private looksLikeFlag(flag: undefined | string): boolean {
		return flag?.[0] === "-";
	}

	private toCamelCase(name: string): string {
		const camelName = toCamelCase(name);

		// Don't allow passing in straight camelcased names
		if (toKebabCase(name) !== name) {
			this.incorrectCaseFlags.add(name);
		}

		return camelName;
	}

	private setFlag(key: string, value: string | boolean) {
		let newValue: FlagValue = value;
		const existing = this.flags.get(key);
		if (existing !== undefined) {
			if (Array.isArray(existing)) {
				newValue = [...existing, value];
			} else {
				newValue = [existing, value];
			}
		}
		this.flags.set(key, newValue);
	}

	private consumeRawArgs(rawArgs: Array<string>) {
		while (rawArgs.length > 0) {
			const arg: string = String(rawArgs.shift());

			if (arg === "--") {
				// We consider a -- by itself to halt parsing of args, the rest of the remaining args are added to _
				this.args = this.args.concat(rawArgs);
				break;
			} else if (arg[0] === "-") {
				// Clean the argument by stripping off the dashes
				const name = arg[1] === "-" ? arg.slice(2) : arg.slice(1);

				// Flags beginning with no- are always false
				if (name.startsWith("no-")) {
					const camelName = this.toCamelCase(name.slice(3));
					this.setFlag(camelName, false);
					continue;
				}

				// Allow for arguments to be passed as --foo=bar
				const equalsIndex = name.indexOf("=");
				if (equalsIndex !== -1) {
					const cleanName = this.toCamelCase(name.slice(0, equalsIndex));
					const value = name.slice(equalsIndex + 1);
					this.setFlag(cleanName, value);
					continue;
				}

				const camelName = this.toCamelCase(name);

				// If the next argument is a flag or we're at the end of the args then just set it to `true`
				if (rawArgs.length === 0 || this.looksLikeFlag(rawArgs[0])) {
					this.setFlag(camelName, true);
				} else {
					// Otherwise, take that value
					this.setFlag(camelName, String(rawArgs.shift()));
				}

				if (arg[0] === "-" && arg[1] !== "-") {
					this.shorthandFlags.add(camelName);
				}

				this.flagToArgIndex.set(camelName, this.args.length);
			} else {
				// Not a flag and hasn't been consumed already by a previous arg so it must be a file
				this.args.push(arg);
			}
		}
	}

	private getFlagsConsumer(): FlagsConsumer {
		const defaultFlags: Dict<FlagValue> = {};

		const flags: Dict<FlagValue> = {};
		for (const [key, value] of this.flags) {
			flags[toCamelCase(key)] = value;
		}

		const consumer = consume({
			filePath: createUnknownPath("argv"),
			value: flags,
			onDefinition: (def, valueConsumer) => {
				const key = def.objectPath.join(".");

				// Detect root object
				if (key === "") {
					return;
				}

				// These flags are ambiguous with how we handle `--no-` booleans
				if (toKebabCase(key).startsWith("no-")) {
					throw new Error(`CLI flag ${key} cannot start with "no"`);
				}

				const value = flags[key];

				// Allow omitting a string flag value
				if (def.type === "string" && value === true) {
					valueConsumer.setValue("");
				}

				this.declareArgument({
					name: key,
					command: this.currentCommand,
					definition: def,
				});
				defaultFlags[key] = (def.default as FlagValue);

				// Automatically convert number strings
				if (def.type === "number" && typeof value !== "number") {
					if (valueConsumer.exists() || def.required) {
						valueConsumer.setValue(valueConsumer.asNumberString());
					}
				}

				// We've parsed arguments like `--foo bar` as `{foo: 'bar}`
				// However, --foo may be a boolean flag, so `bar` needs to be correctly added to args
				if (
					def.type === "boolean" &&
					value !== true &&
					value !== false &&
					value !== undefined
				) {
					const argIndex = this.flagToArgIndex.assert(key);

					// Insert the argument at the correct place
					this.args.splice(argIndex + this.flagToArgOffset, 0, String(value));

					// Increase offset to correct subsequent insertions
					this.flagToArgOffset++;

					//
					valueConsumer.setValue(true);
				}
			},
			context: {
				category: "flags/invalid",
				normalizeKey: (key) => {
					return this.incorrectCaseFlags.has(key) ? key : toKebabCase(key);
				},
				getOriginalValue: (keys: ConsumePath) => {
					return flags[keys[0]];
				},
				getDiagnosticLocation: (
					keys: ConsumePath,
					target: ConsumeSourceLocationRequestTarget,
				) => {
					return serializeCLIFlags(
						{
							...this.getSerializeOptions(),
							defaultFlags,
							flags,
						},
						{
							type: "flag",
							key: String(keys[0]),
							target,
						},
					);
				},
			},
		});

		return {flags: consumer, defaultFlags, rawFlags: flags};
	}

	private declareArgument(decl: ArgDeclaration) {
		// Commands may have colliding flags, this is only a problem in help mode, so make it unique
		const key =
			decl.command === undefined ? decl.name : `${decl.command}.${decl.name}`;

		// Ensure it hasn't been declared more than once
		if (this.declaredFlags.has(key)) {
			throw new Error(`Already declared argument ${key}`);
		}

		// Declare argument
		this.declaredFlags.set(key, decl);
		this.defaultFlags.set(key, decl.definition.default);
	}

	public getInterface(): ParserInterface<T> {
		return new ParserInterface(this);
	}

	private async maybeDefineCommandFlags(
		command: AnyCommandOptions,
		consumer: Consumer,
	): Promise<undefined | RSERObject> {
		// A command name could be made of multiple strings
		const commandParts = splitCommandName(command.name);
		for (let i = 0; i < commandParts.length; i++) {
			if (commandParts[i] !== this.args[i]) {
				return;
			}
		}

		// Remove command name from arguments
		this.args = this.args.slice(commandParts.length);
		return await this.defineCommandFlags(command, consumer);
	}

	private checkBadFlags(
		consumer: Consumer,
		definedCommand: undefined | DefinedCommand,
	) {
		// Ignore flags from command and root parser options
		const ignoreFlags: Array<string> = [
			...((definedCommand !== undefined && definedCommand.command.ignoreFlags) || []),
			...(this.opts.ignoreFlags || []),
		];
		for (const key of ignoreFlags) {
			this.shorthandFlags.delete(key);
			this.incorrectCaseFlags.delete(key);
			consumer.markUsedProperty(key);
		}

		for (const [key] of this.flags) {
			if (this.shorthandFlags.has(key)) {
				const def = this.declaredFlags.get(key)?.definition;
				if (def && def.metadata?.alternateName !== key) {
					consumer.get(key).unexpected(
						descriptions.FLAGS.UNSUPPORTED_SHORTHAND(key),
					);
				}
			}
		}

		for (const incorrectName of this.incorrectCaseFlags) {
			consumer.get(incorrectName).unexpected(
				descriptions.FLAGS.INCORRECT_CASED_FLAG(incorrectName),
			);
		}

		consumer.enforceUsedProperties("flag", false);
	}

	private async writeShellCompletions(
		shell: SupportedCompletionShells,
		directory: AbsoluteFilePath = HOME_PATH,
	) {
		const {programName} = this.opts;
		const {reporter} = this;
		let path;

		// Figure out profiles and basename to use for the completion script
		switch (shell) {
			case "bash": {
				path = directory.append(`.${programName}-completion.sh`);
				break;
			}

			case "fish": {
				path = HOME_PATH.append(
					".config",
					"fish",
					"completions",
					`${programName}.fish`,
				);
				break;
			}
		}

		// Write completions
		const res = await this.generateShellCompletions(shell);
		await writeFile(path, res);
		reporter.success(
			markup`Wrote shell completions to <emphasis>${path}</emphasis>`,
		);

		// Tell the user the next step
		switch (shell) {
			case "bash": {
				const possibleProfiles = [];
				possibleProfiles.push(HOME_PATH.append(".bashrc"));
				possibleProfiles.push(HOME_PATH.append(".bash_profile"));

				// Find the profile
				let profilePath;
				for (const path of possibleProfiles) {
					if (await exists(path)) {
						profilePath = path;
						break;
					}
				}
				if (profilePath === undefined) {
					reporter.error(
						markup`Could not find your bash profile. Tried the following:`,
					);
					reporter.list(
						possibleProfiles.map((path) => {
							return markup`${path}`;
						}),
					);
				} else {
					let file = await readFileText(profilePath);
					if (file.includes(path.getBasename())) {
						reporter.warn(
							markup`Skipped <emphasis>${profilePath}</emphasis> modifications as looks like it was already included`,
						);
					} else {
						file = file.trim();
						file += "\n";
						file += `source ${path.relative(profilePath).preferExplicitRelative().join()}`;
						file += "\n";
						await writeFile(profilePath, file);
						reporter.success(
							markup`Added completions to <emphasis>${profilePath}</emphasis>`,
						);
					}
				}
				break;
			}
		}

		reporter.info(markup`Restart your shell to enable!`);
		this.exit(0);
	}

	private async logShellCompletions(shell: SupportedCompletionShells) {
		const res = await this.generateShellCompletions(shell);
		this.reporter.logRaw(res);
		this.exit(0);
	}

	public async init(): Promise<T> {
		const flagsConsumer = this.getFlagsConsumer();
		const {flags} = flagsConsumer;

		// Show help for --version
		const version = this.opts.version;
		if (version !== undefined) {
			const shouldDisplayVersion = flags.get(
				"version",
				{
					description: markup`Show the version`,
				},
			).asBoolean(false);
			if (shouldDisplayVersion) {
				this.reporter.log(markup`${version}`);
				this.exit(0);
			}
		}

		const {shellCompletionDirectory} = this.opts;
		// `--write-shell-completions <SHELL>` writes the commands to a file
		const writeShellCompletions: undefined | SupportedCompletionShells = flags.get(
			"writeShellCompletions",
			{
				description: markup`Write shell completion commands`,
				inputName: "shell",
			},
		).asStringSetOrVoid(["fish", "bash"]);
		if (writeShellCompletions !== undefined) {
			await this.writeShellCompletions(
				writeShellCompletions,
				shellCompletionDirectory,
			);
		}

		// `--generate-shell-completions <SHELL>` writes the commands to stdout
		const logShellCompletions: undefined | SupportedCompletionShells = flags.get(
			"logShellCompletions",
			{
				description: markup`Generate shell completion commands`,
				inputName: "shell",
			},
		).asStringSetOrVoid(["fish", "bash"]);
		if (logShellCompletions !== undefined) {
			await this.logShellCompletions(logShellCompletions);
		}

		// Show help for --help
		const shouldShowHelp = flags.get(
			"help",
			{
				description: markup`Show this help screen`,
				alternateName: "h",
			},
		).asBoolean(false);

		let definedCommand: undefined | DefinedCommand;

		const rootFlags = await flags.bufferDiagnostics(async (consumer) => {
			const rootFlags = this.opts.defineFlags(consumer);

			for (const [key, command] of this.commands) {
				const definedFlags = await this.maybeDefineCommandFlags(
					command,
					consumer,
				);
				if (definedFlags !== undefined) {
					this.currentCommand = key;
					definedCommand = {flags: definedFlags, command};
					break;
				}
			}

			this.checkBadFlags(consumer, definedCommand);

			this.currentCommand = undefined;

			if (this.opts.commandRequired && !shouldShowHelp) {
				this.commandRequired(definedCommand !== undefined, flagsConsumer);
			}

			return rootFlags;
		});

		// Show help for --help
		if (shouldShowHelp) {
			await this.showHelp(
				definedCommand === undefined ? undefined : definedCommand.command,
			);
			this.exit(1);
		}

		if (definedCommand !== undefined) {
			this.ranCommand = definedCommand.command;
			if (definedCommand.command.hidden === true) {
				this.reporter.warn(
					markup`This command has been hidden. Consider its usage to be experimental and do not expect support or backwards compatibility.`,
				);
			}
			await definedCommand.command.callback(definedCommand.flags);
		}

		return rootFlags;
	}

	private buildOptionsHelp(keys: Array<string>): Array<AnyMarkups> {
		const optionOutput: Array<{
			argName: string;
			arg: StaticMarkup;
			description: StaticMarkup;
		}> = [];
		let argColumnLength: number = 0;

		// Build up options, we need to do this to line up the columns correctly
		for (const key of keys) {
			const decl = this.declaredFlags.get(key)!;

			const {definition: def} = decl;
			const {metadata} = def;
			let argName = decl.name;
			let argCol = toKebabCase(decl.name);

			// For booleans that default to `true`, show the --no- version as that'll be what users should use
			if (def.type === "boolean" && def.default === true) {
				argCol = `--no-${argCol}`;
				argName = `no-${argName}`;
			} else {
				argCol = `--${argCol}`;
			}

			// Add input specifier unless a boolean
			if (def.type !== "boolean") {
				let {inputName} = metadata;

				if (inputName === undefined) {
					if (def.type === "number") {
						inputName = "num";
					} else {
						inputName = "input";
					}
				}

				argCol += ` <${inputName}>`;
			}

			if (metadata.alternateName) {
				argCol += `, -${metadata.alternateName}`;
			}

			// Set arg col length if we'll be longer
			if (argColumnLength < argCol.length) {
				argColumnLength = argCol.length;
			}

			let descCol: StaticMarkup =
				metadata.description === undefined
					? markup`no description found`
					: metadata.description;

			const {default: defaultValue} = def;
			if (defaultValue !== undefined && isDisplayableHelpValue(defaultValue)) {
				descCol = markup`${descCol} - default ${prettyFormatEager(defaultValue)}`;
			}

			if (def.type === "string" && def.allowedValues !== undefined) {
				const displayAllowedValues = def.allowedValues.filter((item) =>
					isDisplayableHelpValue(item)
				);
				if (displayAllowedValues !== undefined) {
					const printedValues = concatMarkup(
						displayAllowedValues.map((value) => prettyFormatEager(value)),
						markup` `,
					);
					descCol = markup`${descCol} - values ${printedValues})`;
				}
			}

			optionOutput.push({
				argName,
				arg: concatMarkup(
					highlightShell({input: argCol, isShorthand: !!metadata.alternateName}),
					markup` `,
				),
				description: descCol,
			});
		}

		// Sort options by argument name
		optionOutput.sort((a, b) => naturalCompare(a.argName, b.argName));

		// Build table rows
		return optionOutput.map((opt) => [
			markup`<view align="right">${opt.arg}</view>`,
			opt.description,
		]);
	}

	private async showUsageHelp(
		description?: StaticMarkup,
		usage: string = "[flags]",
		prefix?: string,
	) {
		const {reporter} = this;
		const {programName} = this.opts;

		await reporter.section(
			markup`Usage`,
			() => {
				if (description !== undefined) {
					reporter.log(description);
					reporter.br({force: true});
				}

				const commandParts = [programName];
				if (prefix !== undefined) {
					commandParts.push(prefix);
				}
				commandParts.push(usage);

				const command = commandParts.join(" ");
				reporter.command(command, false);
			},
		);
	}

	private async showFocusedCommandHelp(command: AnyCommandOptions) {
		const {reporter} = this;
		const {name, usage, description, examples} = command;

		reporter.br({force: true});
		await this.showUsageHelp(description, usage, name);
		await this.showHelpExamples(examples, name);

		// Find arguments that belong to this command
		const argKeys = [];
		for (const [key, decl] of this.declaredFlags) {
			if (decl.command === name) {
				argKeys.push(key);
			}
		}

		const optRows = this.buildOptionsHelp(argKeys);
		if (optRows.length > 0) {
			await reporter.section(
				markup`Command Flags`,
				() => {
					reporter.table([], optRows);
				},
			);
		}

		await reporter.section(
			markup`Global Flags`,
			() => {
				reporter.info(markup`To view global flags run`);
				reporter.command("rome --help");
			},
		);
	}

	private async showGlobalFlags() {
		const {reporter} = this;
		await reporter.section(
			markup`Global Flags`,
			() => {
				// Show options not attached to any commands
				const lonerArgKeys = [];
				for (const [key, decl] of this.declaredFlags) {
					if (decl.command === undefined) {
						lonerArgKeys.push(key);
					}
				}

				reporter.table([], this.buildOptionsHelp(lonerArgKeys));
			},
		);
	}

	private async generateShellCompletions(
		shell: SupportedCompletionShells,
	): Promise<string> {
		// Execute all command defineFlags. Only one is usually ran when the arguments match the command name.
		// But to generate autocomplete we want all the flags to be declared for all commands.
		const {flags} = this.getFlagsConsumer();
		for (const command of this.commands.values()) {
			// capture() will cause diagnostics to be suppressed
			const {consumer} = flags.capture();
			await this.defineCommandFlags(command, consumer);
		}

		const {programName} = this.opts;

		switch (shell) {
			case "bash": {
				return this.genBashCompletions(programName);
			}
			case "fish": {
				return this.genFishCompletions(programName);
			}
		}
	}

	private genFishCompletions(prg: string): string {
		let script = "";
		const scriptPre = `complete -c ${prg}`;

		// add rome
		script += `${scriptPre} -f\n`;

		// add command completions
		for (let [subcmd, meta] of this.commands.entries()) {
			// add command description if exists
			let description = "";
			if (meta.description) {
				description += ` -d '${readMarkup(meta.description)}'`;
			}

			script += `${scriptPre} -n '__fish_use_subcommand' -a '${subcmd}'${description}\n`;
		}

		// add flag completions
		for (let meta of this.declaredFlags.values()) {
			const subcmdCond =
				meta.command === undefined
					? ""
					: `-n '__fish_seen_subcommand_from ${meta.command}'`;
			script += `${scriptPre} ${subcmdCond} -l '${meta.name}'\n`;
		}

		return script;
	}

	private genBashCompletions(prg: string): string {
		let romeCmds = "";
		let commandFuncs = "";
		let globalFlags = "";
		let cmdFlagMap = new Map();

		for (let subcmd of this.commands.keys()) {
			romeCmds += `${subcmd} `;
		}

		for (let meta of this.declaredFlags.values()) {
			if (meta.command === undefined) {
				globalFlags += `--${meta.name} `;
			} else {
				if (cmdFlagMap.has(meta.command)) {
					cmdFlagMap.set(
						meta.command,
						`${cmdFlagMap.get(meta.command)} --${meta.name}`,
					);
				} else {
					cmdFlagMap.set(meta.command, `--${meta.name}`);
				}
			}
		}

		for (let [cmd, flags] of cmdFlagMap.entries()) {
			commandFuncs += `
      __${prg}_${cmd}()
      {
        cmds="";
        local_flags="${flags}"
      }
      `;
		}

		let romeFunc = `
      __${prg}()
      {
          cmds="${romeCmds}"
          local_flags="";
      }
    `;

		let mainScript = `
      #!/usr/bin/env bash
      global_flags="${globalFlags}"

      # initial state
      cmds=""
      local_flags=""

      __is_flag()
      {
        case $1 in
          -*) echo "true"
        esac
      }

      __${prg}_gen_completions()
      {
        local suggestions func flags index

        index="$((\${#COMP_WORDS[@]} - 1))"

        flags="$global_flags $local_flags"

        func="_"

        for ((i=0; i < index; i++))
        do
          leaf=$(echo \${COMP_WORDS[$i]} | grep -o '[^/]*$')
          if [[ ! $(__is_flag $leaf) ]]; then
            func="\${func}_\${leaf}"
          fi
        done

        $func 2> /dev/null

        if [[ $(__is_flag \${COMP_WORDS[$index]}) ]]; then
          suggestions=$flags
        else
          suggestions=$cmds
        fi

        COMPREPLY=($(compgen -W "$suggestions" -- "\${COMP_WORDS[$index]}"))
      }
    `;

		return dedent`
      ${mainScript}
      ${commandFuncs}
      ${romeFunc}
      complete -F __${prg}_gen_completions ${prg}
    `;
	}

	public async showHelp(
		command: undefined | AnyCommandOptions = this.ranCommand,
	) {
		if (command !== undefined) {
			await this.showFocusedCommandHelp(command);
			return;
		}

		const {reporter} = this;
		const {description, usage, examples, programName} = this.opts;

		await this.showUsageHelp(description, usage);
		await this.showGlobalFlags();

		// Sort commands into their appropriate categories for output
		const commandsByCategory: ExtendedMap<
			undefined | string,
			Array<AnyCommandOptions>
		> = new ExtendedMap("commandsByCategory", () => []);
		const categoryNames: Set<string | undefined> = new Set();
		for (const [name, command] of this.commands) {
			if (name[0] === "_") {
				continue;
			}

			const {category} = command;
			const commandsForCategory = commandsByCategory.assert(category);
			commandsForCategory.push(command);
			categoryNames.add(category);
		}

		await reporter.section(
			markup`Commands`,
			() => {
				const sortedCategoryNames: Array<string | undefined> = Array.from(
					categoryNames,
				).sort();

				// Always make sure categoryless commands are displayed first
				if (sortedCategoryNames.includes(undefined)) {
					sortedCategoryNames.splice(sortedCategoryNames.indexOf(undefined), 1);
					sortedCategoryNames.unshift(undefined);
				}

				for (const category of sortedCategoryNames) {
					const commands = commandsByCategory.get(category)!.filter((c) => {
						return !c.hidden;
					});

					if (commands.length === 0) {
						continue;
					}

					if (category !== undefined) {
						reporter.log(markup`<emphasis>${category} Commands</emphasis>`);
					}

					// Sort by name
					commands.sort((a, b) => a.name.localeCompare(b.name));

					reporter.list(
						commands.map((cmd) => {
							return markup`<emphasis>${cmd.name}</emphasis> ${cmd.description ===
							undefined
								? ""
								: cmd.description}`;
						}),
					);
					reporter.br();
				}

				reporter.info(markup`To view help for a specific command run`);
				reporter.command(`${programName} command_name --help`);
			},
		);

		await this.showHelpExamples(examples);
	}

	private async showHelpExamples(examples?: Examples, prefix?: string) {
		const {programName} = this.opts;
		const {reporter} = this;

		if (examples === undefined || examples.length === 0) {
			return;
		}

		await reporter.section(
			markup`Examples`,
			() => {
				for (const {description, command} of examples) {
					const commandParts = [];
					if (programName !== undefined) {
						commandParts.push(programName);
					}
					if (prefix !== undefined) {
						commandParts.push(prefix);
					}
					commandParts.push(command);

					const builtCommand = commandParts.join(" ");

					reporter.br();
					if (description !== undefined) {
						reporter.log(description);
					}
					reporter.command(builtCommand);
				}
			},
		);
	}

	private commandRequired(
		foundCommand: boolean,
		{defaultFlags, rawFlags}: FlagsConsumer,
	) {
		if (foundCommand) {
			return;
		}

		const {programName, commandSuggestions} = this.opts;
		let {args} = this;
		let commandName = args.join(" ");
		let displayArgs: Array<string> = [];

		const opts: Parameters<typeof descriptions.FLAGS.UNKNOWN_COMMAND>[0] = {
			programName,
			commandName,
			suggestedName: undefined,
			suggestedDescription: undefined,
			suggestedCommand: undefined,
		};

		// If we were provided with a list of command suggestions, try and find one
		if (commandSuggestions !== undefined) {
			for (let i = 0; i < args.length; i++) {
				const possibleCommandName = args.slice(0, i + 1).join(" ");
				const suggestion = commandSuggestions[possibleCommandName];
				if (suggestion !== undefined) {
					commandName = possibleCommandName;
					displayArgs = args.slice(i + 1);

					opts.suggestedName = suggestion.commandName;
					opts.suggestedDescription = suggestion.description;
					break;
				}
			}
		}

		// If we don't have a suggestion then try to find another closest one
		if (opts.suggestedName === undefined) {
			opts.suggestedName = findClosestStringMatch(
				commandName,
				Array.from(this.commands.keys()),
			);
		}

		// Set suggestedCommand
		if (opts.suggestedName !== undefined) {
			opts.suggestedCommand = serializeCLIFlags(
				{
					...this.getSerializeOptions(),
					commandName: opts.suggestedName,
					args: displayArgs,
					defaultFlags,
					flags: rawFlags,
				},
				"none",
			).sourceText;
		}

		const diag: Diagnostic = {
			description: descriptions.FLAGS.UNKNOWN_COMMAND(opts),
			location: serializeCLIFlags(
				{
					...this.getSerializeOptions(),
					commandName,
					args: displayArgs,
					defaultFlags,
					flags: rawFlags,
				},
				"command",
			),
		};

		throw new DiagnosticsError("Unknown command", [diag]);
	}

	private getSerializeOptions(): Pick<
		SerializeCLIOptions,
		| "programName"
		| "commandName"
		| "args"
		| "incorrectCaseFlags"
		| "shorthandFlags"
		| "cwd"
	> {
		return {
			programName: this.opts.programName,
			commandName: this.currentCommand,
			args: this.args,
			incorrectCaseFlags: this.incorrectCaseFlags,
			shorthandFlags: this.shorthandFlags,
			cwd: this.opts.cwd,
		};
	}

	public addCommand(opts: AnyCommandOptions) {
		if (this.currentCommand !== undefined) {
			throw new Error("Nested commands aren't allowed");
		}

		this.commands.set(opts.name, opts);
	}

	private async defineCommandFlags(
		command: AnyCommandOptions,
		consumer: Consumer,
	): Promise<RSERObject> {
		this.currentCommand = command.name;

		let flags: RSERObject = {};
		if (command.defineFlags !== undefined) {
			flags = command.defineFlags(consumer);
		}

		this.currentCommand = undefined;

		return flags;
	}

	private exit(code: number) {
		if (!this.opts.noProcessExit) {
			process.exit(code);
		}
	}
}

export class ParserInterface<T> {
	constructor(parser: Parser<T>) {
		this.parser = parser;
	}

	private parser: Parser<T>;

	public init(): Promise<T> {
		return this.parser.init();
	}

	public showHelp(): Promise<void> {
		return this.parser.showHelp();
	}

	public getArgs(): Array<string> {
		return this.parser.args;
	}

	public command(opts: AnyCommandOptions) {
		this.parser.addCommand(opts);
	}
}
