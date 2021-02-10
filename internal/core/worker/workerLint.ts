import { lint } from "@internal/compiler";
import { catchDiagnostics, descriptions } from "@internal/diagnostics";
import { markup } from "@internal/markup";
import { WorkerFormatOptions, WorkerLintOptions, WorkerLintResult, WorkerParseOptions, WorkerFormatResult, WorkerLintTimings, EMPTY_LINT_TIMINGS } from "./types";
import { getFileHandlerFromPathAssert } from "../common/file-handlers";
import { ExtensionHandler, ExtensionLintResult } from "../common/file-handlers/types";
import { FileReference } from "../common/types/files";
import * as jsAnalysis from "@internal/js-analysis";
import Worker from "./Worker";
import { formatAST } from "@internal/formatter";
import {maybeRunESLint} from "./integrations/eslint";

const EMPTY_LINT_RESULT: WorkerLintResult = {
  timingsNs: EMPTY_LINT_TIMINGS,
  save: undefined,
  diagnostics: [],
  suppressions: [],
}

type Param<Options> = {
  worker: Worker
  ref: FileReference,
  options: Options,
  parseOptions: WorkerParseOptions,
};

// Some Windows git repos will automatically convert Unix line endings to Windows
// This retains the line endings for the formatted code if they were present in the source
export function normalizeFormattedLineEndings(
	sourceText: string,
	formatted: string,
): string {
	if (sourceText.includes("\r")) {
		return formatted.replace(/\n/g, "\r\n");
	} else {
		return formatted;
	}
}

async function lintOrFormat(handler: ExtensionHandler, param: Param<WorkerLintOptions>): Promise<undefined | ExtensionLintResult> {
  if (handler.capabilities.lint) {
    return compilerLint(param);
  }

  const res = await uncachedFormat({
    ...param,
    options: {},
  });

  if (res === undefined) {
    return undefined;
  }

  const {mtimeNs, result} = res;

  return {
    mtimeNs,
    sourceText: result.original,
    diagnostics: result.diagnostics,
    formatted: result.formatted,
    suppressions: result.suppressions,
    timingsNs: EMPTY_LINT_TIMINGS,
  };
}

export async function uncachedLint(param: Param<WorkerLintOptions>): Promise<WorkerLintResult> {
  const {worker, ref, options} = param;
  worker.logger.info(markup`Linting: ${ref.real}`);

  const project = worker.getProject(ref.project);

  // Get the extension handler
  const {handler} = getFileHandlerFromPathAssert(ref.real, project.config);

  if (!(handler.capabilities.lint || handler.capabilities.format)) {
    return EMPTY_LINT_RESULT;
  }

  // Catch any diagnostics, in the case of syntax errors etc
  const res = await catchDiagnostics(
    () => lintOrFormat(handler, param),
    {
      category: "lint",
      message: "Caught by WorkerAPI.lint",
    },
  );

  // These are fatal diagnostics
  if (res.diagnostics !== undefined) {
    return {
      ...EMPTY_LINT_RESULT,
      diagnostics: res.diagnostics,
    };
  }

  // `format` could have return undefined which indicates no support
  if (res.value === undefined) {
    return EMPTY_LINT_RESULT;
  }

  // These are normal diagnostics returned from the linter
  const {
    sourceText,
    diagnostics,
    suppressions,
    mtimeNs,
    timingsNs,
  }: ExtensionLintResult = res.value;

  const formatted = normalizeFormattedLineEndings(
    sourceText,
    res.value.formatted,
  );

  // If the file has pending fixes
  const needsSave = project.config.format.enabled && formatted !== sourceText;

  // Autofix if necessary
  if (options.save && needsSave) {
    return {
      timingsNs,
      save: {
        type: "WRITE",
        mtimeNs,
        content: formatted,
      },
      diagnostics,
      suppressions,
    };
  }

  // If there's no pending fix then no need for diagnostics
  if (!needsSave) {
    return {
      timingsNs,
      save: undefined,
      diagnostics,
      suppressions,
    };
  }

  // Add pending autofix diagnostic
  return {
    timingsNs,
    save: undefined,
    suppressions,
    diagnostics: [
      ...diagnostics,
      {
        tags: {fixable: true},
        location: {
          path: ref.uid,
        },
        description: descriptions.LINT.PENDING_FIXES(
          ref.relative.join(),
          handler.language,
          sourceText,
          formatted,
        ),
      },
    ],
  };
}

