/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {naturalCompare} from '@romejs/string-utils';
import {rightPad} from '@romejs/string-ansi';
import {serializeCLIFlags} from './serializeCLIFlags';
import {
  consume,
  Consumer,
  ConsumePath,
  ConsumePropertyDefinition,
  ConsumeSourceLocationRequestTarget,
} from '@romejs/consume';
import {toKebabCase, toCamelCase} from '@romejs/string-utils';
import {DiagnosticsError} from '@romejs/diagnostics';
import {createUnknownFilePath} from '@romejs/path';
import {Dict} from '@romejs/typescript-helpers';
import {escapeMarkup} from '@romejs/string-markup';

type CommandOptions<T extends Dict<unknown>> = {
  name: string;
  category?: string;
  description?: string;
  defineFlags?: (consumer: Consumer) => T;
  callback: (flags: T) => void | Promise<void>;
};

type ArgDeclaration = {
  definition: ConsumePropertyDefinition;
  name: string;
  command: undefined | string;
};

type DefinedCommand = {
  flags: Dict<unknown>;
  command: CommandOptions<Dict<unknown>>;
};

export type ParserOptions<T> = {
  examples?: Array<string>;
  programName?: string;
  usage?: string;
  description?: string;
  version?: string;
  defineFlags: (consumer: Consumer) => T;
};

function splitCommandName(cmd: string): Array<string> {
  return cmd.split(' ');
}

export default class Parser<T> {
  constructor(
    reporter: Reporter,
    opts: ParserOptions<T>,
    rawArgs: Array<string>,
  ) {
    this.reporter = reporter;
    this.opts = opts;

    this.shorthandFlags = new Set();
    this.declaredFlags = new Map();
    this.defaultFlags = new Map();
    this.flags = new Map();
    this.args = [];

    this.consumeRawArgs(rawArgs);

    this.commands = new Map();
    this.ranCommand = false;
    this.currentCommand = undefined;

    if (opts.version !== undefined) {
      this.declareArgument({
        command: undefined,
        name: 'version',
        definition: {
          type: 'boolean',
          objectPath: ['version'],
          default: false,
          required: false,
          metadata: {
            description: 'show the version',
          },
        },
      });
    }

    this.declareArgument({
      command: undefined,
      name: 'help',
      definition: {
        type: 'boolean',
        objectPath: ['help'],
        default: false,
        required: false,
        metadata: {
          description: 'show this help screen',
        },
      },
    });

    this.helpMode = this.flags.has('help');
  }

  reporter: Reporter;
  opts: ParserOptions<T>;

  shorthandFlags: Set<string>;
  flags: Map<string, string | boolean>;
  defaultFlags: Map<string, unknown>;
  declaredFlags: Map<string, ArgDeclaration>;

  ranCommand: boolean;
  commands: Map<string, CommandOptions<Dict<unknown>>>;
  args: Array<string>;

  currentCommand: undefined | string;
  helpMode: boolean;

  looksLikeFlag(flag: undefined | string): boolean {
    return flag !== undefined && flag[0] === '-';
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
          this.flags.set(name.slice(3), false);
          continue;
        }

        // If the next argument is a flag or we're at the end of the args then just set it to `true`
        if (rawArgs.length === 0 || this.looksLikeFlag(rawArgs[0])) {
          this.flags.set(name, true);
        } else {
          // Otherwise, take that value
          this.flags.set(name, String(rawArgs.shift()));
        }

