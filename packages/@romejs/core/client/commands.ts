/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeUnknown} from '@romejs/consume';
import ClientRequest from './ClientRequest';
import {LocalCommand} from '../commands';
import {commandCategories} from '../commands';
import executeMain from '../common/utils/executeMain';
import {DiagnosticsError} from '@romejs/diagnostics';
import {createAbsoluteFilePath} from '@romejs/path';

export const localCommands: Map<string, LocalCommand> = new Map();

localCommands.set('start', {
  category: commandCategories.PROCESS_MANAGEMENT,
  description: 'start daemon (if none running)',
  async callback(req: ClientRequest) {
    const existingServer = await req.client.tryConnectToExistingDaemon();
    if (existingServer) {
      req.client.reporter.success('Already running server.');
      return true;
    }

    const bridge = await req.client.startDaemon();
    return bridge !== undefined;
  },
});

localCommands.set('develop', {
  category: commandCategories.PROCESS_MANAGEMENT,
  description: '',
  async callback(req: ClientRequest) {
    const existingServer = await req.client.tryConnectToExistingDaemon();
    const hasExistingServer = existingServer !== undefined;

    if (!hasExistingServer) {
      await req.client.forceStartDaemon();
    }

    await req.client.query(
      {
        ...req.query,
        terminateWhenIdle: true,
      },
      'master',
    );

    return true;
  },
});

localCommands.set('stop', {
  category: commandCategories.PROCESS_MANAGEMENT,
  description: 'stop a running daemon if one exists',
  async callback(req: ClientRequest) {
    // We might want to use `terminateWhenIdle` here combined with a timeout instead of forcing it to die straight away
    const {reporter} = req.client;
    const bridge = await req.client.tryConnectToExistingDaemon();
    if (bridge) {
      const stop = await req.client.query(
        {
          command: 'stop',
        },
        'master',
      );
      if (stop.type === 'ERROR' && stop.fatal) {
        reporter.success('Stopped server.');
      } else {
        reporter.error('Failed to stop server.');
        return false;
      }
    } else {
      reporter.warn('No running server to stop.');
    }
    return true;
  },
});

localCommands.set('run', {
  category: commandCategories.PROJECT_MANAGEMENT,
  description: 'TODO',
  async callback(req: ClientRequest) {
    const bridge = await req.client.findOrStartMaster();
    if (bridge === undefined) {
      return false;
    }

    process.on('unhandledRejection', error => {
      error;
      //console.log('unhandledRejection', error.stack);
    });

    const res = await req.client.query(
      {
        command: 'run',
        args: req.query.args,
      },
      'master',
    );

    if (res.type !== 'SUCCESS') {
      return false;
    }

    const data = consumeUnknown(res.data);

    if (data.exists()) {
      const type = data.get('type').asString();

      switch (type) {
        case 'executeCode':
          process.execArgv = [...process.execArgv, process.argv[1], 'run'];
          process.argv = [
            process.argv[0],
            String(data.filename),
            ...process.argv.slice(4),
          ];
          const {syntaxError} = await executeMain({
            path: createAbsoluteFilePath(data.get('filename').asString()),
            code: data.get('code').asString(),
            sourceMap: data.get('map').asAny(),
          });
          if (syntaxError !== undefined) {
            throw new DiagnosticsError(syntaxError.message, [syntaxError]);
          }
          await new Promise(() => {});
          break;
      }
    }

    return true;
  },
});

localCommands.set('restart', {
  category: commandCategories.PROCESS_MANAGEMENT,
  description: 'restart daemon',
  async callback(req: ClientRequest) {
    const stopped = await req.client.query({
      command: 'stop',
    });

    if (stopped.type === 'SUCCESS' && stopped.data === true) {
      const started = await req.client.query({
        command: 'start',
      });
      return started.type === 'SUCCESS' && started.data === true;
    } else {
      return false;
    }
  },
});

localCommands.set('status', {
  description: 'get the current daemon status',
  category: commandCategories.PROCESS_MANAGEMENT,
  async callback(req: ClientRequest) {
    const {reporter} = req.client;
    const bridge = await req.client.tryConnectToExistingDaemon();
    if (bridge) {
      const status = await req.client.query(
        {
          command: 'status',
        },
        'master',
      );
      if (status.type === 'SUCCESS') {
        reporter.inspect(status.data);
        return true;
      } else {
        return false;
      }
    } else {
      reporter.error('Server not running.');
      return false;
    }
  },
});
