/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import ClientRequest from './ClientRequest';
import {printDiagnostics} from '@romejs/cli-diagnostics';
import {SelectOption} from '@romejs/cli-reporter';
import {
  Diagnostic,
  DiagnosticAdviceAction,
  derivePositionlessKeyFromDiagnostic,
} from '@romejs/diagnostics';
import {MasterQueryResponse} from '../common/bridges/MasterBridge';
import {ClientRequestFlags} from '../common/types/client';
import {Dict} from '@romejs/typescript-helpers';
import {EMPTY_SUCCESS_RESPONSE} from '../master/MasterRequest';

type State = {
  initial: boolean;
  seen: Set<string>;
  resolvedCount: number;
};

async function check(
  req: ClientRequest,
  state: State,
): Promise<MasterQueryResponse> {
  const {reporter} = req.client;

  reporter.clearScreen();

  if (state.initial) {
    reporter.info('Fetching initial diagnostics');
    state.initial = false;
  } else {
    reporter.info('Updating diagnostics');
  }

  const res = await req.fork({
    ...req.query,
    // We want data no matter what
    noData: false,
  }).initCommand();

  if (res.type === 'SUCCESS') {
    throw new Error('Expected diagnostics or an error');
  }

  // In case it returned an error
  if (res.type !== 'DIAGNOSTICS') {
    return res;
  }

  const diagnostics = res.diagnostics;
  let diag: undefined | Diagnostic;

  for (const _diag of diagnostics) {
    const key = derivePositionlessKeyFromDiagnostic(_diag);
    if (!state.seen.has(key)) {
      state.seen.add(key);
      diag = _diag;
      break;
    }
  }

  if (diag === undefined) {
    return res;
  }

  return await ask(diag, req, state, false);
}

async function ask(
  diag: Diagnostic,
  req: ClientRequest,
  state: State,
  more: boolean,
): Promise<MasterQueryResponse> {
  const {client} = req;
  const {reporter} = client;
  reporter.clearScreen();

  // Extract actions and remove them from the diagnostic
  let {advice = []} = diag.description;
  let hasExtraOptions = false;
  const actions: Array<DiagnosticAdviceAction> = [];
  for (const item of advice) {
    if (item.type === 'action') {
      if (item.extra === true && !more) {
        hasExtraOptions = true;
        continue;
      }

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
  const chosenShortcuts: Set<string> = new Set(['n', 'escape']);

  const actionOptions: Dict<SelectOption> = {};

  let counter = 0;
  for (const action of actions) {
    const key = String(counter++);
    let shortcut =
      action.shortcut !== undefined && !chosenShortcuts.has(action.shortcut)
        ? action.shortcut
        : undefined;
    optionToAction.set(key, action);
    actionOptions[key] = {
      label: action.noun,
      shortcut,
    };
  }

  const options: {
    ignore: SelectOption;
    exit: SelectOption;
    more?: SelectOption;
  } = {
    ignore: {
      label: 'Do nothing',
      shortcut: 'n',
    },
    ...actionOptions,
    exit: {
      label: 'Exit',
      shortcut: 'escape',
    },
  };

  if (!more && hasExtraOptions) {
    options.more = {
      label: 'More options...',
      shortcut: 'm',
    };
  }

  printDiagnostics({
    diagnostics: [diag],
    suppressions: [],
    excludeFooter: true,
    printerOptions: {
      reporter,
    },
  });

  const answer = await reporter.radio(
    'How do you want to resolve this?',
    {
      options,
    },
  );

  if (answer === 'more') {
    return await ask(diag, req, state, true);
  }

  if (answer === 'ignore') {
    return await check(req, state);
  }

  if (answer === 'exit') {
    return EMPTY_SUCCESS_RESPONSE;
  }

  const action = optionToAction.get(answer);
  if (action === undefined) {
    throw new Error('Should have found an action for this option');
  }

  const requestFlags: Partial<ClientRequestFlags> = {
    ...action.requestFlags,
  };

  // Execute action
  const actionRes = await client.query(
    {
      commandName: action.command,
      args: action.args,
      commandFlags: action.commandFlags,
      requestFlags,
    },
    'master',
  );
  if (actionRes.type !== 'DIAGNOSTICS' && actionRes.type !== 'SUCCESS') {
    return actionRes;
  }

  state.resolvedCount++;
  return await check(req, state);
}

export default async function review(
  req: ClientRequest,
): Promise<MasterQueryResponse> {
  const {reporter} = req.client;
  const state: State = {
    initial: true,
    seen: new Set(),
    resolvedCount: 0,
  };
  const res = await check(req, state);

  reporter.clearScreen();

  if (state.seen.size === 0) {
    reporter.success('Nothing to review!');
  } else {
    if (res.type === 'DIAGNOSTICS') {
      printDiagnostics({
        diagnostics: res.diagnostics,
        suppressions: [],
        excludeFooter: true,
        printerOptions: {
          reporter,
        },
      });
      reporter.hr();
      reporter.error(
        `<number emphasis>${res.diagnostics.length}</number> unresolved <grammarNumber plural="issues" singular="issue">${res.diagnostics.length}</grammarNumber> remaining`,
      );
    }

    reporter.success(
      `<number emphasis>${state.resolvedCount}</number> <grammarNumber plural="issues" singular="issue">${state.resolvedCount}</grammarNumber> resolved`,
    );
  }

  return res;
}
