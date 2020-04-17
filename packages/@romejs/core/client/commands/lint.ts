/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLocalCommand} from '../commands';
import ClientRequest from '../ClientRequest';
import {Dict} from '@romejs/typescript-helpers';
import {
  LintCompilerOptions,
  LintCompilerOptionsDecision,
} from '@romejs/js-compiler';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {get1} from '@romejs/ob1';
import {commandCategories} from '@romejs/core/common/commands';
import {Consumer} from '@romejs/consume';
import {DiagnosticCategory} from '@romejs/diagnostics';
import {LinterOptions} from '@romejs/core/master/linter/Linter';

export type LintCommandFlags = {
  compilerOptionsPerFile: LinterOptions['compilerOptionsPerFile'];
  fix: boolean;
  changed: undefined | string;
};
function consumerToLintCompilerOptions(
  consumer: Consumer,
): LintCompilerOptions {
  const decisionsByLine: LintCompilerOptions['decisionsByLine'] = {};

  for (const [line, actions] of consumer.get('decisionsByLine').asMap()) {
    decisionsByLine[line] = actions.asArray().map((action) => {
      return {
        action: action.get('action').asStringSet(['suppress', 'fix']),
        category: action.get('category').asString() as DiagnosticCategory,
      };
    });
  }

  const options: LintCompilerOptions = {
    decisionsByLine,
  };
  return options;
}

export default createLocalCommand<LintCommandFlags>({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint against a set of files',
  allowRequestFlags: ['watch', 'review'],
  usage: '',
  examples: [],

  defineFlags(consumer: Consumer): LintCommandFlags {
    // This is never really intended to be specified via CLI arguments but if you really want then you can pass in a JSON string
    const compilerOptionsPerFile: Dict<LintCompilerOptions> = {};
    const compilerOptionsPerFileFlag = consumer
      .get('compilerOptions')
      .asPossibleParsedJSON();
    if (compilerOptionsPerFileFlag.exists()) {
      for (const [filename, prop] of compilerOptionsPerFileFlag.asMap()) {
        compilerOptionsPerFile[filename] = consumerToLintCompilerOptions(prop);
      }
    }

    return {
      compilerOptionsPerFile,
      fix: consumer.get('fix').asBoolean(false),
      changed: consumer.get('changed').asStringOrVoid(),
    };
  },

  async callback(req: ClientRequest) {
    const {client} = req;

    const res = await client.query(req.query, 'master');

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

    const lintOptionsPerFile: Dict<Required<LintCompilerOptions>> = {};

    for (const diag of res.diagnostics) {
      const {filename, start} = diag.location;
      if (filename === undefined || start === undefined) {
        continue;
      }

      let options = lintOptionsPerFile[filename];
      if (options === undefined) {
        options = {
          decisionsByLine: {},
        };
        lintOptionsPerFile[filename] = options;
      }

      const {category} = diag.description;
      if (category === 'lint/pendingFixes') {
        continue;
      }

      reporter.clearScreen();

      // Print the diagnostic, fetch it's markup, then format it in the master, and print it out
      const printer = new DiagnosticsPrinter({
        reporter,
      });
      printer.addDiagnostic(diag);
      printer.print();

      const answer = await reporter.radio('How do you want to resolve this?', {
        options: {
          ignore: {
            label: 'Ignore',
            shortcut: 'i',
          },
          fix: {
            disabled: !diag.fixable,
            disabledReason: 'Not fixable',
            label: 'Fix',
            shortcut: 'f',
          },
          suppress: {
            label: 'Suppress',
            shortcut: 's',
          },
        },
      });
      if (answer === 'ignore') {
        continue;
      }

      const action: LintCompilerOptionsDecision['action'] = answer;

      let decisionsByLine = options.decisionsByLine[get1(start.line)];
      if (decisionsByLine === undefined) {
        decisionsByLine = [];
        options.decisionsByLine[get1(start.line)] = decisionsByLine;
      }
      decisionsByLine.push({
        action,
        category,
      });
    }

    reporter.clearScreen();
    reporter.info('The following changes will be applied');

    for (let filename in lintOptionsPerFile) {
      const decisions = lintOptionsPerFile[filename].decisionsByLine;
      reporter.section(`<filelink target="${filename}" />`, () => {
        reporter.table(
          ['Line', 'Decisions'],
          Object.entries(decisions).map(([line, actions]) => {
            return [
              `<dim>${line}</dim>`,
              actions
                .map(({action, category}) => `${action} ${category}`)
                .join(', '),
            ];
          }),
        );
      });
    }

    const confirm = await reporter.radioConfirm(
      'Are you sure you want these to be applied?',
    );
    if (!confirm) {
      return false;
    }

    const res2 = await client.query(
      {
        command: 'lint',
        args: Object.keys(lintOptionsPerFile),
        commandFlags: {
          fix: true,
          compilerOptions: lintOptionsPerFile,
        },
      },
      'master',
    );
    console.log(JSON.stringify(res2, null, '  '));
    return res2;
  },
});
