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
import {printDiagnostics} from '@romejs/cli-diagnostics';
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
function consumerToLintCompilerOptions(consumer: Consumer): LintCompilerOptions {
  const decisionsByLine: LintCompilerOptions['decisionsByLine'] = {};

  for (const [line, actions] of consumer.get('decisionsByLine').asMap()) {
    decisionsByLine[line] = actions.asArray().map((action) => {
      return {
        action: action.get('action').asStringSet(['suppress', 'fix', 'ignore']),
        category: (action.get('category').asString() as DiagnosticCategory),
      };
    });
  }

  const options: LintCompilerOptions = {
    decisionsByLine,
  };
  return options;
}

export default createLocalCommand<LintCommandFlags>(
  {
    category: commandCategories.CODE_QUALITY,
    description: 'run lint against a set of files',
    allowRequestFlags: ['watch', 'review', 'allowDirty'],
    usage: '',
    examples: [],

    defineFlags(consumer: Consumer): LintCommandFlags {
      // This is never really intended to be specified via CLI arguments but if you really want then you can pass in a JSON string
      const compilerOptionsPerFile: Dict<LintCompilerOptions> = {};
      const compilerOptionsPerFileFlag = consumer.get('compilerOptions').asPossibleParsedJSON();
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

      const res = await client.query({
        ...req.query,
        noData: false,
      }, 'master');

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
            options: {
              ignore: {
                label: 'Do nothing',
                shortcut: 'n',
              },
              fix: {
                disabled: !diag.fixable,
                disabledReason: 'Not fixable',
                label: 'Apply fix',
                shortcut: 'f',
              },
              suppress: {
                label: 'Add suppression comment',
                shortcut: 's',
              },
              stop: {
                label: 'Save all changes and exit',
              },
              exit: {
                label: 'Exit without saving',
                shortcut: 'escape',
              },
            },
          },
        );

        if (answer === 'exit') {
          return false;
        }

        if (answer === 'stop') {
          break;
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

      if (Object.keys(lintOptionsPerFile).length === 0) {
        reporter.warn('No files');
        return false;
      }

      reporter.clearScreen();
      reporter.info('The following changes will be applied');

      for (let filename in lintOptionsPerFile) {
        const decisions = lintOptionsPerFile[filename].decisionsByLine;
        reporter.section(`<filelink target="${filename}" />`, () => {
          if (Object.keys(decisions).length === 0) {
            reporter.info('File will be formatted');
          } else {
            reporter.table(
              ['Line', 'Changes'],
              Object.entries(decisions).map(
                (
                  [line, actions],
                ) => {
                  return [
                      `<dim>${line}</dim>`,
                      actions.map(
                        ({action, category}) => `${action} ${category}`,
                      ).join(', '),
                    ];
                },
              ),
            );
          }
        });
      }

      const confirm = await reporter.radioConfirm(
        'Are you sure you want these to be applied?',
      );
      if (!confirm) {
        return false;
      }

      const res2 = await client.query({
        command: 'lint',
        args: Object.keys(lintOptionsPerFile),
        commandFlags: {
          fix: true,
          compilerOptions: lintOptionsPerFile,
        },
        requestFlags: {
          allowDirty: true,
        },
      }, 'master');
      return res2;
    },
  },
);
