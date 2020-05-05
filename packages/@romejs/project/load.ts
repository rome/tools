/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// In this file, all methods are synchronous. This is pretty gross since the rest of Rome is async everything.
// This is required so we can integrate the project config code in third-party integrations with sync architectures.
// Project configs are initialized very infrequently anyway so we can live with the extremely minor perf hit.
import {Consumer} from '@romejs/consume';
import {
  DEFAULT_PROJECT_CONFIG,
  PartialProjectConfig,
  ProjectConfig,
  ProjectConfigMeta,
  ProjectConfigMetaHard,
  ProjectConfigObjects,
  ProjectConfigTarget,
} from './types';
import {parsePathPattern} from '@romejs/path-match';
import {
  arrayOfPatterns,
  arrayOfStrings,
  getParentConfigDependencies,
  mergeAbsoluteFilePathSets,
  mergeArrays,
} from './utils';
import {ConsumeJSONResult, consumeJSONExtra} from '@romejs/codec-json';
import {AbsoluteFilePath, AbsoluteFilePathSet} from '@romejs/path';
import {ob1Add, ob1Coerce1, ob1Inc, ob1Number0} from '@romejs/ob1';
import {existsSync, lstatSync, readFileTextSync, readdirSync} from '@romejs/fs';
import crypto = require('crypto');

import {ROME_CONFIG_PACKAGE_JSON_FIELD} from './constants';
import {parseSemverRange} from '@romejs/codec-semver';
import {descriptions} from '@romejs/diagnostics';

const WATCHMAN_CONFIG_FILENAME = '.watchmanconfig';
const IGNORE_FILENAMES = ['.gitignore', '.hgignore'];

function categoryExists(consumer: Consumer): boolean {
  if (!consumer.exists()) {
    return false;
  }

  const value = consumer.asUnknown();
  if (typeof value === 'boolean') {
    consumer.unexpected(descriptions.PROJECT_CONFIG.BOOLEAN_CATEGORY(value));
    return false;
  }

  return true;
}

export function loadCompleteProjectConfig(
  projectFolder: AbsoluteFilePath,
  configPath: AbsoluteFilePath,
): {
  meta: ProjectConfigMeta;
  config: ProjectConfig;
} {
  // TODO use consumer.capture somehow here to aggregate errors
  const {partial, meta} = loadPartialProjectConfig(projectFolder, configPath);
  const {consumer} = meta;

  // Produce a defaultConfig with some folder specific values
  const defaultConfig: ProjectConfig = {
    ...DEFAULT_PROJECT_CONFIG,
    vcs: {
      ...DEFAULT_PROJECT_CONFIG.vcs,
      root: projectFolder,
    },
  };

  const name = consumer.get('name').asString(
    `project-${projectFolder.getBasename()}`,
  );

  const config: ProjectConfig = {
    ...DEFAULT_PROJECT_CONFIG,
    name,
    root: partial.root === undefined
      ? DEFAULT_PROJECT_CONFIG.root
      : partial.root,
    ...mergePartialConfig(defaultConfig, partial),
  };

  // Infer VCS ignore files as lint ignore rules
  for (const filename of IGNORE_FILENAMES) {
    const possiblePath = config.vcs.root.append(filename);
    meta.configDependencies.add(possiblePath);

    if (existsSync(possiblePath)) {
      const file = readFileTextSync(possiblePath);
      const lines: Array<string> = file.split('\n');

      let index = ob1Number0;

      consumer.handleThrownDiagnostics(() => {
        const patterns = lines.map((line, i) => {
          const pattern = parsePathPattern({
            input: line,
            path: possiblePath,
            offsetPosition: {
              index,
              line: ob1Coerce1(i),
              column: ob1Number0,
            },
          });

          index = ob1Add(index, line.length);

          // Newline char
          index = ob1Inc(index);

          return pattern;
        });

        // TODO: Maybe these are useful in other places?
        config.lint.ignore = [...config.lint.ignore, ...patterns];
      });
    }
  }

  // Set fs.watchman=true when the file .watchmanconfig is present and no fs.watchman config was set
  if (partial.files.watchman === undefined) {
    // Try the project and vcs.root folder for a .watchmanconfig
    // We do the Set magic to only visit the projectFolder once if it is also the vcs.root
    for (const dir of new AbsoluteFilePathSet([projectFolder, config.vcs.root])) {
      const watchmanConfigPath = dir.append(WATCHMAN_CONFIG_FILENAME);
      meta.configDependencies.add(watchmanConfigPath);
      if (existsSync(watchmanConfigPath)) {
        config.files.watchman = true;
      }
    }
  }

  return {
    config,
    meta,
  };
}

function loadPartialProjectConfig(
  projectFolder: AbsoluteFilePath,
  configPath: AbsoluteFilePath,
): ReturnType<typeof normalizeProjectConfig> {
  const configFile = readFileTextSync(configPath);
  const res = consumeJSONExtra({
    path: configPath,
    input: configFile,
  });

  return normalizeProjectConfig(res, configPath, configFile, projectFolder);
}

