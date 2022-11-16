// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName;
	path: RomePath;
}
export type FeatureName = "Format" | "Lint";
export interface RomePath {
	id: FileId;
	path: string;
}
/**
 * An id that points into a file database.
 */
export type FileId = number;
export interface SupportsFeatureResult {
	reason?: UnsupportedReason;
}
export type UnsupportedReason =
	| "Ignored"
	| "FeatureNotEnabled"
	| "FileNotSupported";
export interface UpdateSettingsParams {
	configuration: Configuration;
}
/**
 * The configuration that is contained inside the file `rome.json`
 */
export interface Configuration {
	/**
	 * The configuration of the filesystem
	 */
	files?: FilesConfiguration;
	/**
	 * The configuration of the formatter
	 */
	formatter?: FormatterConfiguration;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: JavascriptConfiguration;
	/**
	 * The configuration for the linter
	 */
	linter?: LinterConfiguration;
}
/**
 * The configuration of the filesystem
 */
export interface FilesConfiguration {
	/**
	 * A list of Unix shell style patterns. Rome tools will ignore files/folders that will match these patterns.
	 */
	ignore?: string[];
	/**
	 * The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reason. Defaults to 1 MiB
	 */
	maxSize?: number;
}
export interface FormatterConfiguration {
	enabled?: boolean;
	/**
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: string[];
	/**
	 * The size of the indentation, 2 by default
	 */
	indentSize?: number;
	/**
	 * The indent style.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
export interface JavascriptConfiguration {
	formatter?: JavascriptFormatter;
	/**
	* A list of global bindings that should be ignored by the analyzers

If defined here, they should not emit diagnostics. 
	 */
	globals?: string[];
}
export interface LinterConfiguration {
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: string[];
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export type PlainIndentStyle = "tab" | "space";
/**
	* Validated value for the `line_width` formatter options

The allowed range of values is 1..=320 
	 */
export type LineWidth = number;
export interface JavascriptFormatter {
	/**
	 * When properties in objects are quoted. Defaults to asNeeded.
	 */
	quoteProperties?: QuoteProperties;
	/**
	 * The style for quotes. Defaults to double.
	 */
	quoteStyle?: QuoteStyle;
	/**
	 * Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
	 */
	semicolons?: Semicolons;
	/**
	 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
	 */
	trailingComma?: TrailingComma;
}
export interface Rules {
	a11y?: A11y;
	complexity?: Complexity;
	correctness?: Correctness;
	nursery?: Nursery;
	/**
	 * It enables the lint rules recommended by Rome. `true` by default.
	 */
	recommended?: boolean;
	security?: Security;
	style?: Style;
}
export type QuoteProperties = "asNeeded" | "preserve";
export type QuoteStyle = "double" | "single";
export type Semicolons = "always" | "asNeeded";
export type TrailingComma = "all" | "es5" | "none";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	noAutofocus?: RuleConfiguration;
	noPositiveTabindex?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useAltText?: RuleConfiguration;
	useAnchorContent?: RuleConfiguration;
	useBlankTarget?: RuleConfiguration;
	useButtonType?: RuleConfiguration;
	useKeyWithClickEvents?: RuleConfiguration;
	useKeyWithMouseEvents?: RuleConfiguration;
	useValidAnchor?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	noExtraBooleanCast?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useSimplifiedLogicExpression?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	noArguments?: RuleConfiguration;
	noArrayIndexKey?: RuleConfiguration;
	noAsyncPromiseExecutor?: RuleConfiguration;
	noCatchAssign?: RuleConfiguration;
	noChildrenProp?: RuleConfiguration;
	noCommentText?: RuleConfiguration;
	noCompareNegZero?: RuleConfiguration;
	noDebugger?: RuleConfiguration;
	noDelete?: RuleConfiguration;
	noDoubleEquals?: RuleConfiguration;
	noDupeArgs?: RuleConfiguration;
	noEmptyPattern?: RuleConfiguration;
	noFunctionAssign?: RuleConfiguration;
	noImportAssign?: RuleConfiguration;
	noLabelVar?: RuleConfiguration;
	noMultipleSpacesInRegularExpressionLiterals?: RuleConfiguration;
	noNewSymbol?: RuleConfiguration;
	noRenderReturnValue?: RuleConfiguration;
	noRestrictedGlobals?: RuleConfiguration;
	noShadowRestrictedNames?: RuleConfiguration;
	noSparseArray?: RuleConfiguration;
	noUndeclaredVariables?: RuleConfiguration;
	noUnnecessaryContinue?: RuleConfiguration;
	noUnreachable?: RuleConfiguration;
	noUnsafeNegation?: RuleConfiguration;
	noUnusedVariables?: RuleConfiguration;
	noUselessFragments?: RuleConfiguration;
	noVoidElementsWithChildren?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useSingleCaseStatement?: RuleConfiguration;
	useValidTypeof?: RuleConfiguration;
	useWhile?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	noBannedTypes?: RuleConfiguration;
	noConditionalAssignment?: RuleConfiguration;
	noConstAssign?: RuleConfiguration;
	noDupeKeys?: RuleConfiguration;
	noExplicitAny?: RuleConfiguration;
	noInvalidConstructorSuper?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useCamelCase?: RuleConfiguration;
	useConst?: RuleConfiguration;
	useExhaustiveDependencies?: RuleConfiguration;
	useFlatMap?: RuleConfiguration;
	useNumericLiterals?: RuleConfiguration;
	useValidForDirection?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	noDangerouslySetInnerHtml?: RuleConfiguration;
	noDangerouslySetInnerHtmlWithChildren?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
}
/**
 * A list of rules that belong to this group
 */
