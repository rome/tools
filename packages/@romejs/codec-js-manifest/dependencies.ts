/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from '@romejs/consume';
import {SemverRangeNode, stringifySemver} from '@romejs/codec-semver';
import {parseSemverRange} from '@romejs/codec-semver';
import {tryParseWithOptionalOffsetPosition} from '@romejs/parser-core';
import {createUnknownFilePath} from '@romejs/path';
import {normalizeName} from './name';

export type DependencyPattern =
  | HostedGitPattern
  | HTTPTarballPattern
  | SemverPattern
  | GitPattern
  | FilePattern
  | TagPattern
  | NpmPattern;

export type ManifestDependencies = Map<string, DependencyPattern>;

type UrlWithHash = {url: string; hash: string | undefined};

export function stringifyDependencyPattern(pattern: DependencyPattern): string {
  switch (pattern.type) {
    case 'hosted-git': {
      let str = `${pattern.host}:${pattern.user}/${pattern.repo}`;
      if (pattern.commitish !== undefined) {
        str += `#${pattern.commitish}`;
      }
      return str;
    }

    case 'file':
      return `file:${pattern.path}`;

    case 'semver':
      return stringifySemver(pattern.range);

    case 'tag':
      return pattern.tag;

    case 'git':
    case 'http-tarball':
      if (pattern.hash === undefined) {
        return pattern.url;
      } else {
        return `${pattern.url}#${pattern.hash}`;
      }
    case 'npm':
      if (pattern.version === undefined) {
        return `npm:${pattern.name}`;
      } else {
        return `npm:${pattern.name}@${pattern.version}`;
      }
  }
}

function explodeHashUrl(pattern: string, consumer: Consumer): UrlWithHash {
  const parts = pattern.split('#');

  if (parts.length > 2) {
    throw consumer.unexpected('Too many hashes');
  }

  return {
    hash: parts[1],
    url: parts[0],
  };
}

function removePrefix(prefix: string, value: string): string {
  if (value.startsWith(prefix)) {
    return value.slice(prefix.length);
  } else {
    return value;
  }
}

//# HOSTED GIT

export type HostedGitHost = 'bitbucket' | 'github' | 'gist' | 'gitlab';

type IncompleteHostedGitPattern = {
  type: 'hosted-git';
  host: HostedGitHost;
  user: string;
  repo: string;
  commitish: undefined | string;
};

type HostedGitPattern = IncompleteHostedGitPattern & {
  url: string;
};