export function normalizeProjectConfig(
  res: ConsumeJSONResult,
  configPath: AbsoluteFilePath,
  configFile: string,
  projectFolder: AbsoluteFilePath,
): {
  partial: PartialProjectConfig;
  meta: ProjectConfigMetaHard;
} {
  let {consumer} = res;

  let configSourceSubKey;
  let name: undefined | string;
  const isInPackageJson = configPath.getBasename() === 'package.json';
  if (isInPackageJson) {
    // Infer name from package.json
    name = consumer.get('name').asStringOrVoid();

    consumer = consumer.get(ROME_CONFIG_PACKAGE_JSON_FIELD);
    configSourceSubKey = ROME_CONFIG_PACKAGE_JSON_FIELD;
  }

  const hash = crypto.createHash('sha256').update(configFile).digest('hex');

  const config: PartialProjectConfig = {
    compiler: {},
    bundler: {},
    cache: {},
    lint: {},
    resolver: {},
    develop: {},
    typeCheck: {},
    tests: {},
    files: {},
    vcs: {},
    dependencies: {},
    targets: new Map(),
  };

  if (name !== undefined) {
    config.name = name;
  }

  const meta: ProjectConfigMetaHard = {
    projectFolder,
    configPath,
    consumer,
    consumersChain: [consumer],
    configHashes: [hash],
    configSourceSubKey,
    configDependencies: getParentConfigDependencies(projectFolder),
  };

  // We never use `name` here but it's used in `loadCompleteProjectConfig`
  consumer.markUsedProperty('name');

  if (consumer.has('version')) {
    const version = consumer.get('version');

    consumer.handleThrownDiagnostics(() => {
      config.version = parseSemverRange({
        path: consumer.filename,
        input: version.asString(),
        offsetPosition: version.getLocation('inner-value').start,
      });

      // TODO verify that config.version range satisfies current version
    });
  }

  if (consumer.has('root')) {
    config.root = consumer.get('root').asBoolean();
  }

  const cache = consumer.get('cache');
  if (categoryExists(cache)) {
    // TODO
  }

  const resolver = consumer.get('resolver');
  if (categoryExists(resolver)) {
    // TODO
  }

  const bundler = consumer.get('bundler');
  if (categoryExists(bundler)) {
    if (bundler.has('mode')) {
      config.bundler.mode = bundler.get('mode').asStringSetOrVoid([
        'modern',
        'legacy',
      ]);
    }
  }

  const typeChecking = consumer.get('typeChecking');
  if (categoryExists(typeChecking)) {
    if (typeChecking.has('enabled')) {
      config.typeCheck.enabled = typeChecking.get('enabled').asBoolean();
    }

    if (typeChecking.has('libs')) {
      const libs = normalizeTypeCheckingLibs(
        projectFolder,
        typeChecking.get('libs'),
      );
      config.typeCheck.libs = libs.files;
      meta.configDependencies = new AbsoluteFilePathSet([
        ...meta.configDependencies,
        ...libs.folders,
        ...libs.files,
      ]);
    }
  }

  const dependencies = consumer.get('dependencies');
  if (categoryExists(dependencies)) {
    if (dependencies.has('enabled')) {
      config.dependencies.enabled = dependencies.get('dependencies').asBoolean();
    }
  }

  const lint = consumer.get('lint');
  if (categoryExists(lint)) {
    if (lint.has('ignore')) {
      config.lint.ignore = arrayOfPatterns(lint.get('ignore'));
    }

    if (lint.has('globals')) {
      config.lint.globals = arrayOfStrings(lint.get('globals'));
    }
  }

  const tests = consumer.get('tests');
  if (categoryExists(tests)) {
    if (tests.has('ignore')) {
      config.tests.ignore = arrayOfPatterns(tests.get('ignore'));
    }
  }

  const develop = consumer.get('develop');
  if (categoryExists(develop)) {
    if (develop.has('serveStatic')) {
      config.develop.serveStatic = develop.get('serveStatic').asBoolean();
    }
  }

  const files = consumer.get('files');
  if (categoryExists(files)) {
    if (files.has('watchman')) {
      config.files.watchman = files.get('watchman').asBoolean();
    }

    if (files.has('vendorPath')) {
      config.files.vendorPath = projectFolder.resolve(
        files.get('vendorPath').asString(),
      );
    }

    if (files.has('maxSize')) {
      config.files.maxSize = files.get('maxSize').asNumber();
    }

    if (files.has('assetExtensions')) {
      config.files.assetExtensions = files.get('assetExtensions').asArray().map((
        item,
      ) => item.asString());
    }
  }

  const vcs = consumer.get('vcs');
  if (categoryExists(vcs)) {
    if (vcs.has('root')) {
      config.vcs.root = projectFolder.resolve(vcs.get('root').asString());
    }
  }

  const compiler = consumer.get('compiler');
  if (categoryExists(compiler)) {
    // TODO
  }

  const targets = consumer.get('targets');
  if (categoryExists(targets)) {
    for (const [name, object] of targets.asMap()) {
      const target: ProjectConfigTarget = {
        constraints: object.get('constraints').asImplicitArray().map((item) =>
          item.asString()
        ),
      };
      object.enforceUsedProperties('config target property');
      config.targets.set(name, target);
    }
  }

  // Need to get this before enforceUsedProperties so it will be flagged
  const _extends = consumer.get('extends');

  // Flag unknown properties
  consumer.enforceUsedProperties('config property');

  if (_extends.exists()) {
    return extendProjectConfig(projectFolder, _extends, config, meta);
  }

  return {
    partial: config,
    meta,
  };
}

