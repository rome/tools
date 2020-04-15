/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig} from '@romejs/project';
import {FileReference} from '@romejs/core';
import {
  WorkerParseOptions,
  WorkerLintOptions,
} from '../common/bridges/WorkerBridge';
import Worker, {ParseResult} from '../worker/Worker';
import {Diagnostics, DiagnosticSuppressions} from '@romejs/diagnostics';
import * as compiler from '@romejs/js-compiler';
import {check as typeCheck} from '@romejs/js-analysis';
import {parseJSON, stringifyJSON, consumeJSONExtra} from '@romejs/codec-json';
import {ConstSourceType, ConstProgramSyntax} from '@romejs/js-ast';
import {
  createUnknownFilePath,
  createAbsoluteFilePath,
  UnknownFilePath,
} from '@romejs/path';
import {
  AnalyzeDependencyResult,
  UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from './types/analyzeDependencies';
import {formatJS} from '@romejs/js-formatter';

type ExtensionsMap = Map<string, ExtensionHandler>;

export type GetFileHandlerResult = {
  ext: string;
  handler?: ExtensionHandler;
};

export function getFileHandlerExtensions(
  projectConfig: ProjectConfig,
): Array<string> {
  return [...DEFAULT_HANDLERS.keys(), ...projectConfig.files.assetExtensions];
}

export function getFileHandler(
  path: UnknownFilePath,
  projectConfig: ProjectConfig,
): GetFileHandlerResult {
  const basename = path.getBasename();

  const match = basename.match(/\.([a-zA-Z]+)$/);
  if (match == null) {
    return {ext: '', handler: undefined};
  }

  const ext: string = match[1];
  let handler = DEFAULT_HANDLERS.get(ext);

  // Allow setting custom assert extensions in the project config
  if (handler === undefined && projectConfig.files.assetExtensions.includes(ext)) {
    handler = assetHandler;
  }

  return {ext, handler};
}

export function getFileHandlerAssert(
  path: UnknownFilePath,
  projectConfig: ProjectConfig,
): Required<GetFileHandlerResult> {
  const {handler, ext} = getFileHandler(path, projectConfig);

  if (handler === undefined) {
    throw new Error(`No file handler found for '${path.join()}'`);
  } else {
    return {handler, ext};
  }
}

export type ExtensionLintInfo = ExtensionHandlerMethodInfo & {
  options: WorkerLintOptions;
  format: boolean;
};

export type ExtensionLintResult = {
  sourceText: string;
  diagnostics: Diagnostics;
  formatted: string;
  suppressions: DiagnosticSuppressions;
};

export type ExtensionHandlerMethodInfo = {
  parseOptions: WorkerParseOptions;
  file: FileReference;
  project: compiler.TransformProjectDefinition;
  worker: Worker;
};

export type ExtensionHandler = {
  sourceType?: ConstSourceType;
  syntax?: Array<ConstProgramSyntax>;
  hasteMode?: 'ext' | 'noext';
  isAsset?: boolean;
  canHaveScale?: boolean;
  lint?: (info: ExtensionLintInfo) => Promise<ExtensionLintResult>;
  format?: (info: ExtensionHandlerMethodInfo) => Promise<ExtensionLintResult>;
  toJavaScript?: (opts: ExtensionHandlerMethodInfo) => Promise<{
    generated: boolean;
    sourceText: string;
  }>;
  analyzeDependencies?: (
    opts: ExtensionHandlerMethodInfo,
  ) => Promise<AnalyzeDependencyResult>;
};

const textHandler: ExtensionHandler = {
  sourceType: 'module',

  // Mock a single default export

  // We could always just pass this through to analyzeDependencies and get the same result due to the toJavaScript call below,

  // but the return value is predictable so we inline it
  async analyzeDependencies() {
    return {
      ...UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
      moduleType: 'es',
      exports: [
        {
          type: 'local',
          // TODO we could fake this?
          loc: undefined,
          kind: 'value',
          valueType: 'other',
          name: 'default',
        },
      ],
    };
  },

  async toJavaScript({file, worker}) {
    const src = await worker.readFile(file.real);
    const serial = JSON.stringify(src);
    return {
      sourceText: `export default ${serial};`,
      generated: true,
    };
  },
};

export const ASSET_EXPORT_TEMPORARY_VALUE = 'VALUE_INJECTED_BY_BUNDLER';

const assetHandler: ExtensionHandler = {
  // analyzeDependencies shim
  ...textHandler,

  canHaveScale: true,
  isAsset: true,

  async toJavaScript() {
    // This exists just so analyzeDependencies has something to look at

    // When bundling we'll have custom logic in the compiler to handle assets and inject the correct string
    return {
      generated: true,
      sourceText: `export default '${ASSET_EXPORT_TEMPORARY_VALUE}';`,
    };
  },
};

const jsonHandler: ExtensionHandler = {
  // analyzeDependencies shim
  ...textHandler,

  hasteMode: 'noext',

  async format(info: ExtensionHandlerMethodInfo): Promise<ExtensionLintResult> {
    const {file, project, worker} = info;
    const {uid} = file;

    const real = createAbsoluteFilePath(file.real);
    const sourceText = await worker.readFile(real);
    const path = createUnknownFilePath(uid);

    let formatted: string = sourceText;

    if (project.config.format.enabled) {
      if (sourceText.length > 50_000) {
        // Fast path for big JSON files
        parseJSON({
          path,
          input: sourceText,
        });
      } else {
        const {consumer, comments, hasExtensions} = consumeJSONExtra({
          input: sourceText,
          path,
        });

        if (hasExtensions) {
          formatted = stringifyJSON({consumer, comments});
        } else {
          formatted = String(JSON.stringify(
            consumer.asUnknown(),
            undefined,
            '  ',
          ));
        }
      }
    }

    return {
      sourceText,
      diagnostics: [],
      suppressions: [],
      formatted,
    };
  },

  async toJavaScript({file, worker}) {
    const src = await worker.readFile(file.real);

    // Parse the JSON to make sure it's valid
    const obj = parseJSON({
      path: createUnknownFilePath(file.uid),
      input: src,
    });

    const rawJson = JSON.stringify(obj);
    const json: string = rawJson === undefined ? 'undefined' : rawJson;

    // TODO handle unicode newlines here
    return {
      sourceText: `export default ${json};`,
      generated: true,
    };
  },
};

// These are extensions that be implicitly tried when a file is referenced
// This is mostly for compatibility with Node.js projects. This list should not
// be extended. Explicit extensions are required in the browser for as modules and
// should be required everywhere.
// TypeScript is unfortunately included here as it produces an error if you use an
// import source with ".ts"
export const IMPLICIT_JS_EXTENSIONS = ['js', 'ts', 'tsx', 'json'];

// Extensions that have a `lint` handler
export const LINTABLE_EXTENSIONS: Array<string> = [];

// Extensions that have a `format` handler
export const FORMATTABLE_EXTENSIONS: Array<string> = [];

function setHandler(ext: string, handler: ExtensionHandler) {
  if (handler.lint !== undefined) {
    LINTABLE_EXTENSIONS.push(ext);
  }

  if (handler.format !== undefined) {
    FORMATTABLE_EXTENSIONS.push(ext);
  }

  DEFAULT_HANDLERS.set(ext, handler);
}

// Used when filtering files, inserted by buildJSHandler
export const JS_EXTENSIONS: Array<string> = [];

function buildJSHandler(
  ext: string,
  syntax: Array<ConstProgramSyntax>,
  sourceType?: ConstSourceType,
): ExtensionHandler {
  JS_EXTENSIONS.push(ext);

  return {
      hasteMode: 'ext',
      syntax,
      sourceType,

      async analyzeDependencies({file, worker, parseOptions}) {
        const {ast, sourceText, project, generated} = await worker.parseJS(
          file,
          parseOptions,
        );
        worker.logger.info(`Analyzing:`, file.real);

        return worker.api.interceptAndAddGeneratedToDiagnostics(
          await compiler.analyzeDependencies({
            ast,
            sourceText,
            project,
            options: {},
          }),
          generated,
        );
      },

      async toJavaScript({file, worker}) {
        return {
          sourceText: await worker.readFile(file.real),
          generated: false,
        };
      },

      async format(
        info: ExtensionHandlerMethodInfo,
      ): Promise<ExtensionLintResult> {
        const {file: ref, parseOptions, worker} = info;

        const {ast, sourceText, generated}: ParseResult = await worker.parseJS(
          ref,
          parseOptions,
        );

        const res = formatJS(ast, {
          typeAnnotations: true,
          format: 'pretty',
        });

        return worker.api.interceptAndAddGeneratedToDiagnostics({
          formatted: res.getCode(),
          sourceText,
          suppressions: [],
          diagnostics: ast.diagnostics,
        }, generated);
      },

      async lint(info: ExtensionLintInfo): Promise<ExtensionLintResult> {
        const {file: ref, project, format, parseOptions, options, worker} = info;

        const {ast, sourceText, generated}: ParseResult = await worker.parseJS(
          ref,
          parseOptions,
        );

        worker.logger.info(`Linting: `, ref.real);

        // Run the compiler in lint-mode which is where all the rules are actually ran
        const res = await compiler.lint({
          options: {},
          ast,
          project,
          sourceText,
          format,
        });

        // Extract lint diagnostics
        let {diagnostics} = res;

        // Only enable typechecking if enabled in .romeconfig
        let typeCheckingEnabled = project.config.typeCheck.enabled === true;
        if (project.config.typeCheck.libs.has(ref.real)) {
          // don't typecheck lib files
          typeCheckingEnabled = false;
        }

        // Run type checking if necessary
        if (typeCheckingEnabled) {
          const typeCheckProvider = await worker.getTypeCheckProvider(
            ref.project,
            options.prefetchedModuleSignatures,
            parseOptions,
          );
          const typeDiagnostics = await typeCheck({
            ast,
            provider: typeCheckProvider,
            project,
          });
          diagnostics = [...diagnostics, ...typeDiagnostics];
        }

        return worker.api.interceptAndAddGeneratedToDiagnostics({
          suppressions: res.suppressions,
          diagnostics,
          sourceText,
          formatted: res.src,
        }, generated);
      },
    };
}

const DEFAULT_HANDLERS: ExtensionsMap = new Map();

const DEFUALT_ASSET_EXTENSIONS = [
  // Images
  'png',
  'jpg',
  'jpeg',
  'gif',

  // Video
  'webm',
  'mp4',
  'm4v',
  'avi',
  'mkv',

  // Audio
  'mp3',

  // Fonts
  'woff',
  'woff2',
  'eot',
  'ttf',
  'otf',
];
for (const ext of DEFUALT_ASSET_EXTENSIONS) {
  setHandler(ext, assetHandler);
}

setHandler('html', textHandler);
setHandler('htm', textHandler);
setHandler('css', textHandler);
setHandler('txt', textHandler);
setHandler('md', textHandler);
setHandler('csv', textHandler);
setHandler('tsv', textHandler);

setHandler('js', buildJSHandler('js', ['jsx', 'flow'])); // TODO eventually remove the syntax shit
setHandler('jsx', buildJSHandler('jsx', ['jsx']));
setHandler('cjs', buildJSHandler('cjs', [], 'script'));
setHandler('mjs', buildJSHandler('mjs', [], 'module'));
setHandler('ts', buildJSHandler('ts', ['ts'], 'module'));
setHandler('tsx', buildJSHandler('tsx', ['ts', 'jsx'], 'module'));
setHandler('json', jsonHandler);
setHandler('rjson', jsonHandler);
