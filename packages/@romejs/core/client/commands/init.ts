/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from '../../common/commands';
import {createLocalCommand} from '../commands';
import ClientRequest from '../ClientRequest';
import {Dict} from '@romejs/typescript-helpers';
import {exists, writeFile} from '@romejs/fs';
import {VERSION} from '../../common/constants';
import {Consumer} from '@romejs/consume';
import {stringifyRJSON} from '@romejs/codec-json';

export default createLocalCommand({
  category: commandCategories.PROJECT_MANAGEMENT,
  description: 'create a project config',
  usage: '',
  examples: [],
  defineFlags(consumer: Consumer) {
    return {};
  },
  async callback(req: ClientRequest) {
    const {reporter} = req.client;

    const configPath = req.client.flags.cwd.append('rome.rjson');
    if (await exists(configPath)) {
      reporter.error(
        `<filelink target="${configPath.join()}" emphasis>rome.rjson</filelink> file already exists`,
      );
      reporter.info(
        'Use <command>rome config</command> to update an existing config',
      );
      return false;
    }

    const config: Dict<unknown> = {
      version: `^${VERSION}`,
    };
    await writeConfig();

    async function writeConfig() {
      await writeFile(configPath, stringifyRJSON(config));
    }

    // Run lint, capture diagnostics

    reporter.success(
      `Created config <filelink emphasis target="${configPath.join()}" />`,
    );

    return true;
  },
});