export interface Style {
	noImplicitBoolean?: RuleConfiguration;
	noNegationElse?: RuleConfiguration;
	noShoutyConstants?: RuleConfiguration;
	noUnusedTemplateLiteral?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useBlockStatements?: RuleConfiguration;
	useFragmentSyntax?: RuleConfiguration;
	useOptionalChain?: RuleConfiguration;
	useSelfClosingElements?: RuleConfiguration;
	useShorthandArrayType?: RuleConfiguration;
	useSingleVarDeclarator?: RuleConfiguration;
	useTemplate?: RuleConfiguration;
}
export type RuleConfiguration = RulePlainConfiguration | RuleWithOptions;
export type RulePlainConfiguration = "warn" | "error" | "off";
export interface RuleWithOptions {
	level: RulePlainConfiguration;
	options: any;
}
export interface OpenFileParams {
	content: string;
	language_hint?: Language;
	path: RomePath;
	version: number;
}
/**
 * Supported languages by Rome
 */
export type Language =
	| "JavaScript"
	| "JavaScriptReact"
	| "TypeScript"
	| "TypeScriptReact"
	| "Json"
	| "Unknown";
export interface ChangeFileParams {
	content: string;
	path: RomePath;
	version: number;
}
export interface CloseFileParams {
	path: RomePath;
}
export interface GetSyntaxTreeParams {
	path: RomePath;
}
export interface GetSyntaxTreeResult {
	ast: string;
	cst: string;
}
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: RomePath;
}
export type TextSize = number;
export interface GetFormatterIRParams {
	path: RomePath;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	max_diagnostics: number;
	path: RomePath;
}
export type RuleCategories = RuleCategory[];
export type RuleCategory = "Syntax" | "Lint" | "Action";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
	errors: number;
	skipped_diagnostics: number;
}
/**
 * Serializable representation for a [Diagnostic](super::Diagnostic).
 */
export interface Diagnostic {
	advices: Advices;
	category?: Category;
	description: string;
	location?: Location;
	message: MarkupBuf;
	severity: Severity;
	source?: Diagnostic;
	tags: DiagnosticTags;
	verbose_advices: Advices;
}
/**
 * Implementation of [Visitor] collecting serializable [Advice] into a vector.
 */