const GITHUB_SHORTHAND = /^[^:@%/\s.-][^:@%/\s]*[/][^:@\s/%]+(?:#.*)?$/;

const HOSTED_GIT_PREFIXES: Array<HostedGitHost> = [
  'bitbucket',
  'github',
  'gist',
  'gitlab',
];

function parseHostedGit(
  host: HostedGitHost,
  pattern: string,
  consumer: Consumer,
): HostedGitPattern {
  // Extract and trim hash
  let commitish: undefined | string;
  if (pattern.includes('#')) {
    const hashIndex = pattern.indexOf('#');
    commitish = pattern.slice(hashIndex + 1);
    pattern = pattern.slice(0, hashIndex - 1);
  }

  const parts = pattern.split('/');
  if (parts.length > 2) {
    throw consumer.unexpected('Expected only 2 parts');
  }

  const user = parts[0];
  if (user === undefined) {
    throw consumer.unexpected('We are missing a user!');
  }

  const repo = parts[1];
  if (repo === undefined) {
    throw consumer.unexpected('We are missing a repo!');
  }

  const incomplete: IncompleteHostedGitPattern = {
    type: 'hosted-git',
    host,
    user,
    repo,
    commitish,
  };

  return {
    ...incomplete,
    url: getHostedGitURL(incomplete),
  };
}

export function getHostedGitURL(pattern: IncompleteHostedGitPattern): string {
  switch (pattern.host) {
    case 'bitbucket':
      return '';

    case 'gitlab':

    case 'gist':
      return '';

    case 'github':
      return '';
  }
}

//# REGULAR GIT

type GitPattern = UrlWithHash & {
  type: 'git';
};

const GIT_PATTERN_MATCHERS = [
  /^git:/,
  /^git\+.+:/,
  /^ssh:/,
  /^https?:.+\.git$/,
  /^https?:.+\.git#.+/,
];

function parseGit(pattern: string, consumer: Consumer): GitPattern {
  return {
    type: 'git',
    ...explodeHashUrl(pattern, consumer),
  };
}

//# TARBALL

type HTTPTarballPattern = UrlWithHash & {
  type: 'http-tarball';
};

function parseHttpTarball(
  pattern: string,
  consumer: Consumer,
): HTTPTarballPattern {
  return {
    type: 'http-tarball',
    ...explodeHashUrl(pattern, consumer),
  };
}

//# SEMVER

type SemverPattern = {
  type: 'semver';
  range: SemverRangeNode;
};

function parseSemver(
  pattern: string,
  consumer: Consumer,
  loose: boolean,
): SemverPattern {
  const ast = tryParseWithOptionalOffsetPosition(
    {
      loose,
      path: consumer.path,
      input: pattern,
    },
    {
      getOffsetPosition: () => consumer.getLocation('inner-value').start,
      parse: opts => parseSemverRange(opts),
    },
  );

  return {
    type: 'semver',
    range: ast,
  };
}

//# FILE

const FILE_PREFIX_REGEX = /^\.{1,2}\//;

type FilePattern = {
  type: 'file';
  path: string;
};

function parseFile(pattern: string): FilePattern {
  return {
    type: 'file',
    path: removePrefix('file:', pattern),
  };
}

//# TAG

// This regex will likely need to be refined, not sure what the allowable characters of a tag are
const TAG_REGEX = /^[a-z]+$/g;

type TagPattern = {
  type: 'tag';
  tag: string;
};

function parseTag(pattern: string): TagPattern {
  return {
    type: 'tag',
    tag: pattern,
  };
}

//#

export function parseGitDependencyPattern(
  consumer: Consumer,
): undefined | GitPattern | HostedGitPattern {
  const pattern = consumer.asString();

  for (const host of HOSTED_GIT_PREFIXES) {
    const prefix = `${host}:`;
    if (pattern.startsWith(prefix)) {
      return parseHostedGit(host, removePrefix(prefix, pattern), consumer);
    }
  }

  for (const matcher of GIT_PATTERN_MATCHERS) {
    if (matcher.test(pattern)) {
      return parseGit(pattern, consumer);
    }
  }

  if (GITHUB_SHORTHAND.test(pattern)) {
    return parseHostedGit('github', pattern, consumer);
  }
}

//# NPM

type NpmPattern = {
  type: 'npm';
  name: string;
  version?: string;
};

export function parseNpmDependencyPattern(pattern: string): NpmPattern {
  let name: string = pattern.slice(
    pattern.indexOf('@'),
    pattern.lastIndexOf('@'),
  );
  let version: string = pattern.slice(pattern.lastIndexOf('@') + 1);

  return {
    type: 'npm',
    name: name,
    version: version,
  };
}

export function parseDependencyPattern(
  consumer: Consumer,
  loose: boolean,
): DependencyPattern {
  const pattern = consumer.asString();

  const gitPattern = parseGitDependencyPattern(consumer);
  if (gitPattern !== undefined) {
    return gitPattern;
  }

  if (pattern.startsWith('http://') || pattern.startsWith('https://')) {
    return parseHttpTarball(pattern, consumer);
  }

  if (
    FILE_PREFIX_REGEX.test(pattern) ||
    createUnknownFilePath(pattern).isAbsolute() ||
    pattern.startsWith('file:')
  ) {
    return parseFile(pattern);
  }

  if (pattern.match(TAG_REGEX)) {
    return parseTag(pattern);
  }

  if (pattern.startsWith('npm:')) {
    return parseNpmDependencyPattern(pattern);
  }

  return parseSemver(pattern, consumer, loose);
}

export function normalizeDependencies(
  root: Consumer,
  key: string,
  loose: boolean,
): ManifestDependencies {
  const map: ManifestDependencies = new Map();

  if (!root.has(key)) {
    return map;
  }

  const consumer = root.get(key);

  // Some ridiculous code has the dependencies property as an empty array
  if (Array.isArray(consumer.asUnknown()) && loose) {
    return map;
  }

  for (const [name, value] of consumer.asMap()) {
    normalizeName({
      name,
      loose,
      unexpected: ({message, at, advice}) => {
        value.unexpected(message, {
          at,
          advice,
          target: 'key',
        });
      },
    });

    map.set(name, parseDependencyPattern(value, loose));
  }

  return map;
}
