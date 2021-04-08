import {AnyRoot, ConstJSSourceType} from "@internal/ast";
import {Manifest} from "@internal/codec-js-manifest";
import {
	BundleCompileOptions,
	CompileResult,
	CompilerProject,
	LintCompilerOptions,
} from "@internal/compiler";
import {
	Diagnostic,
	DiagnosticIntegrity,
	DiagnosticLocation,
	DiagnosticSuppression,
} from "@internal/diagnostics";
import {BridgeClient, BridgeServer} from "@internal/events";
import {FormatterOptions} from "@internal/formatter";
import {ModuleSignature} from "@internal/js-analysis";
import {StaticMarkup} from "@internal/markup";
import {Duration, ZeroIndexed} from "@internal/numbers";
import {AbsoluteFilePath, Path} from "@internal/path";
import {Dict, OptionalProps} from "@internal/typescript-helpers";
import WorkerBridge from "../common/bridges/WorkerBridge";
import {AnalyzeDependencyResult} from "../common/types/analyzeDependencies";
import {AssembledBundle} from "../common/types/bundler";
import {FileReference} from "../common/types/files";
import {UserConfig} from "../common/userConfig";
import {RecoverySaveFile} from "../server/fs/RecoveryStore";
import {TestServerRunnerOptions} from "../server/testing/types";
import {FocusedTest} from "./test/TestWorkerFile";
import workerThreads = require("worker_threads");
import {ReporterNamespace} from "@internal/cli-reporter";
import {Resource} from "@internal/resources";

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

export type WorkerType = "test-runner" | "file-processor" | "script-runner";

export type ThreadWorkerContainer = {
	type: WorkerType;
	id: number;
	fileCount: number;
	byteCount: bigint;
	bridge: BridgeServer<typeof WorkerBridge>;
	displayName: string;
	logger: ReporterNamespace;
	thread: {
		worker: workerThreads.Worker;
		resources: Resource;
	};
	// Whether we've completed a handshake with the worker and it's ready to receive requests
	ready: boolean;
	// Whether we should assign files to this worker
	ghost: boolean;
};

export type WorkerContainer = OptionalProps<ThreadWorkerContainer, "thread">;

export type PartialWorkerOptions = {
	type: WorkerType;
	id: number;
	cacheWriteDisabled: boolean;
	cacheReadDisabled: boolean;
	env: Dict<undefined | string>;
	inspectorPort: undefined | number;
};

export type WorkerOptions = PartialWorkerOptions & {
	userConfig: UserConfig;
	dedicated: boolean;
	bridge: BridgeClient<typeof WorkerBridge>;
};

export type WorkerProject = Required<CompilerProject> & {
	configPath: AbsoluteFilePath;
	configCacheKeys: Dict<string>;
};

export type WorkerProjects = Map<number, WorkerProject>;

export type WorkerPartialManifestsTransport = Map<
	number,
	undefined | WorkerPartialManifest
>;

export type WorkerPartialManifest = {
	project: number;
	path: AbsoluteFilePath;
	hash: string;
	type: Manifest["type"];
};

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
				path: Path;
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
	diagnostics: Diagnostic[];
	suppressions: DiagnosticSuppression[];
};

export type WorkerIntegrationTiming = {
	type: "official" | "plugin";
	displayName: StaticMarkup;
	took: Duration;
};

export type WorkerIntegrationTimings = Map<string, WorkerIntegrationTiming>;

export type WorkerLintResult = {
	save: undefined | RecoverySaveFile;
	diagnostics: Diagnostic[];
	suppressions: DiagnosticSuppression[];
	timings: WorkerIntegrationTimings;
};

export type WorkerBufferPosition = {
	line: ZeroIndexed;
	character: ZeroIndexed;
};

export type WorkerBufferPatch = {
	range: {
		start: WorkerBufferPosition;
		end: WorkerBufferPosition;
	};
	text: string;
};

export type WorkerUpdateInlineSnapshotResult = {
	diagnostics: Diagnostic[];
	file: undefined | RecoverySaveFile;
};

export type TestRef = {
	path: AbsoluteFilePath;
	testName: string;
};

export type TestFileRef = OptionalProps<TestRef, "testName">;

export type TestWorkerPrepareTestOptions = {
	partial: boolean;
	path: AbsoluteFilePath;
	contextDirectory: AbsoluteFilePath;
	projectDirectory: string;
	assembled: AssembledBundle;
	cwd: string;
	globalOptions: TestServerRunnerOptions;
	logFound: boolean;
};

export type TestWorkerPrepareTestResult = {
	foundTests: Map<string, DiagnosticLocation>;
	focusedTests: FocusedTest[];
};

export type TestWorkerRunTestOptions = {
	path: AbsoluteFilePath;
	testNames: string[];
};