function normalizeTypeCheckingLibs(
  projectFolder: AbsoluteFilePath,
  consumer: Consumer,
): {
  folders: Array<AbsoluteFilePath>;
  files: AbsoluteFilePathSet;
} {
  const libFiles: AbsoluteFilePathSet = new AbsoluteFilePathSet();

  // Normalize library folders
  const folders: Array<AbsoluteFilePath> = arrayOfStrings(consumer).map((
    libFolder,
  ) => projectFolder.resolve(libFolder));

  // Crawl library folders and add their files
  for (const folder of folders) {
    const files = readdirSync(folder);
    for (const file of files) {
      const stats = lstatSync(file);
      if (stats.isFile()) {
        libFiles.add(file);
      } else if (stats.isDirectory()) {
        folders.push(file);
      }
    }
  }

  return {
    files: libFiles,
    folders,
  };
}

function extendProjectConfig(
  projectFolder: AbsoluteFilePath,
  extendsStrConsumer: Consumer,
  config: PartialProjectConfig,
  meta: ProjectConfigMetaHard,
): ReturnType<typeof normalizeProjectConfig> {
  const extendsRelative = extendsStrConsumer.asString();

  if (extendsRelative === 'parent') {
    // TODO maybe do some magic here?
  }

  const extendsPath = projectFolder.resolve(extendsRelative);
  const {partial: extendsObj, meta: extendsMeta} = loadPartialProjectConfig(
    extendsPath.getParent(),
    extendsPath,
  );

  // Check for recursive config
  for (const path of extendsMeta.configDependencies) {
    if (path.equal(extendsPath)) {
      throw extendsStrConsumer.unexpected(
        descriptions.PROJECT_CONFIG.RECURSIVE_CONFIG,
      );
    }
  }

  const merged: PartialProjectConfig = mergePartialConfig(extendsObj, config);

  const lintIgnore = mergeArrays(extendsObj.lint.ignore, config.lint.ignore);
  if (lintIgnore !== undefined) {
    merged.lint.ignore = lintIgnore;
  }

  const lintGlobals = mergeArrays(extendsObj.lint.globals, config.lint.globals);
  if (lintGlobals !== undefined) {
    merged.lint.globals = lintGlobals;
  }

  const testingIgnore = mergeArrays(
    extendsObj.tests.ignore,
    config.tests.ignore,
  );
  if (testingIgnore !== undefined) {
    merged.tests.ignore = testingIgnore;
  }

  const typeCheckingLibs = mergeAbsoluteFilePathSets(
    extendsObj.typeCheck.libs,
    config.typeCheck.libs,
  );
  if (typeCheckingLibs !== undefined) {
    merged.typeCheck.libs = typeCheckingLibs;
  }

  return {
    partial: merged,
    meta: {
      ...meta,
      consumersChain: [...meta.consumersChain, ...extendsMeta.consumersChain],
      configDependencies: new AbsoluteFilePathSet([
        ...meta.configDependencies,
        ...extendsMeta.configDependencies,
        extendsPath,
      ]),
      configHashes: [...meta.configHashes, ...extendsMeta.configHashes],
    },
  };
}

type MergedPartialConfig<
  A extends PartialProjectConfig,
  B extends PartialProjectConfig
> = {[Key in keyof ProjectConfigObjects]: A[Key] & B[Key]};

function mergePartialConfig<
  A extends PartialProjectConfig,
  B extends PartialProjectConfig
>(a: A, b: B): MergedPartialConfig<A, B> {
  return {
    cache: {
      ...a.cache,
      ...b.cache,
    },
    compiler: {
      ...a.compiler,
      ...b.compiler,
    },
    lint: {
      ...a.lint,
      ...b.lint,
    },
    develop: {
      ...a.develop,
      ...b.develop,
    },
    bundler: {
      ...a.bundler,
      ...b.bundler,
    },
    dependencies: {
      ...a.dependencies,
      ...b.dependencies,
    },
    resolver: {
      ...a.resolver,
      ...b.resolver,
    },
    typeCheck: {
      ...a.typeCheck,
      ...b.typeCheck,
    },
    tests: {
      ...a.tests,
      ...b.tests,
    },
    files: {
      ...a.files,
      ...b.files,
    },
    vcs: {
      ...a.vcs,
      ...b.vcs,
    },
    targets: new Map([...a.targets.entries(), ...b.targets.entries()]),
  };
}
