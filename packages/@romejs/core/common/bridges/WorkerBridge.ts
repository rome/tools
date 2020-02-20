/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BundleCompileOptions} from '@romejs/js-compiler';
import {ModuleSignature} from '@romejs/js-analysis';
import {Manifest} from '@romejs/codec-js-manifest';
import {Program, ConstSourceType} from '@romejs/js-ast';
import {CompileResult, TransformStageName} from '@romejs/js-compiler';
import {Profile} from '@romejs/v8';
import {ProfilingStartData} from './MasterBridge';
import {PartialDiagnostics} from '@romejs/diagnostics';
import {ProjectConfigJSON} from '@romejs/project';
import {DiagnosticsError} from '@romejs/diagnostics';
import {Bridge} from '@romejs/events';
import {JSONFileReference} from '../types/files';
import {AnalyzeDependencyResult} from '../types/analyzeDependencies';

export type WorkerProjects = Array<{
  id: number;
  folder: string;
  config: undefined | ProjectConfigJSON;
}>;

export type WorkerCompileResult = CompileResult & {
  cached: boolean;
};

export type WorkerPartialManifest = {
  type: Manifest['type'];
};

export type WorkerPartialManifests = Array<{
  id: number;
  manifest: undefined | WorkerPartialManifest;
}>;

// Omit analyze value as the worker will fetch it itself, skips sending over a large payload that it already has in memory

export type WorkerCompilerOptions = {
  bundle?: WorkerBundleCompileOptions;
};

export type WorkerBundleCompileOptions = Omit<BundleCompileOptions, 'analyze'>;

//

export type WorkerAnalyzeDependencyResult = AnalyzeDependencyResult & {
  cached: boolean;
};

export type WorkerParseOptions = {
  compact: boolean;
  sourceType: undefined | ConstSourceType;
};

export type WorkerStatus = {
  astCacheSize: number;
  memoryUsage: {
    rss: number;
    heapTotal: number;
    heapUsed: number;
    external: number;
  };
  pid: number;
  uptime: number;
};

export type PrefetchedModuleSignatures = {
  [key: string]:
    | {
        type: 'USE_CACHED';
        filename: string;
      }
    | {
        type: 'RESOLVED';
        graph: ModuleSignature;
      }
    | {
        type: 'OWNED';
        file: JSONFileReference;
      }
    | {
        type: 'POINTER';
        key: string;
      };
};

export default class WorkerBridge extends Bridge {
  log = this.createEvent<string, void>({
    name: 'log',
    direction: 'server->client',
  });

  updateProjects = this.createEvent<{projects: WorkerProjects}, void>({
    name: 'updateProjects',
    direction: 'server->client',
  });

  updateManifests = this.createEvent<{manifests: WorkerPartialManifests}, void>(
    {
      name: 'updateManifests',
      direction: 'server->client',
    },
  );

  profilingStart = this.createEvent<ProfilingStartData, void>({
    name: 'profiling.start',
    direction: 'server->client',
  });

  profilingStop = this.createEvent<void, Profile>({
    name: 'profiling.stop',
    direction: 'server->client',
  });

  status = this.createEvent<void, WorkerStatus>({
    name: 'status',
    direction: 'server->client',
  });

  evict = this.createEvent<{filename: string}, void>({
    name: 'evict',
    direction: 'server->client',
  });

  moduleSignatureJS = this.createEvent<
    {file: JSONFileReference},
    ModuleSignature
  >({
    name: 'moduleSignatureJS',
    direction: 'server->client',
  });

  analyzeDependencies = this.createEvent<
    {file: JSONFileReference},
    AnalyzeDependencyResult
  >({
    name: 'analyzeDependencies',
    direction: 'server->client',
  });

  lint = this.createEvent<
    {
      file: JSONFileReference;
      prefetchedModuleSignatures: PrefetchedModuleSignatures;
      fix: boolean;
    },
    PartialDiagnostics
  >({name: 'lint', direction: 'server->client'});

  compileJS = this.createEvent<
    {
      file: JSONFileReference;
      stage: TransformStageName;
      options: WorkerCompilerOptions;
    },
    CompileResult
  >({name: 'compileJS', direction: 'server->client'});

  parseJS = this.createEvent<
    {
      file: JSONFileReference;
      opts: WorkerParseOptions;
    },
    Program
  >({name: 'parseJS', direction: 'server->client'});

  init() {
    this.addErrorTransport('DiagnosticsError', {
      serialize(err: Error) {
        if (!(err instanceof DiagnosticsError)) {
          throw new Error('Expected DiagnosticsError');
        }

        return {
          diagnostic: err.diagnostics,
        };
      },
      hydrate(err, data) {
        return new DiagnosticsError(
          String(err.message),
          data.diagnostics as any,
        );
      },
    });
  }
}