export interface Advices {
	advices: Advice[];
}
export type Category =
	| "lint/correctness/noArguments"
	| "lint/correctness/noAsyncPromiseExecutor"
	| "lint/correctness/noCatchAssign"
	| "lint/correctness/noCommentText"
	| "lint/correctness/noCompareNegZero"
	| "lint/correctness/noDelete"
	| "lint/correctness/noDoubleEquals"
	| "lint/correctness/noDupeArgs"
	| "lint/correctness/noEmptyPattern"
	| "lint/correctness/noFunctionAssign"
	| "lint/correctness/noImportAssign"
	| "lint/correctness/noLabelVar"
	| "lint/correctness/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/correctness/noShadowRestrictedNames"
	| "lint/correctness/noSparseArray"
	| "lint/correctness/noUnnecessaryContinue"
	| "lint/correctness/noUnsafeNegation"
	| "lint/correctness/useSingleCaseStatement"
	| "lint/correctness/useWhile"
	| "lint/correctness/noNewSymbol"
	| "lint/correctness/noUselessFragments"
	| "lint/correctness/noUnusedVariables"
	| "lint/correctness/noUnreachable"
	| "lint/correctness/noRestrictedGlobals"
	| "lint/correctness/noUndeclaredVariables"
	| "lint/correctness/useValidTypeof"
	| "lint/correctness/noVoidElementsWithChildren"
	| "lint/correctness/noArrayIndexKey"
	| "lint/correctness/noChildrenProp"
	| "lint/correctness/noRenderReturnValue"
	| "lint/correctness/noDebugger"
	| "lint/style/noNegationElse"
	| "lint/style/noShoutyConstants"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "lint/style/useFragmentSyntax"
	| "lint/style/useTemplate"
	| "lint/style/useSingleVarDeclarator"
	| "lint/style/useOptionalChain"
	| "lint/style/useBlockStatements"
	| "lint/style/noImplicitBoolean"
	| "lint/style/noUnusedTemplateLiteral"
	| "lint/complexity/useSimplifiedLogicExpression"
	| "lint/complexity/noExtraBooleanCast"
	| "lint/a11y/noAutofocus"
	| "lint/a11y/noPositiveTabindex"
	| "lint/a11y/useKeyWithMouseEvents"
	| "lint/a11y/useAnchorContent"
	| "lint/a11y/useBlankTarget"
	| "lint/a11y/useValidAnchor"
	| "lint/a11y/useKeyWithClickEvents"
	| "lint/a11y/useButtonType"
	| "lint/a11y/useAltText"
	| "lint/security/noDangerouslySetInnerHtml"
	| "lint/security/noDangerouslySetInnerHtmlWithChildren"
	| "lint/nursery/noBannedTypes"
	| "lint/nursery/noConditionalAssignment"
	| "lint/nursery/noConstAssign"
	| "lint/nursery/noDupeKeys"
	| "lint/nursery/noExplicitAny"
	| "lint/nursery/noInvalidConstructorSuper"
	| "lint/nursery/useCamelCase"
	| "lint/nursery/useConst"
	| "lint/nursery/useExhaustiveDependencies"
	| "lint/nursery/useFlatMap"
	| "lint/nursery/useNumericLiterals"
	| "lint/nursery/useValidForDirection"
	| "files/missingHandler"
	| "format"
	| "internalError/io"
	| "internalError/fs"
	| "internalError/panic"
	| "lint"
	| "parse"
	| "parse/noSuperWithoutExtends"
	| "suppressions/unknownGroup"
	| "suppressions/unknownRule"
	| "suppressions/unused"
	| "args/fileNotFound"
	| "flags/invalid"
	| "semanticTests";
export interface Location {
	path: Resource_for_String;
	source_code?: string;
	span?: TextRange;
}
export type MarkupBuf = MarkupNodeBuf[];
/**
 * The severity to associate to a diagnostic.
 */
export type Severity = "Fatal" | "Error" | "Warning" | "Information" | "Hint";
export type DiagnosticTags = DiagnosticTag[];
/**
	* Serializable representation of a [Diagnostic](super::Diagnostic) advice

See the [Visitor] trait for additional documentation on all the supported advice types. 
	 */
