/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ClientFlags,
  ClientRequestFlags,
  DEFAULT_CLIENT_REQUEST_FLAGS,
} from '@romejs/core';
import {
  Client,
  PLATFORMS,
  DEFAULT_CLIENT_FLAGS,
  masterCommands,
  localCommands,
} from '@romejs/core';
import setProcessTitle from './utils/setProcessTitle';
import {parseCLIFlagsFromProcess} from '@romejs/cli-flags';
import {
  createAbsoluteFilePath,
  UnknownFilePath,
  maybeCreateAbsoluteFilePath,
} from '@romejs/path';
import {Consumer} from '@romejs/consume';
import {
  getFilenameTimestamp,
  ClientProfileOptions,
} from '@romejs/core/client/Client';
import {commandCategories} from '@romejs/core/commands';
import {writeFile} from '@romejs/fs';
import fs = require('fs');
import {stripAnsi} from '@romejs/string-ansi';
import {Dict} from '@romejs/typescript-helpers';

type CLIFlags = {
  logs: boolean;
  logWorkers: undefined | boolean;
  logPath: undefined | UnknownFilePath;
  markersPath: undefined | UnknownFilePath;
  rage: boolean;
  ragePath: undefined | UnknownFilePath;
  profile: boolean;
  profilePath: undefined | UnknownFilePath;
  profileTimeout: number;
  profileSampling: number;
  profileWorkers: boolean;
  temporaryDaemon: boolean;
};

