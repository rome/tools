/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand, commandCategories} from '../../commands';

import {modifyProjectConfig, assertHardMeta} from '@romejs/project';
import {createUnknownFilePath} from '@romejs/path';

export default createMasterCommand(
  {
    category: commandCategories.PROJECT_MANAGEMENT,
    description: 'Modify a project config',

    usage: '(enable|disable|enable-category|disable-category|set) key [value]',

    examples: [
      {
        command: 'enable-category lint',
        description: 'Enable linting',
      },
      {
        command: 'set name my_awesome_project',
        description: 'Set the project name',
      },
    ],

    async default(req: MasterRequest): Promise<void> {
      const {reporter} = req;
      req.expectArgumentLength(2, 3);

      const project = await req.assertClientCwdProject();

      let keyParts: string;
      let value: boolean | string;

      const [action, ...restArgs] = req.query.args;
      switch (action) {
        case 'enable': {
          req.expectArgumentLength(2);
          keyParts = req.query.args[1];
          value = true;
          break;
        }

        case 'disable': {
          req.expectArgumentLength(2);
          keyParts = req.query.args[1];
          value = false;
          break;
        }

        case 'enable-category': {
          req.expectArgumentLength(2);
          const category = req.query.args[1];
          keyParts = `${category}.enabled`;
          value = true;
          break;
        }

        case 'disable-category': {
          req.expectArgumentLength(2);
          const category = req.query.args[1];
          keyParts = `${category}.enabled`;
          value = false;
          break;
        }

        case 'set-directory': {
          req.expectArgumentLength(3);
          [keyParts, value] = restArgs;

          // If the value is an absolute path, then make it relative to the project folder
          const path = createUnknownFilePath(value);
          if (path.isAbsolute()) {
              value =
              assertHardMeta(project.meta).projectFolder.relative(path).join();
          }

          break;
        }

        case 'set': {
          req.expectArgumentLength(3);
          [keyParts, value] = restArgs;
          break;
        }

        default:
          throw req.throwDiagnosticFlagError(`Unknown action ${action}`, {
            type: 'arg',
            key: 0,
          });
      }

      try {
        await modifyProjectConfig(
          project.meta,
          {
            pre: (meta) => {
              reporter.success(
                `Setting <emphasis>${keyParts}</emphasis> to <emphasis>${JSON.stringify(
                  value,
                )}</emphasis> in the project config <filelink emphasis target="${meta.configPath.join()}" />`,
              );

              if (value === 'true' || value === 'false') {
                const suggestedCommand = value === 'true' ? 'enable' : 'disable';
                reporter.warn(
                  `Value is the string <emphasis>${value}</emphasis> but it looks like a boolean. You probably meant to use the command:`,
                );
                reporter.command(`config ${suggestedCommand} ${keyParts}`);
              }
            },

            modify: (consumer) => {
              // Set the specified value
              let keyConsumer = consumer;
              for (const key of keyParts.split('.')) {
                if (!keyConsumer.exists()) {
                  keyConsumer.setValue({});
                }
                keyConsumer = keyConsumer.get(key);
              }
              keyConsumer.setValue(value);
            },
          },
        );
      } catch (err) {
        reporter.error(
          'Error occured while testing new project config. Your changes have not been saved.',
        );
        throw err;
      }
    },
  },
);