export type Advice =
	| { Log: [LogCategory, MarkupBuf] }
	| { List: MarkupBuf[] }
	| { Frame: Location }
	| { Diff: TextEdit }
	| { Backtrace: [MarkupBuf, Backtrace] }
	| { Command: string }
	| { Group: [MarkupBuf, Advices] };
/**
 * Represents the resource a diagnostic is associated with.
 */
export type Resource_for_String =
	| "Argv"
	| "Memory"
	| { File: FilePath_for_String };
export type TextRange = [TextSize, TextSize];
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
 * Internal enum used to automatically generate bit offsets for [DiagnosticTags] and help with the implementation of `serde` and `schemars` for tags.
 */
export type DiagnosticTag =
	| "Fixable"
	| "Internal"
	| "UnnecessaryCode"
	| "DeprecatedCode";
/**
 * The category for a log advice, defines how the message should be presented to the user.
 */
export type LogCategory = "None" | "Info" | "Warn" | "Error";
export interface TextEdit {
	dictionary: string;
	ops: CompressedOp[];
}
export type Backtrace = BacktraceFrame[];
/**
 * Represents the path of a file on the filesystem.
 */
export type FilePath_for_String =
	| { Path: string }
	| { FileId: FileId }
	| { PathAndId: { file_id: FileId; path: string } };
/**
 * Enumeration of all the supported markup elements
 */
export type MarkupElement =
	| "Emphasis"
	| "Dim"
	| "Italic"
	| "Underline"
	| "Error"
	| "Success"
	| "Warn"
	| "Info"
	| "Inverse"
	| { Hyperlink: { href: string } };
export type CompressedOp =
	| { DiffOp: DiffOp }
	| { EqualLines: { line_count: number } };
/**
 * Serializable representation of a backtrace frame.
 */
export interface BacktraceFrame {
	ip: number;
	symbols: BacktraceSymbol[];
}
export type DiffOp =
	| { Equal: { range: TextRange } }
	| { Insert: { range: TextRange } }
	| { Delete: { range: TextRange } };
/**
 * Serializable representation of a backtrace frame symbol.
 */
export interface BacktraceSymbol {
	colno?: number;
	filename?: string;
	lineno?: number;
	name?: string;
}
export interface PullActionsParams {
	path: RomePath;
	range: TextRange;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	group_name: string;
	rule_name: string;
	suggestion: CodeSuggestion;
}
/**
	* The category of a code action, this type maps directly to the [CodeActionKind] type in the Language Server Protocol specification

[CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind 
	 */
export type ActionCategory =
	| "QuickFix"
	| { Refactor: RefactorKind }
	| { Source: SourceActionKind }
	| { Other: string };
/**
 * A Suggestion that is provided by rslint, and can be reported to the user, and can be automatically applied if it has the right [`Applicability`].
 */
export interface CodeSuggestion {
	applicability: Applicability;
	labels: TextRange[];
	msg: MarkupBuf;
	/**
	 * If the `FileId` is `None`, it's in the same file as his parent.
	 */
	span: FileSpan;
	suggestion: TextEdit;
}
/**
 * The sub-category of a refactor code action
 */
export type RefactorKind =
	| "None"
	| "Extract"
	| "Inline"
	| "Rewrite"
	| { Other: string };
/**
 * The sub-category of a source code action
 */
export type SourceActionKind =
	| "None"
	| "FixAll"
	| "OrganizeImports"
	| { Other: string };
/**
 * Indicates how a tool should manage this suggestion.
 */
export type Applicability = "Always" | "MaybeIncorrect";
/**
 * A range that is indexed in a specific file.
 */
export interface FileSpan {
	file: FileId;
	range: TextRange;
}
export interface FormatFileParams {
	path: RomePath;
}
export interface Printed {
	code: string;
	range?: TextRange;
	sourcemap: SourceMarker[];
	verbatim_ranges: TextRange[];
}
/**
 * Lightweight sourcemap marker between source and output tokens
 */
