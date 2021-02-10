import { AnyRoot, ConstJSSourceType } from "@internal/ast";
import { Manifest } from "@internal/codec-js-manifest";
import { BundleCompileOptions, CompileResult, CompilerProject, LintCompilerOptions } from "@internal/compiler";
import { DiagnosticIntegrity, Diagnostics, DiagnosticSuppressions } from "@internal/diagnostics";
import { BridgeClient } from "@internal/events";
import { FormatterOptions } from "@internal/formatter";
import { ModuleSignature } from "@internal/js-analysis";
import { Number0 } from "@internal/ob1";
import { AbsoluteFilePath, AnyPath } from "@internal/path";
import { Dict } from "@internal/typescript-helpers";
import WorkerBridge from "../common/bridges/WorkerBridge";
import { AnalyzeDependencyResult } from "../common/types/analyzeDependencies";
import { FileReference } from "../common/types/files";
import { UserConfig } from "../common/userConfig";
import { RecoverySaveFile } from "../server/fs/RecoveryStore";

export type WorkerParseResult = {
	ast: AnyRoot;
	integrity: undefined | DiagnosticIntegrity;
	mtimeNs: bigint;
	project: WorkerProject;
	path: AbsoluteFilePath;
	lastAccessed: number;
	sourceText: string;
	astModifiedFromSource: boolean;
};

export type WorkerBuffer = {
	content: string;
	mtimeNs: bigint;
};

export type WorkerOptions = {
	userConfig: UserConfig;
	dedicated: boolean;
	bridge: BridgeClient<typeof WorkerBridge>;
	id: number;
	cacheWriteDisabled: boolean;
	cacheReadDisabled: boolean;
};

export type WorkerProject = Required<CompilerProject> & {
	configPath: AbsoluteFilePath;
	configCacheKeys: Dict<string>;
};

export type WorkerProjects = Map<number, WorkerProject>;

export type WorkerPartialManifest = {
	path: AbsoluteFilePath;
	hash: string;
	type: Manifest["type"];
};

export type WorkerPartialManifests = {
	id: number;
	manifest: undefined | WorkerPartialManifest;
}[];

// Omit analyze value as the worker will fetch it itself, skips sending over a large payload that it already has in memory
export type WorkerCompilerOptions = {
	bundle?: WorkerBundleCompileOptions;
};

export type WorkerBundleCompileOptions = Omit<BundleCompileOptions, "analyze">;

export type CachedWrapper<T> = {
	value: T;
	integrity: undefined | DiagnosticIntegrity;
	cached: boolean;
};

export type WorkerAnalyzeDependencyResult = CachedWrapper<AnalyzeDependencyResult>;

export type WorkerCompileResult = CachedWrapper<CompileResult>;

export type WorkerLintOptions = {
	compilerOptions?: LintCompilerOptions;
	prefetchedModuleSignatures: WorkerPrefetchedModuleSignatures;
	applySafeFixes: boolean;
	suppressionExplanation?: string;
	save: boolean;
};

export type WorkerFormatOptions = Omit<FormatterOptions, "projectConfig"> & {
	forceFormat?: boolean;
};

export type WorkerParseOptions = {
	sourceTypeJS?: ConstJSSourceType;
	cache?: boolean;
	allowParserDiagnostics?: boolean;
	allowCorrupt?: boolean;
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

export type WorkerPrefetchedModuleSignatures = {
	[key: string]:
		| {
				type: "USE_CACHED";
				path: AnyPath;
			}
		| {
				type: "RESOLVED";
				graph: ModuleSignature;
			}
		| {
				type: "OWNED";
				ref: FileReference;
			}
		| {
				type: "POINTER";
				key: string;
			};
};

export type WorkerFormatResult = {
	original: string;
	formatted: string;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
};

export type WorkerLintTimings = {
	eslint: bigint;
	prettier: bigint;
};

export const EMPTY_LINT_TIMINGS: WorkerLintTimings ={
  eslint: 0n,
  prettier: 0n,
};

export type WorkerLintResult = {
	save: undefined | RecoverySaveFile;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
	timingsNs: WorkerLintTimings;
};

export type WorkerBufferPosition = {
	line: Number0;
	character: Number0;
};

export type WorkerBufferPatch = {
	range: {
		start: WorkerBufferPosition;
		end: WorkerBufferPosition;
	};
	text: string;
};

export type WorkerUpdateInlineSnapshotResult = {
	diagnostics: Diagnostics;
	file: undefined | RecoverySaveFile;
};