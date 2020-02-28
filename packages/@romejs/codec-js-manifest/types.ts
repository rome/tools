/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDependencies} from './dependencies';
import {SPDXExpressionNode} from '@romejs/codec-spdx-license';
import {SemverVersionNode} from '@romejs/codec-semver';
import {Consumer} from '@romejs/consume';
import {AbsoluteFilePath} from '@romejs/path';
import {JSONObject, JSONPropertyValue} from '@romejs/codec-json';
import {Dict} from '@romejs/typescript-helpers';

export type StringObject = Dict<string>;
export type MString = undefined | string;
export type MStringArray = undefined | Array<string>;
export type MStringObject = undefined | StringObject;
export type MBoolean = undefined | boolean;

export type ManifestMap = Map<string, string>;

export type ManifestPerson = {
  name: MString;
  email: MString;
  twitter: MString;
  github: MString;
  url: MString;
};

export type ManifestRepository = {
  type: string;
  url: string;
  directory: MString;
};

export type ManifestBugs = {
  url: MString;
  email: MString;
};

export type Manifest = {
  name: MString;
  description: MString;
  version: undefined | SemverVersionNode;
  license: undefined | SPDXExpressionNode;
  private: boolean;
  type: undefined | 'module' | 'commonjs';

  homepage: MString;
  repository: undefined | ManifestRepository;
  bugs: undefined | ManifestBugs;

  browser: MString;
  main: MString;
  'rome:main': MString;
  'jsnext:main': MString;

  author: undefined | ManifestPerson;
  contributors: undefined | Array<ManifestPerson>;
  maintainers: undefined | Array<ManifestPerson>;

  files: Array<string>;
  keywords: Array<string>;
  cpu: Array<string>;
  os: Array<string>;

  bin: ManifestMap;
  scripts: ManifestMap;
  engines: ManifestMap;

  dependencies: ManifestDependencies;
  devDependencies: ManifestDependencies;
  optionalDependencies: ManifestDependencies;
  peerDependencies: ManifestDependencies;
  bundledDependencies: Array<string>;

  raw: JSONObject;
};

// Serialized version of a Manifest
export type JSONManifest = {
  name: Manifest['name'];
  description: Manifest['description'];
  version: MString;
  license: MString;
  private: Manifest['private'];
  type: Manifest['type'];

  homepage: Manifest['homepage'];
  repository: Manifest['repository'];
  bugs: Manifest['bugs'];

  browser: Manifest['browser'];
  main: Manifest['main'];
  'rome:main': Manifest['rome:main'];
  'jsnext:main': Manifest['jsnext:main'];

  author: Manifest['author'];
  contributors: Manifest['contributors'];
  maintainers: Manifest['maintainers'];

  files: MStringArray;
  keywords: MStringArray;
  cpu: MStringArray;
  os: MStringArray;

  bin: MStringObject;
  scripts: MStringObject;
  engines: MStringObject;

  dependencies: MStringObject;
  devDependencies: MStringObject;
  optionalDependencies: MStringObject;
  peerDependencies: MStringObject;
  bundledDependencies: MStringArray;

  [key: string]: JSONPropertyValue;
};

export type ManifestDefinition = {
  path: AbsoluteFilePath;
  folder: AbsoluteFilePath;
  id: number;
  consumer: Consumer;
  manifest: Manifest;
  hash: string;
};
