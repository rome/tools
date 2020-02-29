/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Manifest} from '@romejs/codec-js-manifest';
import Master from '../Master';
import {Platform} from '../../common/types/platform';
import {ProjectDefinition, DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {PLATFORM_ALIASES} from '../../common/types/platform';
import {FileReference} from '../../common/types/files';
import resolverSuggest from './resolverSuggest';
import {
  AbsoluteFilePath,
  createUnknownFilePath,
  UnknownFilePath,
  URLFilePath,
} from '@romejs/path';
import {DiagnosticPointer, PartialDiagnosticAdvice} from '@romejs/diagnostics';
import {IMPLICIT_JS_EXTENSIONS} from '../../common/fileHandlers';
import {writeFile} from '@romejs/fs';
import https = require('https');
import {MOCKS_FOLDER_NAME} from '@romejs/core/common/constants';

function request(
  url: string,
): Promise<
  ResolverQueryResponseFetchError | {type: 'DOWNLOADED'; content: string}
> {
  return new Promise(resolve => {
    const req = https.get(url, res => {
      if (res.statusCode !== 200) {
        console.log('non-200 return');
        resolve({
          type: 'FETCH_ERROR',
          source: undefined,
          advice: [
            {
              type: 'log',
              category: 'info',
              message: `<hyperlink target="${url}" /> returned a ${res.statusCode} status code`,
            },
          ],
        });
        return;
      }

      let data = '';

      res.on('data', chunk => {
        data += chunk;
      });

      res.on('end', () => {
        resolve({type: 'DOWNLOADED', content: data});
      });
    });

    req.on('error', err => {
      resolve({
        type: 'FETCH_ERROR',
        source: undefined,
        advice: [
          {
            type: 'log',
            category: 'info',
            message: `<hyperlink target="${url}" /> resulted in the error "${err.message}"`,
          },
        ],
      });
    });
  });
}

const NODE_MODULES = 'node_modules';

export type ResolverRemoteQuery = Omit<ResolverOptions, 'origin'> & {
  origin: URLFilePath | AbsoluteFilePath;
  source: UnknownFilePath;
  // Allows a resolution to stop at a folder or package boundary
  requestedType?: 'package' | 'folder';
  // Treat the source as a path (without being explicitly relative), and then a module/package if it fails to resolve
  entry?: boolean;
  // Strict disables implicit extensions
  strict?: boolean;
};

export type ResolverLocalQuery = Omit<ResolverRemoteQuery, 'origin'> & {
  origin: AbsoluteFilePath;
};

export type ResolverQuerySource =
  | undefined
  | {
      source?: string;
      pointer?: DiagnosticPointer;
    };

type ResolverQueryResponseFoundType =
  | 'package'
  | 'mock'
  | 'haste'
  | 'implicitPlatform'
  | 'implicitScale'
  | 'implicitExtension'
  | 'implicitIndex';

export type ResolverQueryResponseFound = {
  type: 'FOUND';
  types: Array<ResolverQueryResponseFoundType>;
  path: AbsoluteFilePath;
  ref: FileReference;
};

export type ResolverQueryResponseMissing = {
  type: 'MISSING';
  source: undefined | ResolverQuerySource;
  advice?: undefined;
};

export type ResolverQueryResponseUnsupported = {
  type: 'UNSUPPORTED';
  source: undefined | ResolverQuerySource;
  advice: PartialDiagnosticAdvice;
};

export type ResolverQueryResponseFetchError = {
  type: 'FETCH_ERROR';
  source: undefined | ResolverQuerySource;
  advice: PartialDiagnosticAdvice;
};

type FilenameVariant = {
  path: UnknownFilePath;
  types: Array<ResolverQueryResponseFoundType>;
};

const QUERY_RESPONSE_MISSING: ResolverQueryResponseMissing = {
  type: 'MISSING',
  source: undefined,
};

export type ResolverQueryResponse =
  | ResolverQueryResponseFound
  | ResolverQueryResponseMissing
  | ResolverQueryResponseFetchError
  | ResolverQueryResponseUnsupported;

function shouldReturnQueryResponse(res: ResolverQueryResponse): boolean {
  return res.type === 'FOUND' || res.source !== undefined;
}

export function isPathLike(source: UnknownFilePath): boolean {
  return source.isAbsolute() || source.isExplicitRelative();
}

function appendTypeQueryResponse(
  res: ResolverQueryResponse,
  types: Array<ResolverQueryResponseFoundType>,
): ResolverQueryResponse {
  if (res.type === 'FOUND') {
    return {
      ...res,
      types: [...res.types, ...types],
    };
  } else {
    return res;
  }
}

export type ResolverOptions = {
  origin?: AbsoluteFilePath;
  mocks?: boolean;
  platform?: Platform;
  scale?: number;
};

function getPreferredMainKey(
  manifest: Manifest,
): undefined | {key: string; value: string} {
  if (manifest['jsnext:main'] !== undefined) {
    return {key: 'jsnext:main', value: manifest['jsnext:main']};
  }

  if (manifest.main !== undefined) {
    return {key: 'main', value: manifest.main};
  }
}

export default class Resolver {
  constructor(master: Master) {
    this.master = master;
  }

  master: Master;

  init() {}

  async findProjectFromQuery(query: ResolverRemoteQuery) {
    // If we were passed an absolute path then we should find and add the project it belongs to
    if (query.source.isAbsolute()) {
      await this.master.projectManager.findProject(
        query.source.assertAbsolute(),
      );
    } else if (query.origin.isAbsolute()) {
      const origin = query.origin.assertAbsolute();
      await this.master.projectManager.findProject(origin);
      await this.master.projectManager.findProject(
        origin.append(query.source.assertRelative()),
      );
    }
  }

  async resolveEntryAssert(
    query: ResolverRemoteQuery,
    querySource?: ResolverQuerySource,
  ): Promise<ResolverQueryResponseFound> {
    await this.findProjectFromQuery(query);
    return this.resolveAssert({...query, entry: true}, querySource);
  }

  // I found myself wanting only `ref.path` a lot so this is just a helper method
  async resolveEntryAssertPath(
    query: ResolverRemoteQuery,
    querySource?: ResolverQuerySource,
  ): Promise<AbsoluteFilePath> {
    const res = await this.resolveEntryAssert(query, querySource);
    return res.path;
  }

  async resolveEntry(
    query: ResolverRemoteQuery,
  ): Promise<ResolverQueryResponse> {
    await this.findProjectFromQuery(query);
    return this.resolveRemote({...query, entry: true});
  }

  async resolveAssert(
    query: ResolverRemoteQuery,
    origQuerySource?: ResolverQuerySource,
  ): Promise<ResolverQueryResponseFound> {
    const resolved = await this.resolveRemote(query);
    if (resolved.type === 'FOUND') {
      return resolved;
    } else {
      throw resolverSuggest(this, query, resolved, origQuerySource);
    }
  }

  async resolveRemote(
    query: ResolverRemoteQuery,
  ): Promise<ResolverQueryResponse> {
    const {origin, source} = query;

    if (source.isURL()) {
      const sourceURL = source.assertURL();
      const protocol = sourceURL.getProtocol();

      switch (protocol) {
        case 'http':
        case 'https': {
          let projectConfig = DEFAULT_PROJECT_CONFIG;

          if (origin.isAbsolute()) {
            const project = this.master.projectManager.findProjectExisting(
              query.origin.assertAbsolute(),
            );
            if (project !== undefined) {
              projectConfig = project.config;
            }
          }

          const remotePath = projectConfig.files.vendorPath.append(
            source
              .join()
              .replace(/[\/:]/g, '$')
              .replace(/\$+/g, '$'),
          );

          if (!this.master.memoryFs.exists(remotePath)) {
            const result = await request(source.join());
            if (result.type === 'DOWNLOADED') {
              await writeFile(remotePath, result.content);
            } else {
              return result;
            }
          }

          return {
            type: 'FOUND',
            types: [],
            ref: this.master.projectManager.getURLFileReference(
              remotePath,
              sourceURL,
            ),
            path: remotePath,
          };
        }

        default:
          return {
            type: 'UNSUPPORTED',
            source: undefined,
            advice: [
              {
                type: 'log',
                category: 'info',
                message: `<emphasis>${protocol}</emphasis> is not a supported remote protocol`,
              },
            ],
          };
      }
    }

    if (origin.isURL()) {
      if (source.isAbsolute() || source.isExplicitRelative()) {
        // Relative to the origin
        return this.resolveRemote({
          ...query,
          source: origin.resolve(source),
        });
      } else {
        // TODO add support for import maps
        return {
          type: 'MISSING',
          source: undefined,
        };
      }
    }

    return this.resolveLocal({
      ...query,
      origin: query.origin.assertAbsolute(),
    });
  }

  resolveLocal(query: ResolverLocalQuery): ResolverQueryResponse {
    // Do some basic checks to determine if this is an absolute or relative path
    if (isPathLike(query.source)) {
      return this.resolvePath(query);
    }

    // Now resolve it as a module
    const resolved = this.resolveModule(query);

    // If we didn't resolve to a module, and we were asked to resolve relative, then do that
    if (resolved.type === 'MISSING' && query.entry === true) {
      return this.resolvePath(query);
    }

    return resolved;
  }

  *getFilenameVariants(
    query: ResolverLocalQuery,
    path: UnknownFilePath,
  ): Iterable<FilenameVariant> {
    const seen: Set<string> = new Set();
    for (const variant of this._getFilenameVariants(query, path, [])) {
      const filename = variant.path.join();
      if (seen.has(filename)) {
        continue;
      }

      seen.add(filename);
      yield variant;
    }
  }

  *_getFilenameVariants(
    query: ResolverLocalQuery,
    path: UnknownFilePath,
    callees: Array<ResolverQueryResponseFoundType>,
  ): Iterable<FilenameVariant> {
    const {platform} = query;

    yield {path, types: callees};

    //
    const {handler} = this.master.projectManager.getHandlerWithProject(
      path.isAbsolute() ? path.assertAbsolute() : query.origin,
    );
    const usesUnknownExtension = !query.strict && handler === undefined;

    // Check with appended `platform`
    if (platform !== undefined && !callees.includes('implicitPlatform')) {
      yield* this._getFilenameVariants(
        query,
        path.addExtension(`.${platform}`),
        [...callees, 'implicitPlatform'],
      );

      // Check if this platform has any subplatforms
      const platformAliases = PLATFORM_ALIASES[platform];
      if (platformAliases !== undefined) {
        for (const platform of platformAliases) {
          yield* this._getFilenameVariants(
            query,
            path.addExtension(`.${platform}`, true),
            [...callees, 'implicitPlatform'],
          );
        }
      }
    }

    // Check with appended extensions
    if (usesUnknownExtension && !callees.includes('implicitExtension')) {
      for (const ext of IMPLICIT_JS_EXTENSIONS) {
        yield* this._getFilenameVariants(query, path.addExtension(`.${ext}`), [
          ...callees,
          'implicitExtension',
        ]);
      }
    }

    // Check with appended `scale`, other.filename
    if (
      handler !== undefined &&
      handler.canHaveScale === true &&
      !callees.includes('implicitScale')
    ) {
      const scale = query.scale === undefined ? 3 : query.scale;
      for (let i = scale; i >= 1; i--) {
        yield* this._getFilenameVariants(
          query,
          path.changeBasename(
            `${path.getExtensionlessBasename()}@${String(i)}x${
              path.memoizedExtension
            }`,
          ),
          [...callees, 'implicitScale'],
        );
      }
    }
  }

  finishResolverQueryResponse(
    path: AbsoluteFilePath,
    types?: Array<ResolverQueryResponseFoundType>,
  ): ResolverQueryResponse {
    return {
      type: 'FOUND',
      types: types === undefined ? [] : types,
      ref: this.master.projectManager.getFileReference(path),
      path,
    };
  }

  getOriginFolder(query: ResolverLocalQuery): AbsoluteFilePath {
    const {memoryFs} = this.master;
    const {origin} = query;

    if (memoryFs.isFile(origin)) {
      return origin.getParent();
    } else {
      return origin;
    }
  }

  resolvePath(
    query: ResolverLocalQuery,
    checkVariants: boolean = true,
    types?: Array<ResolverQueryResponseFoundType>,
  ): ResolverQueryResponse {
    const {memoryFs} = this.master;

    // Resolve the path heiarchy
    const originFolder = this.getOriginFolder(query);
    const resolvedOrigin = originFolder.resolve(query.source);

    // Check if this is an absolute filename
    if (memoryFs.isFile(resolvedOrigin)) {
      // If we're querying a package then we should never return a file
      if (query.requestedType === 'package') {
        return QUERY_RESPONSE_MISSING;
      }

      return this.finishResolverQueryResponse(resolvedOrigin, types);
    }

    // Check variants
    if (checkVariants) {
      for (const variant of this.getFilenameVariants(query, resolvedOrigin)) {
        if (variant.path.equal(resolvedOrigin)) {
          continue;
        }

        const resolved = this.resolvePath(
          {...query, source: variant.path},
          false,
          variant.types,
        );

        if (shouldReturnQueryResponse(resolved)) {
          return appendTypeQueryResponse(resolved, variant.types);
        }
      }
    }

    // check if this is a folder
    if (memoryFs.isDirectory(resolvedOrigin)) {
      if (query.requestedType === 'folder') {
        return this.finishResolverQueryResponse(resolvedOrigin, types);
      }

      // If this has a package.json then follow the `main` field
      const manifestDef = memoryFs.getManifestDefinition(resolvedOrigin);
      if (manifestDef !== undefined) {
        // If we're resolving a package then don't follow this
        if (query.requestedType === 'package') {
          return this.finishResolverQueryResponse(resolvedOrigin, types);
        }

        const main = getPreferredMainKey(manifestDef.manifest);
        if (main !== undefined) {
          const resolved = this.resolvePath(
            {
              ...query,
              origin: resolvedOrigin,
              source: createUnknownFilePath(main.value),
            },
            true,
            ['package'],
          );

          if (resolved.type === 'FOUND') {
            return resolved;
          } else {
            const pointer = manifestDef.consumer
              .get(main.key)
              .getDiagnosticPointer('value');

            return {
              ...resolved,
              source:
                pointer === undefined
                  ? undefined
                  : {
                      pointer,
                      source: main.value,
                    },
            };
          }
        }
      }

      if (!query.strict) {
        // Check if it has an index.* file
        for (const ext of IMPLICIT_JS_EXTENSIONS) {
          const indexResolved = this.resolvePath(
            {
              ...query,
              source: resolvedOrigin.append('index.' + ext),
            },
            true,
            ['implicitIndex'],
          );

          if (shouldReturnQueryResponse(indexResolved)) {
            return indexResolved;
          }
        }
      }
    }

    return QUERY_RESPONSE_MISSING;
  }

  resolvePackageFolder(
    query: ResolverLocalQuery,
    moduleName: string,
  ): undefined | AbsoluteFilePath {
    // Find the project
    const project = this.master.projectManager.findProjectExisting(
      query.origin,
    );
    if (project === undefined) {
      return;
    }

    // Find the package
    const projects = this.master.projectManager.getHierarchyFromProject(
      project,
    );

    for (const project of projects) {
      const pkg = project.packages.get(moduleName);
      if (pkg !== undefined) {
        return pkg.folder;
      }
    }
  }

  resolvePackage(
    query: ResolverLocalQuery,
    moduleName: string,
    moduleNameParts: Array<string>,
  ): ResolverQueryResponse {
    const packageDir = this.resolvePackageFolder(query, moduleName);

    if (packageDir === undefined) {
      return QUERY_RESPONSE_MISSING;
    } else {
      return this.resolvePath(
        {
          ...query,
          source: packageDir.append(moduleNameParts),
        },
        true,
        ['package'],
      );
    }
  }

  resolveMock(
    query: ResolverLocalQuery,
    project: ProjectDefinition | undefined,
    parentDirectories: Array<AbsoluteFilePath>,
  ): ResolverQueryResponse {
    if (project === undefined) {
      return QUERY_RESPONSE_MISSING;
    }

    const moduleName = query.source.assertRelative();

    for (const dir of parentDirectories) {
      const mocksDir = dir.append(MOCKS_FOLDER_NAME);

      // No use resolving against a directory that doesn't exist
      if (!this.master.memoryFs.exists(mocksDir)) {
        continue;
      }

      const resolved = this.resolveLocal({
        ...query,
        source: mocksDir.append(moduleName),
      });

      if (shouldReturnQueryResponse(resolved)) {
        return appendTypeQueryResponse(resolved, ['mock']);
      }
    }

    return QUERY_RESPONSE_MISSING;
  }

  resolveHaste(
    query: ResolverLocalQuery,
    mainProject: ProjectDefinition,
    moduleName: string,
    moduleNameParts: Array<string>,
  ): ResolverQueryResponse {
    const projects = this.master.projectManager.getHierarchyFromProject(
      mainProject,
    );

    for (const project of projects) {
      // Check for an entry for the direct name, in the case we're given something like react-relay/modern/ReactRelayQueryFetcher we'll resolve react-relay then resolve from it with the rest of the path
      const resolved = project.hasteMap.get(moduleName);
      if (resolved !== undefined) {
        if (moduleNameParts.length === 0) {
          return this.finishResolverQueryResponse(resolved, ['haste']);
        } else {
          return this.resolvePath(
            {
              ...query,
              source: resolved.append(moduleNameParts),
            },
            true,
            ['haste'],
          );
        }
      }

      // Check all filename variants, we use the full module path here so using the parts separately isn't necessary
      for (const {path} of this.getFilenameVariants(query, query.source)) {
        const resolved = project.hasteMap.get(path.join());
        if (resolved !== undefined) {
          return this.finishResolverQueryResponse(resolved, ['haste']);
        }
      }
    }

    return QUERY_RESPONSE_MISSING;
  }

  // Given a reference to a module, extract the module name and any trailing relative paths
  splitModuleName(path: UnknownFilePath): [string, Array<string>] {
    // fetch the first part of the path as that's the module name
    // possible values of `moduleNameFull` could be `react` or `react/lib/whatever`
    const [moduleName, ...moduleNameParts] = path.getSegments();

    // For scoped modules in the form of `@romejs/bar`, make sure we keep the `/bar` on the module name
    if (moduleName[0] === '@' && moduleNameParts.length > 0) {
      return [moduleName + '/' + moduleNameParts.shift(), moduleNameParts];
    }

    return [moduleName, moduleNameParts];
  }

  resolveModule(query: ResolverLocalQuery): ResolverQueryResponse {
    const {origin, source} = query;

    // get project for the origin
    const project = this.master.projectManager.findProjectExisting(origin);

    // get all the parent directories for when we crawl up
    const parentDirectories = this.getOriginFolder(query).getChain();

    // if mocks are enabled for this query then check all parent mocks folder
    if (query.mocks === true) {
      const mockResolved = this.resolveMock(query, project, parentDirectories);
      if (shouldReturnQueryResponse(mockResolved)) {
        return mockResolved;
      }
    }

    // Extract the module name and it's relative file parts
    const [moduleName, moduleNameParts] = this.splitModuleName(source);

    // Check the haste map
    if (project !== undefined && project.hasteMap.size > 0) {
      const hasteResolved = this.resolveHaste(
        query,
        project,
        moduleName,
        moduleNameParts,
      );
      if (shouldReturnQueryResponse(hasteResolved)) {
        return hasteResolved;
      }
    }

    // Check if it matches any of our project packages
    const packageResolved = this.resolvePackage(
      query,
      moduleName,
      moduleNameParts,
    );
    if (shouldReturnQueryResponse(packageResolved)) {
      return packageResolved;
    }

    // Check all parent directories for node_modules
    for (const dir of parentDirectories) {
      // Check for node_modules/*
      const nodeModulesLoc = dir.append([
        NODE_MODULES,
        source.assertRelative(),
      ]);
      const nodeModulesResolved = this.resolvePath(
        {
          ...query,
          source: nodeModulesLoc,
        },
        true,
        ['package'],
      );
      if (shouldReturnQueryResponse(nodeModulesResolved)) {
        return nodeModulesResolved;
      }
    }

    return QUERY_RESPONSE_MISSING;
  }
}