export async function compilerLint(
  {worker, ref, options, parseOptions}: Param<WorkerLintOptions>,
): Promise<ExtensionLintResult> {
  const {ast, mtimeNs, sourceText, project, astModifiedFromSource} = await worker.parse(
    ref,
    parseOptions,
  );

  const res = await lint({
    applySafeFixes: options.applySafeFixes,
    suppressionExplanation: options.suppressionExplanation,
    ref,
    options: {
      lint: options.compilerOptions,
    },
    ast,
    project,
    sourceText,
  });

  // Extract lint diagnostics
  let {diagnostics} = res;

  // Only enable typechecking if enabled in .romeconfig
  let typeCheckingEnabled = project.config.typeCheck.enabled;
  if (project.config.typeCheck.libs.has(ref.real)) {
    // don't typecheck lib files
    typeCheckingEnabled = false;
  }

  // Run type checking if necessary
  if (typeCheckingEnabled && ast.type === "JSRoot") {
    const typeCheckProvider = await worker.getTypeCheckProvider(
      ref.project,
      options.prefetchedModuleSignatures,
      parseOptions,
    );
    const typeDiagnostics = await jsAnalysis.check({
      ast,
      provider: typeCheckProvider,
      project,
    });
    diagnostics = [...diagnostics, ...typeDiagnostics];
  }

  let timingsNs: WorkerLintTimings = EMPTY_LINT_TIMINGS;

  const eslintResult = await maybeRunESLint({worker, ref, project});
  if (eslintResult !== undefined) {
    timingsNs = {
      ...timingsNs,
      eslint: eslintResult.timingNs,
    };

    diagnostics = [
      ...diagnostics,
      ...eslintResult.diagnostics,
    ];
  }

  return worker.api.interceptDiagnostics(
    {
      timingsNs,
      suppressions: res.suppressions,
      diagnostics,
      sourceText,
      formatted: res.src,
      mtimeNs,
    },
    {astModifiedFromSource},
  );
}

export async function uncachedFormat(
  {worker, ref, options, parseOptions}: Param<WorkerFormatOptions>,
): Promise<undefined | {
  mtimeNs: bigint;
  result: WorkerFormatResult,
}> {
  const project = worker.getProject(ref.project);

  worker.logger.info(markup`Formatting: ${ref.real}`);

  const {handler} = getFileHandlerFromPathAssert(ref.real, project.config);

  if (
    !(options.forceFormat ||
    (handler.capabilities.format && project.config.format.enabled))
  ) {
    return undefined;
  }

  const {customFormat} = handler;
  if (customFormat !== undefined) {
    const [integrity, stats] = await Promise.all([
      worker.cache.getIntegrity(ref),
      worker.cache.getFile(ref).then((file) => file.getStats()),
    ]);

    const res = await customFormat({
      integrity,
      mtimeNs: stats.mtimeNs,
      file: ref,
      project,
      worker,
      parseOptions,
    });

    return {
      mtimeNs: res.mtimeNs,
      result: {
        original: res.sourceText,
        formatted: res.formatted,
        diagnostics: res.diagnostics,
        suppressions: res.suppressions,
      },
    };
  }

  const {ast, sourceText, astModifiedFromSource, mtimeNs} = await worker.parse(
    ref,
    parseOptions,
  );

  const out = formatAST(
    ast,
    {
      ...options,
      projectConfig: project.config,
    },
  );

  return  {
    mtimeNs,
    result: worker.api.interceptDiagnostics({
      formatted: normalizeFormattedLineEndings(sourceText, out.code),
      original: sourceText,
      diagnostics: ast.diagnostics,
      suppressions: [],
    }, {astModifiedFromSource}),
  };
}