        if (arg[0] === '-' && arg[1] !== '-') {
          this.shorthandFlags.add(name);
        }
      } else {
        // Not a flag and hasn't been consumed already by a previous arg so it must be a file
        this.args.push(arg);
      }
    }
  }

  setFlagAlias(key: string, alias: string) {
    const value = this.flags.get(key);
    if (value !== undefined) {
      this.flags.delete(key);
      this.flags.set(alias, value);
    }
  }

  getFlagsConsumer(): Consumer {
    const defaultFlags: Dict<unknown> = {};

    const flags: Dict<unknown> = {};
    for (const [key, value] of this.flags) {
      flags[toCamelCase(key)] = value;
    }

    return consume({
      filePath: createUnknownFilePath('argv'),
      value: flags,

      onDefinition: def => {
        const key = def.objectPath.join('.');

        // Detect root object
        if (key === '') {
          return;
        }

        this.declareArgument({
          name: key,
          command: this.currentCommand,
          definition: def,
        });
        defaultFlags[key] = def.default;
      },

      context: {
        category: 'flags/invalid',
        getOriginalValue: (keys: ConsumePath) => {
          return flags[keys[0]];
        },
        getDiagnosticPointer: (
          keys: ConsumePath,
          target: ConsumeSourceLocationRequestTarget,
        ) => {
          let prefixParts = [];

          const {programName} = this.opts;
          if (programName !== undefined) {
            prefixParts.push(programName);
          }
          if (this.currentCommand !== undefined) {
            prefixParts.push(this.currentCommand);
          }

          return serializeCLIFlags(
            {
              prefix: prefixParts.join(' '),
              args: this.args,
              defaultFlags,
              flags,
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
    this.setFlagAlias(toKebabCase(key), key);
    this.declaredFlags.set(key, decl);
    this.defaultFlags.set(key, decl.definition.default);
  }

  getInterface(): ParserInterface<T> {
    return new ParserInterface(this);
  }

  async shouldRunCommand(
    commandName: string,
    consumer: Consumer,
  ): Promise<undefined | DefinedCommand> {
    // A command name could be made of multiple strings
    const commandParts = splitCommandName(commandName);
    for (let i = 0; i < commandParts.length; i++) {
      if (commandParts[i] !== this.args[i]) {
        return;
      }
    }

    // Remove command name from arguments
    this.args = this.args.slice(commandParts.length);
    return await this.defineCommandFlags(commandName, consumer);
  }

  async init(): Promise<T> {
    // Show help for --help
    if (this.helpMode) {
      await this.showHelp();
      process.exit(1);
    }

    // Show help for --version
    if (this.flags.has('version')) {
      this.reporter.logAll(String(this.opts.version));
      process.exit(0);
    }

    // We've parsed arguments like `--foo bar` as `{foo: 'bar}`
    // However, --foo may be a boolean flag, so `bar` needs to be correctly added to args
    for (const [key, value] of this.flags) {
      const declared = this.declaredFlags.get(key);

      if (
        declared !== undefined &&
        declared.definition.type === 'boolean' &&
        value !== true &&
        value !== false
      ) {
        // This isn't necessarily the correct position... Probably doesn't matter?
        this.args.push(value);

        //
        this.flags.set(key, true);
      }
    }

    const consumer = this.getFlagsConsumer();

    let definedCommand: undefined | DefinedCommand;

    const {diagnostics, result} = await consumer.capture(async consumer => {
      for (const shorthandName of this.shorthandFlags) {
        consumer
          .get(shorthandName)
          .unexpected(`Shorthand flags are not supported`);
      }

      const result = this.opts.defineFlags(consumer);

      for (const key of this.commands.keys()) {
        const defined = await this.shouldRunCommand(key, consumer);
        if (defined) {
          this.currentCommand = key;
          definedCommand = defined;
          break;
        }
      }

      consumer.enforceUsedProperties('flag', false);
      this.currentCommand = undefined;

      return result;
    });

    if (result === undefined) {
      throw new DiagnosticsError('CLI flag parsing diagnostics', diagnostics);
    }

    if (definedCommand !== undefined) {
      this.ranCommand = true;
      await definedCommand.command.callback(definedCommand.flags);
    }

    return result;
  }

  buildOptionsHelp(keys: Array<string>): Array<string> {
    const lines = [];

    const optionOutput: Array<{
      argName: string;
      arg: string;
      description: string;
    }> = [];
    let argColumnLength: number = 0;

    // Build up options, we need to do this to line up the columns correctly
    for (const key of keys) {
      const decl = this.declaredFlags.get(key);
      if (decl === undefined) {
        throw new Error('Expected argument declaration');
      }

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

      const descCol: string =
        metadata === undefined || metadata.description === undefined
          ? 'no description found'
          : metadata.description;

      optionOutput.push({
        argName,
        arg: argCol,
        description: descCol,
      });
    }

    // Sort options by argument name
    optionOutput.sort((a, b) => naturalCompare(a.argName, b.argName));

    // Output options
    for (const {arg, description} of optionOutput) {
      lines.push(
        `  <brightBlack>${rightPad(
          escapeMarkup(arg),
          argColumnLength,
          ' ',
        )}</brightBlack>  ${escapeMarkup(description)}`,
      );
    }

    return lines;
  }

  showCommandsHelp(heading: string, rawCommandNames: Array<string>) {
    if (rawCommandNames.length === 0) {
      return undefined;
    }

    const {reporter} = this;
    reporter.spacer();
    reporter.logAll(`<emphasis>${heading}</emphasis>`);
    reporter.spacer();

    // Build output
    const commandNames: Array<string> = rawCommandNames.sort();
    const cmdOutput: Array<[string, string, Array<string>]> = [];
    let nameColumnLength: number = 0;
    for (const cmd of commandNames) {
      const opts = this.commands.get(cmd);
      if (opts === undefined) {
        throw new Error('Expected command options');
      }

      if (cmd[0] === '_') {
        continue;
      }

      // Set arg col length if we'll be longer
      if (nameColumnLength < cmd.length) {
        nameColumnLength = cmd.length;
      }

      // Find arguments that belong to this command
      const argKeys = [];
      for (const [key, decl] of this.declaredFlags) {
        if (decl.command === cmd) {
          argKeys.push(key);
        }
      }

      const desc =
        opts.description === undefined
          ? 'no description available'
          : opts.description;
      const optLines = this.buildOptionsHelp(argKeys);
      cmdOutput.push([cmd, desc, optLines]);
    }

    for (const [nameCol, descCol, optLines] of cmdOutput) {
      reporter.logAll(
        `  <brightBlack>${rightPad(
          nameCol,
          nameColumnLength,
          ' ',
        )}</brightBlack>  ${descCol}`,
      );

      reporter.indent();
      for (const line of optLines) {
        reporter.logAll(line);
      }
      reporter.dedent();
    }
  }

  async showHelp() {
    const {description, usage, programName, examples} = this.opts;

    const consumer = this.getFlagsConsumer();
    await this.opts.defineFlags(consumer);

    for (const key of this.commands.keys()) {
      await this.defineCommandFlags(key, consumer);
    }

    const {reporter} = this;
    reporter.indent();

    reporter.spacer();
    reporter.logAll(
      `<emphasis>Usage:</emphasis> ${programName} ${
        usage === undefined ? '[flags]' : usage
      }`,
    );
    reporter.spacer();

    if (description !== undefined) {
      reporter.logAll(description);
      reporter.spacer();
    }

    reporter.logAll('<emphasis>Options</emphasis>');
    reporter.spacer();

    // Show options not attached to any commands
    const lonerArgKeys = [];
    for (const [key, decl] of this.declaredFlags) {
      if (decl.command === undefined) {
        lonerArgKeys.push(key);
      }
    }
    for (const line of this.buildOptionsHelp(lonerArgKeys)) {
      reporter.logAll(line);
    }

    // Sort commands into their appropriate categories for output
    const commandNames = new Set(this.commands.keys());
    const commandsByCategory: Map<string, Array<string>> = new Map();
    for (const name of commandNames) {
      const command = this.commands.get(name);
      if (command === undefined) {
        throw new Error('Expected command');
      }

      const {category} = command;
      if (category === undefined) {
        continue;
      }

      let categoryNames = commandsByCategory.get(category);
      if (categoryNames === undefined) {
        categoryNames = [];
        commandsByCategory.set(category, categoryNames);
      }
      categoryNames.push(name);

      commandNames.delete(name);
    }

    // Display commands by category
    const categoryNames = Array.from(commandsByCategory.keys()).sort();
    for (const category of categoryNames) {
      const commandNames = commandsByCategory.get(category);
      if (commandNames === undefined) {
        throw new Error('Expected command names');
      }
      this.showCommandsHelp(`${category} Commands`, commandNames);
    }

    // Display rest of the commands
    this.showCommandsHelp(
      commandsByCategory.size > 0 ? 'Other Commands' : 'Commands',
      Array.from(commandNames),
    );

    // Output examples
    if (examples !== undefined) {
      reporter.spacer();
      reporter.logAll('<emphasis>Examples:</emphasis>');
      reporter.spacer();

      reporter.indent();
      for (const cmd of examples) {
        reporter.command(cmd);
      }
      reporter.dedent();
    }

    reporter.spacer();
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
        `Unknown command <emphasis>${this.args.join(
          ' ',
        )}</emphasis>. Run --help to see available commands.`,
      );
    }

    process.exit(1);
  }

  addCommand(opts: CommandOptions<Dict<unknown>>) {
    if (this.currentCommand !== undefined) {
      throw new Error("Nested commands aren't allowed");
    }

    this.commands.set(opts.name, opts);
  }

  async defineCommandFlags(
    cmd: string,
    consumer: Consumer,
  ): Promise<DefinedCommand> {
    const opts = this.commands.get(cmd);
    if (opts === undefined) {
      throw new Error('Expected options');
    }

    this.currentCommand = cmd;

    let flags: Dict<unknown> = {};
    if (opts.defineFlags !== undefined) {
      flags = opts.defineFlags(consumer);
    }

    this.currentCommand = undefined;

    return {flags, command: opts};
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

  getArgs(): Array<string> {
    return this.parser.args;
  }

  commandRequired() {
    this.parser.commandRequired();
  }

  command(opts: CommandOptions<Dict<unknown>>) {
    this.parser.addCommand(opts);
  }
}
