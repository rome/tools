/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {commandCategories} from '../../common/commands';
import {createMasterCommand} from '../commands';
import Bundler from '../bundler/Bundler';
import {createDirectory, writeFile} from '@romejs/fs';
import {Consumer} from '@romejs/consume';
import {markup} from '@romejs/string-markup';

type Flags = {
  quiet: boolean;
};

export default createMasterCommand<Flags>({
  category: commandCategories.SOURCE_CODE,
  description: 'build a standalone js bundle for a package',
  usage: '',
  examples: [],
  defineFlags(consumer: Consumer): Flags {
    return {
      quiet: consumer.get('quiet').asBoolean(false),
    };
  },
  async callback(req: MasterRequest, commandFlags: Flags): Promise<void> {
    const {flags} = req.client;
    const {args} = req.query;
    const {reporter} = req;
    req.expectArgumentLength(2);

    const [entryFilename, outputFolder] = args;
    const bundler = Bundler.createFromMasterRequest(req);

    const resolution = await bundler.getResolvedEntry(entryFilename);
    const {files: outFiles} = await bundler.bundleManifest(resolution);

    const savedList = [];
    const dir = flags.cwd.resolve(outputFolder);
    for (const [filename, {kind, content}] of outFiles) {
      const buff = content();
      const file = dir.append(filename);
      const loc = file.join();
      savedList.push(
        markup`<filelink target="${loc}">${filename}</filelink> <filesize dim>${Buffer.byteLength(
          buff,
        )}</filesize> <inverse>${kind}</inverse>`,
      );
      await createDirectory(file.getParent(), {recursive: true});
      await writeFile(file, buff);
    }

    if (commandFlags.quiet) {
      reporter.success(markup`Saved to <filelink target="${dir.join()}" />`);
    } else {
      reporter.success(
        markup`Saved the following files to <filelink target="${dir.join()}" />`,
      );
      reporter.list(savedList);
    }
  },
});
