/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {serializeCLIFlags} from './serializeCLIFlags';
import {
  ConsumePath,
  ConsumePropertyDefinition,
  ConsumeSourceLocationRequestTarget,
  Consumer,
  consume,
} from '@romejs/consume';
import {naturalCompare, toCamelCase, toKebabCase} from '@romejs/string-utils';
import {createUnknownFilePath} from '@romejs/path';
import {Dict} from '@romejs/typescript-helpers';
import {markup} from '@romejs/string-markup';
import {descriptions} from '@romejs/diagnostics';

export type Examples = Array<{
  description: string;
  command: string;
}>;

type CommandOptions<T extends Dict<unknown>> = {
  name: string;
  category?: string;
  description?: string;
  usage?: string;
  examples?: Examples;
  ignoreFlags?: Array<string>;
  defineFlags?: (consumer: Consumer) => T;
  callback: (flags: T) => void | Promise<void>;
};

type AnyCommandOptions = CommandOptions<Dict<unknown>>;

type ArgDeclaration = {
  definition: ConsumePropertyDefinition;
  name: string;
  command: undefined | string;
};

type DefinedCommand = {
  flags: Dict<unknown>;
  command: AnyCommandOptions;
};

export type ParserOptions<T> = {
  examples?: Examples;
  programName: string;
  usage?: string;
  description?: string;
  version?: string;
  ignoreFlags?: Array<string>;
  defineFlags: (consumer: Consumer) => T;
};

function splitCommandName(cmd: string): Array<string> {
  return cmd.split(' ');
}

// Whether we can display this value in help
function isDisplayableHelpValue(value: unknown): value is string | number {
  return typeof value === 'string' || typeof value === 'number';
}

type _FlagValue = undefined | number | string | boolean;

export type FlagValue = _FlagValue | Array<_FlagValue>;

type SupportedAutocompleteShells = 'bash' | 'fish';

export default class Parser<T> {
  constructor(
    reporter: Reporter,
    opts: ParserOptions<T>,
    rawArgs: Array<string>,
  ) {
    this.reporter = reporter;
    this.opts = opts;

    this.shorthandFlags = new Set();
    this.incorrectCaseFlags = new Set();
    this.declaredFlags = new Map();
    this.defaultFlags = new Map();
    this.flags = new Map();
    this.args = [];

    // These are used to track where we should insert an argument for a boolean flag value
    this.flagToArgIndex = new Map();
    this.flagToArgOffset = 0;

    this.consumeRawArgs(rawArgs);

    this.commands = new Map();
    this.ranCommand = undefined;
    this.currentCommand = undefined;
  }

  reporter: Reporter;
  opts: ParserOptions<T>;
  incorrectCaseFlags: Set<string>;
  shorthandFlags: Set<string>;
  flags: Map<string, FlagValue>;
  defaultFlags: Map<string, unknown>;
  declaredFlags: Map<string, ArgDeclaration>;
  flagToArgIndex: Map<string, number>;
  flagToArgOffset: number;
  currentCommand: undefined | string;
  ranCommand: undefined | AnyCommandOptions;
  commands: Map<string, AnyCommandOptions>;
  args: Array<string>;

  looksLikeFlag(flag: undefined | string): boolean {
    return flag?.[0] === '-';
  }

  toCamelCase(name: string): string {
    const camelName = toCamelCase(name);

    // Don't allow passing in straight camelcased names
    if (toKebabCase(name) !== name) {
      this.incorrectCaseFlags.add(name);
    }

    return camelName;
  }

