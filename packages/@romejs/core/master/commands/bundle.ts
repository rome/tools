/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {Consumer} from '@romejs/consume';
import {createMasterCommand} from '../../commands';
import Bundler from '../bundler/Bundler';
import {commandCategories} from '../../commands';
import {createDirectory, writeFile} from '@romejs/fs';

export default createMasterCommand({
  category: commandCategories.SOURCE_CODE,
  description: 'build a standalone js bundle for a package',

  async default(
    req: MasterRequest,
    flagConsumer: Consumer,
  ): Promise<{
    content: string;
    map: string;
  }> {
    const {flags} = req.client;
    const {args} = req.query;
    const {reporter} = req;
    req.expectArgumentLength(2);

    const [entryFilename, outputFolder] = args;
    const bundler = Bundler.createFromMasterRequest(req);

    const resolution = await bundler.getResolvedEntry(entryFilename);
    const {files: outFiles, entry} = await bundler.bundleManifest(resolution);

    const savedList = [];
    const dir = flags.cwd.resolve(outputFolder);
    for (const [filename, {kind, content}] of outFiles) {
      const file = dir.append(filename);
      const loc = file.join();
      savedList.push(
        `<filelink target="${loc}">${filename}</filelink> <filesize dim>${Buffer.byteLength(
          content,
        )}</filesize> <inverse>${kind}</inverse>`,
      );
      await createDirectory(file.getParent(), {recursive: true});
      await writeFile(file, content);
    }

    reporter.success(
      `Saved the following files to <filelink target="${dir.join()}" />`,
    );
    reporter.list(savedList);

    return {
      content: entry.js.content,
      map: entry.sourceMap.content,
    };
  },
});