export default async function cli() {
  setProcessTitle('cli');

  const p = parseCLIFlagsFromProcess({
    programName: 'rome',
    usage: '[command] [flags]',
    defineFlags(
      c: Consumer,
    ): {
      cliFlags: CLIFlags;
      clientFlags: ClientFlags;
      requestFlags: ClientRequestFlags;
    } {
      return {
        clientFlags: {
          clientName: 'cli',
          cwd: createAbsoluteFilePath(c.get('cwd').asString(process.cwd())),
          verbose: c.get('verbose').asBoolean(DEFAULT_CLIENT_FLAGS.verbose),
          silent: c.get('silent').asBoolean(DEFAULT_CLIENT_FLAGS.silent),
          ...overrideClientFlags,
        },

        cliFlags: {
          markersPath: maybeCreateAbsoluteFilePath(
            c.get('markersPath').asStringOrVoid(),
          ),
          profile: c.get('profile').asBoolean(false),
          profilePath: maybeCreateAbsoluteFilePath(
            c.get('profilePath').asStringOrVoid(),
          ),
          profileTimeout: c.get('profileTimeout').asNumber(0),
          profileWorkers: c.get('profileWorkers').asBoolean(true),
          profileSampling: c.get('profileSampling').asNumber(100),
          temporaryDaemon: c.get('temporaryDaemon').asBoolean(false),
          rage: c.get('rage').asBoolean(false),
          ragePath: maybeCreateAbsoluteFilePath(
            c.get('ragePath').asStringOrVoid(),
          ),
          logs: c.get('logs').asBoolean(false),
          logWorkers: c.get('logWorkers').asBooleanOrVoid(),
          logPath: maybeCreateAbsoluteFilePath(
            c.get('logPath').asStringOrVoid(),
          ),
          ...overrideCLIFlags,
        },

        requestFlags: {
          benchmark: c
            .get('benchmark')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.benchmark),
          benchmarkIterations: c
            .get('benchmarkIterations')
            .asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.benchmarkIterations),
          collectMarkers: c
            .get('collectMarkers')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.collectMarkers),
          watch: c.get('watch').asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.watch),
          fieri: c.get('fieri').asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.fieri),
          focus: c.get('focus').asString(DEFAULT_CLIENT_REQUEST_FLAGS.focus),
          grep: c.get('grep').asString(DEFAULT_CLIENT_REQUEST_FLAGS.grep),
          maxDiagnostics: c
            .get('maxDiagnostics')
            .asNumber(DEFAULT_CLIENT_REQUEST_FLAGS.maxDiagnostics),
          verboseDiagnostics: c
            .get('verboseDiagnostics')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.verboseDiagnostics),
          showAllDiagnostics: c
            .get('showAllDiagnostics')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.showAllDiagnostics),
          inverseGrep: c
            .get('inverseGrep')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.inverseGrep),
          resolverPlatform: c
            .get('resolverPlatform')
            .asStringSetOrVoid(PLATFORMS),
          resolverScale: c.get('resolverScale').asNumberOrVoid(),
          resolverMocks: c
            .get('resolverMocks')
            .asBoolean(DEFAULT_CLIENT_REQUEST_FLAGS.resolverMocks),
          ...overrideRequestFlags,
        },
      };
    },
  });

  let command = '';
  let overrideClientFlags: Partial<ClientFlags> = {};
  let overrideRequestFlags: Partial<ClientRequestFlags> = {};
  let overrideCLIFlags: Partial<CLIFlags> = {};
  let commandFlags: Dict<unknown> = {};
  let args: Array<string> = [];

  // Create command handlers. We use a set here since we may have some conflicting master and local command names. We always want the local command to take precedence.
  const commandNames = new Set([
    ...localCommands.keys(),
    ...masterCommands.keys(),
  ]);
  for (const cmd of commandNames) {
    const local = localCommands.get(cmd);
    if (local !== undefined) {
      p.command({
        name: cmd,
        category: local.category,
        description: local.description,
        defineFlags: local.defineFlags,
        examples: local.examples,
        usage: local.usage,
        callback(_commandFlags) {
          if (local.defineFlags !== undefined) {
            commandFlags = _commandFlags;
          }

          args = p.getArgs();
          command = cmd;
        },
      });
      continue;
    }

    const master = masterCommands.get(cmd);
    if (master !== undefined) {
      p.command({
        name: cmd,
        category: master.category,
        description: master.description,
        defineFlags: master.defineFlags,
        usage: master.usage,
        examples: master.examples,

        callback(_commandFlags) {
          if (master.defineFlags !== undefined) {
            commandFlags = _commandFlags;
          }

          if (master.overrideClientFlags !== undefined) {
            overrideClientFlags = master.overrideClientFlags;
          }

          if (master.overrideRequestFlags !== undefined) {
            overrideRequestFlags = master.overrideRequestFlags;
          }

          args = p.getArgs();
          command = cmd;
        },
      });
    }
  }

  // Mock `rage` command that just uses the master noop command and adds the --rage flag
  p.command({
    name: 'rage',
    category: commandCategories.INTERNAL,
    description: '',

    callback() {
      overrideCLIFlags = {
        rage: true,
      };

      command = '_noop';
    },
  });

  // Mock `logs` command that just uses the master noop command and adds the --logs flag
  p.command({
    name: 'logs',
    category: commandCategories.INTERNAL,
    description: '',

    callback() {
      overrideCLIFlags = {
        logs: true,
      };

      command = '_noop';
    },
  });

  // Initialize flags
  let {clientFlags: clientFlags, cliFlags, requestFlags} = await p.init();

  // Force collection of markers if markersPath or we are raging
  if (cliFlags.markersPath || cliFlags.rage) {
    requestFlags.collectMarkers = true;
  }

  // Force logs when logPath or logWorkers is set
  if (cliFlags.logPath !== undefined || cliFlags.logWorkers === true) {
    cliFlags.logs = true;
  }

  p.commandRequired();

  const client = new Client({
    globalErrorHandlers: true,
    flags: clientFlags,
    stdin: process.stdin,
    stdout: process.stdout,
    stderr: process.stderr,
  });

  client.bridgeAttachedEvent.subscribe(async () => {
    const profileOptions: ClientProfileOptions = {
      samplingInterval: cliFlags.profileSampling,
      timeoutInterval: cliFlags.profileTimeout,
      includeWorkers: cliFlags.profileWorkers,
    };

    if (cliFlags.rage) {
      const {ragePath} = cliFlags;
      const filename = clientFlags.cwd
        .resolve(
          ragePath === undefined
            ? `rome-rage-${getFilenameTimestamp()}.tar.gz`
            : ragePath,
        )
        .join();
      await client.rage(filename, profileOptions);
      return;
    }

    if (cliFlags.profile) {
      await client.profile(profileOptions, async events => {
        const {cwd} = clientFlags;
        const {profilePath} = cliFlags;

        const resolvedProfilePath = cwd.resolve(
          profilePath === undefined
            ? `Profile-${getFilenameTimestamp()}.json`
            : profilePath,
        );

        const str = JSON.stringify(events, undefined, '  ');
        await writeFile(resolvedProfilePath, str);

        client.reporter.success(
          `Wrote CPU profile to <filelink emphasis target="${resolvedProfilePath.join()}" />`,
        );
      });
    }

    if (cliFlags.logs) {
      let fileout: undefined | fs.WriteStream;
      if (cliFlags.logPath !== undefined) {
        fileout = fs.createWriteStream(
          clientFlags.cwd.resolve(cliFlags.logPath).join(),
        );

        client.endEvent.subscribe(() => {
          if (fileout !== undefined) {
            fileout.end();
          }
        });
      }

      await client.subscribeLogs(cliFlags.logWorkers === true, chunk => {
        if (fileout === undefined) {
          client.reporter.writeAll(chunk);
        } else {
          fileout.write(stripAnsi(chunk));
        }
      });
    }
  });

  if (cliFlags.temporaryDaemon) {
    await client.forceStartDaemon();
  }

  const res = await client.query({
    command,
    commandFlags,
    args,
    requestFlags,
    // Daemon would have been started before, so terminate when we complete
    terminateWhenIdle: cliFlags.temporaryDaemon,
    // We don't use the data result, so no point transporting it over the bridge
    noData: true,
  });

  await client.end();

  if (res.type === 'SUCCESS') {
    // Write markers if we were collecting them
    if (requestFlags.collectMarkers) {
      const markersPath = clientFlags.cwd.resolve(
        cliFlags.markersPath === undefined
          ? `Markers-${getFilenameTimestamp()}.json`
          : cliFlags.markersPath,
      );

      await writeFile(markersPath, JSON.stringify(res.markers, null, '  '));

      client.reporter.success(
        `Wrote markers to <filelink emphasis target="${markersPath.join()}" />`,
      );
    }
  }

  switch (res.type) {
    case 'ERROR':
      if (!res.handled) {
        console.error('Unhandled CLI query error');
        console.error(res.stack);
      }
      process.exit(1);
      break;

    case 'INVALID_REQUEST':
      await p.showHelp();
      process.exit(1);
      break;

    case 'DIAGNOSTICS':
      process.exit(1);
      break;

    case 'SUCCESS':
      process.exit(0);
      break;
  }
}
