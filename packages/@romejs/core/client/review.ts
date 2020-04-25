/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import ClientRequest from './ClientRequest';
import {printDiagnostics} from '@romejs/cli-diagnostics';
import {SelectOption} from '@romejs/cli-reporter';
import {DiagnosticAdviceAction} from '@romejs/diagnostics';
import {MasterQueryResponse} from '../common/bridges/MasterBridge';
import {ClientRequestFlags} from '../common/types/client';
import {localCommands} from './commands';
import {masterCommands} from '../master/commands';

export default async function review(
  req: ClientRequest,
): Promise<MasterQueryResponse> {
  const {client} = req;

  const res = await req.fork({
    ...req.query,
    // We want data no matter what
    noData: false,
  }).initCommand();

  const {requestFlags} = req.query;
  if (requestFlags === undefined || !requestFlags.review) {
    return res;
  }

  if (res.type === 'SUCCESS') {
    throw new Error('Expected diagnostics or an error');
  }

  // In case it returned an error
  if (res.type !== 'DIAGNOSTICS') {
    return res;
  }

  const {reporter} = client;

  for (let diag of res.diagnostics) {
    const {filename, start} = diag.location;
    if (filename === undefined || start === undefined) {
      continue;
    }

    let {category, advice = []} = diag.description;
    if (category === 'lint/pendingFixes') {
      continue;
    }

    reporter.clearScreen();

    // Extract actions and remove them from the diagnostic
    const actions: Array<DiagnosticAdviceAction> = [];
    for (const item of advice) {
      if (item.type === 'action') {
        actions.push(item);
      }
    }
    advice = advice.filter((item) => item.type !== 'action');
    diag = {
      ...diag,
      description: {
        ...diag.description,
        advice,
      },
    };

    const optionToAction: Map<string, DiagnosticAdviceAction> = new Map();

    const optionsWithIgnoreAndActions: {
      ignore: SelectOption;
      [key: string]: SelectOption;
    } = {
      ignore: {
        label: 'Do nothing',
        shortcut: 'n',
      },
    };

    let counter = 0;
    for (const action of actions) {
      const key = String(counter++);
      optionToAction.set(key, action);
      optionsWithIgnoreAndActions[key] = {
        label: action.noun,
      };
    }

    const options: {
      exit: SelectOption;
    } = {
      ...optionsWithIgnoreAndActions,
      exit: {
        label: 'Exit',
        shortcut: 'escape',
      },
    };

    printDiagnostics({
      diagnostics: [diag],
      suppressions: [],
      excludeFooter: true,
      printerOptions: {
        reporter,
      },
    });

    const answer = await reporter.radio('How do you want to resolve this?', {
      options,
    });

    if (answer === 'exit') {
      break;
    }

    const action = optionToAction.get(answer);
    if (action === undefined) {
      throw new Error();
    }

    const requestFlags: Partial<ClientRequestFlags> = {
      ...action.requestFlags,
    };

    // If this command allows the allowDirty flag then set it
    // The validation of the flag would have happened with the initial query
    // Also pretty sure the presence of `commandDef === undefined` is an error
    const commandDef = localCommands.get(action.command) || masterCommands.get(
      action.command,
    );
    if (commandDef !== undefined && commandDef.allowRequestFlags !== undefined &&
        commandDef.allowRequestFlags.includes('allowDirty')) {
      requestFlags.allowDirty = true;
    }

    const res = await client.query({
      command: action.command,
      args: action.args,
      commandFlags: action.commandFlags,
      requestFlags,
    }, 'master');
    if (res.type === 'ERROR' || res.type === 'INVALID_REQUEST') {
      return res;
    }
  }

  return {
    type: 'SUCCESS',
    data: undefined,
    hasData: false,
    markers: [],
  };
}