  setFlag(key: string, value: string | boolean) {
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

  consumeRawArgs(rawArgs: Array<string>) {
    while (rawArgs.length > 0) {
      const arg: string = String(rawArgs.shift());

      if (arg === '--') {
        // We consider a -- by itself to halt parsing of args, the rest of the remaining args are added to _
        this.args = this.args.concat(rawArgs);
        break;
      } else if (arg[0] === '-') {
        // Clean the argument by stripping off the dashes
        const name = arg[1] === '-' ? arg.slice(2) : arg.slice(1);

        // Flags beginning with no- are always false
        if (name.startsWith('no-')) {
          const camelName = this.toCamelCase(name.slice(3));
          this.setFlag(camelName, false);
          continue;
        }

        // Allow for arguments to be passed as --foo=bar
        const equalsIndex = name.indexOf('=');
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

        this.flagToArgIndex.set(camelName, this.args.length);

        if (arg[0] === '-' && arg[1] !== '-') {
          this.shorthandFlags.add(camelName);
        }
      } else {
        // Not a flag and hasn't been consumed already by a previous arg so it must be a file
        this.args.push(arg);
      }
    }
  }

  getFlagsConsumer(): Consumer {
    const defaultFlags: Dict<FlagValue> = {};

    const flags: Dict<FlagValue> = {};
    for (const [key, value] of this.flags) {
      flags[toCamelCase(key)] = value;
    }

    return consume({
      filePath: createUnknownFilePath('argv'),
      value: flags,
      onDefinition: (def, valueConsumer) => {
        const key = def.objectPath.join('.');

        // Detect root object
        if (key === '') {
          return;
        }

        const value = flags[key];

        // Allow omitting a string flag value
        if (def.type === 'string' && value === true) {
          valueConsumer.setValue('');
        }

        this.declareArgument({
          name: key,
          command: this.currentCommand,
          definition: def,
        });
        defaultFlags[key] = (def.default as FlagValue);

        // We've parsed arguments like `--foo bar` as `{foo: 'bar}`
        // However, --foo may be a boolean flag, so `bar` needs to be correctly added to args
        if (
          def.type === 'boolean' &&
          value !== true &&
          value !== false &&
          value !== undefined
        ) {
          const argIndex = this.flagToArgIndex.get(key);
          if (argIndex === undefined) {
            throw new Error('No arg index. Should always exist.');
          }

          // Insert the argument at the correct place
          this.args.splice(argIndex + this.flagToArgOffset, 0, String(value));

          // Increase offset to correct subsequent insertions
          this.flagToArgOffset++;

          //
          valueConsumer.setValue(true);
        }
      },
      context: {
        category: 'flags/invalid',
        normalizeKey: (key) => {
          return this.incorrectCaseFlags.has(key) ? key : toKebabCase(key);
        },
        getOriginalValue: (keys: ConsumePath) => {
          return flags[keys[0]];
        },
        getDiagnosticPointer: (
          keys: ConsumePath,
          target: ConsumeSourceLocationRequestTarget,
        ) => {
          const {programName} = this.opts;

          return serializeCLIFlags(
            {
              programName,
              commandName: this.currentCommand,
              args: this.args,
              defaultFlags,
              flags,
              incorrectCaseFlags: this.incorrectCaseFlags,
              shorthandFlags: this.shorthandFlags,
            },
            {
              type: 'flag',
              key: String(keys[0]),
              target,
            },
          );
        },
      },
    });
  }

  hasArg(name: string): boolean {
    return this.flags.has(name) && this.flags.get(name) !== undefined;
  }

  declareArgument(decl: ArgDeclaration) {
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

  getInterface(): ParserInterface<T> {
    return new ParserInterface(this);
  }

  async maybeDefineCommandFlags(
    command: AnyCommandOptions,
    consumer: Consumer,
  ): Promise<undefined | Dict<unknown>> {
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

  checkBadFlags(consumer: Consumer, definedCommand: undefined | DefinedCommand) {
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

    for (const shorthandName of this.shorthandFlags) {
      consumer.get(shorthandName).unexpected(
        descriptions.FLAGS.UNSUPPORTED_SHORTHANDS,
      );
    }

    for (const incorrectName of this.incorrectCaseFlags) {
      consumer.get(incorrectName).unexpected(
        descriptions.FLAGS.INCORRECT_CASED_FLAG(incorrectName),
      );
    }

    consumer.enforceUsedProperties('flag', false);
  }

  async init(): Promise<T> {
    const consumer = this.getFlagsConsumer();

    // Show help for --version
    const version = this.opts.version;
    if (version !== undefined) {
      const shouldDisplayVersion = consumer.get(
        'version',
        {
          description: 'Show the version',
        },
      ).asBoolean(false);
      if (shouldDisplayVersion) {
        this.reporter.logAll(version);
        process.exit(0);
      }
    }

    const generateAutocomplete: undefined | SupportedAutocompleteShells = consumer.get(
      'generateAutocomplete',
      {
        description: 'Generate a shell autocomplete',
      },
    ).asStringSetOrVoid(['fish', 'bash']);
    if (generateAutocomplete !== undefined) {
      await this.generateAutocomplete(generateAutocomplete);
      process.exit(0);
    }

    // Show help for --help
    const shouldShowHelp = consumer.get(
      'help',
      {
        description: 'Show this help screen',
      },
    ).asBoolean(false);

    let definedCommand: undefined | DefinedCommand;

    const rootFlags = await consumer.bufferDiagnostics(async (consumer) => {
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

      if (!shouldShowHelp) {
        this.checkBadFlags(consumer, definedCommand);
      }

      this.currentCommand = undefined;

      return rootFlags;
    });

    // Show help for --help
    if (shouldShowHelp) {
      await this.showHelp(
        definedCommand === undefined ? undefined : definedCommand.command,
      );
      process.exit(1);
    }

    if (definedCommand !== undefined) {
      this.ranCommand = definedCommand.command;
      await definedCommand.command.callback(definedCommand.flags);
    }

    return rootFlags;
  }

  buildOptionsHelp(keys: Array<string>): Array<Array<string>> {
    const optionOutput: Array<{
      argName: string;
      arg: string;
      description: string;
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
      if (def.type === 'boolean' && def.default === true) {
        argCol = `--no-${argCol}`;
        argName = `no-${argName}`;
      } else {
        argCol = `--${argCol}`;
      }

      const {default: defaultValue, allowedValues} = def;

      // Add input specifier unless a boolean
      if (def.type !== 'boolean') {
        // TODO some way to customize this
        // Property metadata in the consumer is a fine place but we want this to be non-CLI specific
        let inputName = undefined;

        if (inputName === undefined) {
          if (def.type === 'number' || def.type === 'number-range') {
            inputName = 'num';
          } else {
            inputName = 'input';
          }
        }

        argCol += ` <${inputName}>`;
      }

      // Set arg col length if we'll be longer
      if (argColumnLength < argCol.length) {
        argColumnLength = argCol.length;
      }

      let descCol: string =
        metadata === undefined || metadata.description === undefined
          ? 'no description found'
          : metadata.description;

      if (defaultValue !== undefined && isDisplayableHelpValue(defaultValue)) {
        descCol += ` (default: ${defaultValue})`;
      }

      if (allowedValues !== undefined) {
        const displayAllowedValues = allowedValues.filter((item) =>
          isDisplayableHelpValue(item)
        );
        if (displayAllowedValues !== undefined) {
          descCol += ` (values: ${displayAllowedValues.join('|')})`;
        }
      }

      optionOutput.push({
        argName,
        arg: markup`<color fg="brightBlack">${argCol}</color>`,
        description: descCol,
      });
    }

    // Sort options by argument name
    optionOutput.sort((a, b) => naturalCompare(a.argName, b.argName));

    // Build table rows
    return optionOutput.map((opt) => [opt.arg, opt.description]);
  }

  showUsageHelp(
    description?: string,
    usage: string = '[flags]',
    prefix?: string,
  ) {
    const {reporter} = this;
    const {programName} = this.opts;

    reporter.section(
      `Usage`,
      () => {
        if (description !== undefined) {
          reporter.logAll(description);
          reporter.br(true);
        }

        const commandParts = [programName];
        if (prefix !== undefined) {
          commandParts.push(prefix);
        }
        commandParts.push(usage);

        const command = commandParts.join(' ');
        reporter.command(command);
      },
    );
  }

  showFocusedCommandHelp(command: AnyCommandOptions) {
    const {reporter} = this;
    const {name, usage, description, examples} = command;

    reporter.br(true);
    this.showUsageHelp(description, usage, name);
    this.showHelpExamples(examples, name);

    // Find arguments that belong to this command
    const argKeys = [];
    for (const [key, decl] of this.declaredFlags) {
      if (decl.command === name) {
        argKeys.push(key);
      }
    }

    const optRows = this.buildOptionsHelp(argKeys);
    if (optRows.length > 0) {
      reporter.section(
        'Command Flags',
        () => {
          reporter.table([], optRows);
        },
      );
    }

    reporter.section(
      'Global Flags',
      () => {
        reporter.info('To view global flags run');
        reporter.command('rome --help');
      },
    );
  }

  showGlobalFlags() {
    const {reporter} = this;
    reporter.section(
      'Global Flags',
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

  async generateAutocomplete(shell: SupportedAutocompleteShells) {
    const {reporter} = this;

    // Execute all command defineFlags. Only one is usually ran when the arguments match the command name.
    // But to generate autocomplete we want all the flags to be declared for all commands.
    const flags = this.getFlagsConsumer();
    for (const command of this.commands.values()) {
      // capture() will cause diagnostics to be suppressed
      const {consumer} = flags.capture();
      await this.defineCommandFlags(command, consumer);
    }

    // this.declaredFlags contains flag information, ignore the keys as they will have command suffixes
    // utilize `command` for the owned command and `name` for the actual flag name

    // this.commands contains command information

    // reporter.logAllNoMarkup to output to stdout
    reporter;

    switch (shell) {
      case 'bash':
        // TODO
        break;

      case 'fish':
        // TODO
        break;
    }
  }

  async showHelp(command: undefined | AnyCommandOptions = this.ranCommand) {
    if (command !== undefined) {
      this.showFocusedCommandHelp(command);
      return;
    }

    const {reporter} = this;
    const {description, usage, examples, programName} = this.opts;

    this.showUsageHelp(description, usage);
    this.showGlobalFlags();

    // Sort commands into their appropriate categories for output
    const commandsByCategory: Map<undefined | string, Array<AnyCommandOptions>> = new Map();
    const categoryNames: Set<string | undefined> = new Set();
    for (const [name, command] of this.commands) {
      if (name[0] === '_') {
        continue;
      }

      const {category} = command;
      let commandsForCategory = commandsByCategory.get(category);
      if (commandsForCategory === undefined) {
        commandsForCategory = [];
        commandsByCategory.set(category, commandsForCategory);
      }
      commandsForCategory.push(command);
      categoryNames.add(category);
    }

    reporter.section(
      'Commands',
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
          const commands = commandsByCategory.get(category)!;

          if (category !== undefined) {
            reporter.logAll(`<emphasis>${category} Commands</emphasis>`);
          }

          // Sort by name
          commands.sort((a, b) => a.name.localeCompare(b.name));

          reporter.list(
            commands.map((cmd) => {
              return `<emphasis>${cmd.name}</emphasis> ${cmd.description ===
              undefined
                ? ''
                : cmd.description}`;
            }),
          );
          reporter.br();
        }

        reporter.info('To view help for a specific command run');
        reporter.command(`${programName} command_name --help`);
      },
    );

    this.showHelpExamples(examples);
  }

  showHelpExamples(examples?: Examples, prefix?: string) {
    const {programName} = this.opts;
    const {reporter} = this;

    if (examples === undefined || examples.length === 0) {
      return;
    }

    reporter.section(
      'Examples',
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

          const builtCommand = commandParts.join(' ');

          reporter.br();
          if (description !== undefined) {
            reporter.logAll(description);
          }
          reporter.command(builtCommand);
        }
      },
    );
  }

  commandRequired() {
    if (this.ranCommand) {
      return;
    }

    if (this.args.length === 0) {
      this.reporter.error(
        'No command specified. Run --help to see available commands.',
      );
    } else {
      // TODO command name is not sanitized for markup
      // TODO produce a diagnostic instead
      this.reporter.error(
        `Unknown command <emphasis>${this.args.join(' ')}</emphasis>. Run --help to see available commands.`,
      );
    }

    process.exit(1);
  }

  addCommand(opts: AnyCommandOptions) {
    if (this.currentCommand !== undefined) {
      throw new Error("Nested commands aren't allowed");
    }

    this.commands.set(opts.name, opts);
  }

  async defineCommandFlags(
    command: AnyCommandOptions,
    consumer: Consumer,
  ): Promise<Dict<unknown>> {
    this.currentCommand = command.name;

    let flags: Dict<unknown> = {};
    if (command.defineFlags !== undefined) {
      flags = command.defineFlags(consumer);
    }

    this.currentCommand = undefined;

    return flags;
  }
}

export class ParserInterface<T> {
  constructor(parser: Parser<T>) {
    this.parser = parser;
  }

  parser: Parser<T>;

  init(): Promise<T> {
    return this.parser.init();
  }

  showHelp(): Promise<void> {
    return this.parser.showHelp();
  }

  getArgs(): Array<string> {
    return this.parser.args;
  }

  commandRequired() {
    this.parser.commandRequired();
  }

  command(opts: AnyCommandOptions) {
    this.parser.addCommand(opts);
  }
}