export interface SourceMarker {
	/**
	 * Position of the marker in the output code
	 */
	dest: TextSize;
	/**
	 * Position of the marker in the original source
	 */
	source: TextSize;
}
export interface FormatRangeParams {
	path: RomePath;
	range: TextRange;
}
export interface FormatOnTypeParams {
	offset: TextSize;
	path: RomePath;
}
export interface FixFileParams {
	fix_file_mode: FixFileMode;
	path: RomePath;
}
/**
 * Which fixes should be applied during the analyzing phase
 */
export type FixFileMode = "SafeFixes" | "SafeAndSuggestedFixes";
export interface FixFileResult {
	/**
	 * List of all the code actions applied to the file
	 */
	actions: FixAction[];
	/**
	 * New source code for the file with all fixes applied
	 */
	code: string;
	/**
	 * number of skipped suggested fixes
	 */
	skipped_suggested_fixes: number;
}
export interface FixAction {
	group_name: string;
	/**
	 * Source range at which this action was applied
	 */
	range: TextRange;
	/**
	 * Name of the rule that emitted this code action
	 */
	rule_name: string;
}
export interface RenameParams {
	new_name: string;
	path: RomePath;
	symbol_at: TextSize;
}
export interface RenameResult {
	/**
	 * List of text edit operations to apply on the source code
	 */
	indels: TextEdit;
	/**
	 * Range of source code modified by this rename operation
	 */
	range: TextRange;
}
export interface Workspace {
	supportsFeature(
		params: SupportsFeatureParams,
	): Promise<SupportsFeatureResult>;
	updateSettings(params: UpdateSettingsParams): Promise<void>;
	openFile(params: OpenFileParams): Promise<void>;
	changeFile(params: ChangeFileParams): Promise<void>;
	closeFile(params: CloseFileParams): Promise<void>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	getControlFlowGraph(params: GetControlFlowGraphParams): Promise<string>;
	getFormatterIr(params: GetFormatterIRParams): Promise<string>;
	pullDiagnostics(
		params: PullDiagnosticsParams,
	): Promise<PullDiagnosticsResult>;
	pullActions(params: PullActionsParams): Promise<PullActionsResult>;
	formatFile(params: FormatFileParams): Promise<Printed>;
	formatRange(params: FormatRangeParams): Promise<Printed>;
	formatOnType(params: FormatOnTypeParams): Promise<Printed>;
	fixFile(params: FixFileParams): Promise<FixFileResult>;
	rename(params: RenameParams): Promise<RenameResult>;
	destroy(): void;
}
export function createWorkspace(transport: Transport): Workspace {
	return {
		supportsFeature(params) {
			return transport.request("rome/supports_feature", params);
		},
		updateSettings(params) {
			return transport.request("rome/update_settings", params);
		},
		openFile(params) {
			return transport.request("rome/open_file", params);
		},
		changeFile(params) {
			return transport.request("rome/change_file", params);
		},
		closeFile(params) {
			return transport.request("rome/close_file", params);
		},
		getSyntaxTree(params) {
			return transport.request("rome/get_syntax_tree", params);
		},
		getControlFlowGraph(params) {
			return transport.request("rome/get_control_flow_graph", params);
		},
		getFormatterIr(params) {
			return transport.request("rome/get_formatter_ir", params);
		},
		pullDiagnostics(params) {
			return transport.request("rome/pull_diagnostics", params);
		},
		pullActions(params) {
			return transport.request("rome/pull_actions", params);
		},
		formatFile(params) {
			return transport.request("rome/format_file", params);
		},
		formatRange(params) {
			return transport.request("rome/format_range", params);
		},
		formatOnType(params) {
			return transport.request("rome/format_on_type", params);
		},
		fixFile(params) {
			return transport.request("rome/fix_file", params);
		},
		rename(params) {
			return transport.request("rome/rename", params);
		},
		destroy() {
			transport.destroy();
		},
	};
}
