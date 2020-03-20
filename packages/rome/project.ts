/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformProjectDefinition} from '@romejs/js-compiler';
import {
  AbsoluteFilePath,
  AbsoluteFilePathMap,
  AbsoluteFilePathSet,
} from '@romejs/path';
import {
  loadCompleteProjectConfig,
  ProjectConfigMeta,
  ProjectConfig,
  ROME_CONFIG_FILENAMES,
  ROME_CONFIG_PACKAGE_JSON_FIELD,
  getParentConfigDependencies,
} from '@romejs/project';
import {exists, readFileText} from '@romejs/fs';
import {consumeJSON} from '@romejs/codec-json';
import fs = require('fs');

type FoundProject = {
  watchers: AbsoluteFilePathMap<fs.FSWatcher>;
  meta: ProjectConfigMeta;
  config: ProjectConfig;
  definition: TransformProjectDefinition;
};

const foundProjects: AbsoluteFilePathMap<FoundProject> =
new AbsoluteFilePathMap();

function addProject(
  projectFolder: AbsoluteFilePath,
  configPath: AbsoluteFilePath,
) {
  const {config, meta} = loadCompleteProjectConfig(projectFolder, configPath);

  const project: FoundProject = {
    watchers: new AbsoluteFilePathMap(),
    config,
    meta,
    definition: {
      config,
      folder: projectFolder,
    },
  };

  foundProjects.set(projectFolder, project);

  watchEvict(project, meta.configDependencies);
}

function watchEvict(project: FoundProject, deps: AbsoluteFilePathSet) {
  for (const cachePath of deps) {
    if (project.watchers.has(cachePath)) {
      continue;
    }

    const watcher = fs.watch(cachePath.join(), () => {
      evictProject(project);
    });
    project.watchers.set(cachePath, watcher);
  }
}

function evictProject(evictProject: FoundProject) {
  for (const watcher of evictProject.watchers.values()) {
    watcher.close();
  }

  for (const [path, project] of foundProjects) {
    if (project === evictProject) {
      foundProjects.delete(path);
    }
  }
}

export async function findProject(path: AbsoluteFilePath): Promise<FoundProject> {
  const tried: Array<AbsoluteFilePath> = [];

  for (const segment of path.getChain()) {
    // Check if we've already found a project for this
    const cached = foundProjects.get(segment);
    if (cached !== undefined) {
      // Set paths that we've already visited
      for (const segment of tried) {
        foundProjects.set(segment, cached);
      }

      // Invalidate cache entries when files that are deeper than the project we found could create a new project target
      watchEvict(cached, getParentConfigDependencies(segment.getParent()));

      return cached;
    }

    // Check for possible configs
    for (const configFilename of ROME_CONFIG_FILENAMES) {
      const possibleConfigPath = segment.append(configFilename);
      if (await exists(possibleConfigPath)) {
        addProject(segment, possibleConfigPath);

        // Get the definition again. This time hitting the cached value and then setting all ancestors.
        return findProject(path);
      }
    }

    // Check for package.json
    const packagePath = segment.append('package.json');
    if (await exists(packagePath)) {
      const input = await readFileText(packagePath);
      const json = await consumeJSON({input, path: packagePath});
      if (json.has(ROME_CONFIG_PACKAGE_JSON_FIELD)) {
        addProject(segment, packagePath);
        return findProject(path);
      }
    }
  }

  throw new Error(`No project found for ${path.join()}`);
}
