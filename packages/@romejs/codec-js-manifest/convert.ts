/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {stringifySPDXLicense} from '@romejs/codec-spdx-license';
import {ManifestDependencies, stringifyDependencyPattern} from './dependencies';
import {Manifest, JSONManifest} from './types';
import {stringifySemver} from '@romejs/codec-semver';
import {Dict} from '@romejs/typescript-helpers';
import {stringifyPathPattern} from '@romejs/path-match';

export function convertManifestToJSON(manifest: Manifest): JSONManifest {
  return {
    // Include unknown properties from the initial package.json
    ...manifest.raw,

    name: manifest.name,
    description: manifest.description,
    private: manifest.private,
    type: manifest.type,

    homepage: manifest.homepage,
    repository: manifest.repository,
    bugs: manifest.bugs,

    browser: manifest.browser,
    main: manifest.main,
    'rome:main': manifest['rome:main'],
    'jsnext:main': manifest['jsnext:main'],

    author: manifest.author,
    contributors: manifest.contributors,
    maintainers: manifest.maintainers,

    version:
      manifest.version === undefined
        ? undefined
        : stringifySemver(manifest.version),
    license:
      manifest.license === undefined
        ? undefined
        : stringifySPDXLicense(manifest.license),

    files: maybeArray(
      manifest.files.map(pattern => stringifyPathPattern(pattern)),
    ),
    keywords: maybeArray(manifest.keywords),
    cpu: maybeArray(manifest.cpu),
    os: maybeArray(manifest.os),

    bin: mapToObject(manifest.bin),
    scripts: mapToObject(manifest.scripts),
    engines: mapToObject(manifest.engines),

    dependencies: dependencyMapToObject(manifest.dependencies),
    devDependencies: dependencyMapToObject(manifest.devDependencies),
    optionalDependencies: dependencyMapToObject(manifest.optionalDependencies),
    peerDependencies: dependencyMapToObject(manifest.peerDependencies),

    // Common misspelling. If this existed then it was turned into bundledDependencies
    bundleDependencies: undefined,
    bundledDependencies: maybeArray(manifest.bundledDependencies),
  };
}

function maybeArray<T>(items: Array<T>): undefined | Array<T> {
  if (items.length === 0) {
    return undefined;
  } else {
    return items;
  }
}

function mapToObject<T>(map: Map<string, T>): undefined | Dict<T> {
  if (map.size === 0) {
    return;
  }

  const obj: Dict<T> = {};
  for (const [key, value] of map) {
    obj[key] = value;
  }
  return obj;
}

function dependencyMapToObject(
  map: ManifestDependencies,
): undefined | Dict<string> {
  if (map.size === 0) {
    return;
  }

  const obj: Dict<string> = {};
  for (const [key, pattern] of map) {
    obj[key] = stringifyDependencyPattern(pattern);
  }
  return obj;
}
