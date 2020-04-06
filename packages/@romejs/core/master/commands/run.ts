/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {ProjectDefinition} from '@romejs/project';
import {SourceMap} from '@romejs/codec-source-map';
import Bundler from '../bundler/Bundler';
import {commandCategories, createMasterCommand} from '../../commands';
import {AbsoluteFilePath, createRelativeFilePath} from '@romejs/path';

// This will be dispatched to the client where it has a special case for `executeCode`
type RunResult = {
  type: 'executeCode';
  filename: string;
  code: string;
  map: SourceMap;
};

export default createMasterCommand(
  {
    category: commandCategories.PROJECT_MANAGEMENT,
    description: 'TODO',

    async default(req: MasterRequest): Promise<RunResult> {
      const {args} = req.query;
      const {flags} = req.client;
      const {master} = req;
      req.expectArgumentLength(1);

      async function executeCode(path: AbsoluteFilePath): Promise<RunResult> {
        const bundler = Bundler.createFromMasterRequest(req);
        const {entry} = await bundler.bundle(path);
        return {
          type: 'executeCode',
          filename: path.join(),
          code: entry.js.content,
          map: entry.sourceMap.map,
        };
      }

      // Get the current project
      const project: undefined | ProjectDefinition = await master.projectManager.findProject(
        flags.cwd,
      );

      // check for absolute paths
      const target = args[0];
      const resolved = await master.resolver.resolveEntry({
        ...req.getResolverOptionsFromFlags(),
        source: createRelativeFilePath(target),
      });
      if (resolved.type === 'FOUND') {
        return executeCode(resolved.path);
      }

      // check for bin files in any manifests that belong to any projects
      if (project !== undefined) {
        for (const {manifest, folder} of project.packages.values()) {
          const relative = manifest.bin.get(target);
          if (relative === undefined) {
            continue;
          }

          const resolved = await master.resolver.resolveEntryAssertPath({
            ...req.getResolverOptionsFromFlags(),
            origin: folder,
            platform: 'node',
            source: createRelativeFilePath(relative),
          });

          return executeCode(resolved);
        }
      }

      // TODO check node_modules/.bin

      // TODO check package.json scripts
      throw new Error(`Failed to find "${target}"`);
    },
  },
);
