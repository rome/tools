import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName;
	path: RomePath;
}
export type FeatureName = "Format" | "Lint";
export interface RomePath {
	id: number;
	path: string;
}
export interface UpdateSettingsParams {
	settings: WorkspaceSettings;
}
export interface WorkspaceSettings {
	format?: FormatSettings;
	languages?: LanguagesSettings;
	linter?: LinterSettings;
}
export interface FormatSettings {
	enabled: boolean;
	format_with_errors: boolean;
	indent_style?: IndentStyle;
	line_width?: LineWidth;
}
export interface LanguagesSettings {
	javascript?: LanguageSettings_for_JsLanguage;
}
export interface LinterSettings {
	enabled: boolean;
	rules?: Rules;
}
export type IndentStyle = "Tab" | { Space: number };
export type LineWidth = number;
export interface LanguageSettings_for_JsLanguage {
	format?: JsFormatSettings;
	globals?: string[];
	linter?: JsLinterSettings;
}
export interface Rules {
	js?: Js;
	jsx?: Jsx;
	recommended?: boolean;
	regex?: Regex;
	ts?: Ts;
}
export interface JsFormatSettings {
	quote_style?: QuoteStyle;
}
export interface JsLinterSettings {
	globals: string[];
}
export interface Js {
	recommended?: boolean;
}
export interface Jsx {
	recommended?: boolean;
}
export interface Regex {
	recommended?: boolean;
}
export interface Ts {
	recommended?: boolean;
}
export type QuoteStyle = "Double" | "Single";
export interface OpenFileParams {
	content: string;
	path: RomePath;
	version: number;
}
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
	cursor: number;
	path: RomePath;
}
export interface GetFormatterIRParams {
	path: RomePath;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	path: RomePath;
}
export type RuleCategories = RuleCategory[];
export type RuleCategory = "Syntax" | "Lint" | "Action";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
}
export interface Diagnostic {
	children: SubDiagnostic[];
	code?: string;
	code_link?: string;
	file_id: number;
	footers: Footer[];
	primary?: SubDiagnostic;
	severity: Severity;
	suggestions: CodeSuggestion[];
	summary?: string;
	tag?: DiagnosticTag;
	title: MarkupBuf;
}
export interface SubDiagnostic {
	msg: MarkupBuf;
	severity: Severity;
	span: FileSpan;
}
export interface Footer {
	msg: MarkupBuf;
	severity: Severity;
}
export type Severity = "Help" | "Note" | "Warning" | "Error" | "Bug";
export interface CodeSuggestion {
	applicability: Applicability;
	labels: TextRangeSchema[];
	msg: MarkupBuf;
	span: FileSpan;
	style: SuggestionStyle;
	substitution: SuggestionChange;
}
export type DiagnosticTag = "Unnecessary" | "Deprecated" | "Both";
export type MarkupBuf = MarkupNodeBuf[];
export interface FileSpan {
	file: number;
	range: TextRangeSchema;
}
export type Applicability =
	| "Always"
	| "MaybeIncorrect"
	| "HasPlaceholders"
	| "Unspecified";
export interface TextRangeSchema {
	end: number;
	start: number;
}
export type SuggestionStyle = "DontShow" | "HideCode" | "Inline" | "Full";
export type SuggestionChange = { Indels: Indel[] } | { String: string };
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
export interface Indel {
	delete: TextRangeSchema;
	insert: string;
}
export type MarkupElement =
	| "Emphasis"
	| "Dim"
	| "Italic"
	| "Underline"
	| "Error"
	| "Success"
	| "Warn"
	| "Info"
	| { Hyperlink: { href: string } };
export interface PullActionsParams {
	path: RomePath;
	range: TextRangeSchema;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	rule_name: string;
	suggestion: CodeSuggestion;
}
export type ActionCategory = "QuickFix" | "Refactor";
export interface FormatFileParams {
	path: RomePath;
}
export interface Printed {
	code: string;
	range?: TextRangeSchema;
	sourcemap: SourceMarker[];
	verbatim_ranges: TextRangeSchema[];
}
export interface SourceMarker {
	dest: number;
	source: number;
}
export interface FormatRangeParams {
	path: RomePath;
	range: TextRangeSchema;
}
export interface FormatOnTypeParams {
	offset: number;
	path: RomePath;
}
export interface FixFileParams {
	fix_file_mode: FixFileMode;
	path: RomePath;
}
export type FixFileMode = "SafeFixes" | "SafeAndSuggestedFixes";
export interface FixFileResult {
	actions: FixAction[];
	code: string;
	skipped_suggested_fixes: number;
}
export interface FixAction {
	range: TextRangeSchema;
	rule_name: string;
}
export interface RenameParams {
	new_name: string;
	path: RomePath;
	symbol_at: number;
}
export interface RenameResult {
	indels: Indel[];
	range: TextRangeSchema;
}
export interface Workspace {
	supports_feature(params: SupportsFeatureParams): Promise<boolean>;
	update_settings(params: UpdateSettingsParams): Promise<void>;
	open_file(params: OpenFileParams): Promise<void>;
	change_file(params: ChangeFileParams): Promise<void>;
	close_file(params: CloseFileParams): Promise<void>;
	get_syntax_tree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	get_control_flow_graph(params: GetControlFlowGraphParams): Promise<string>;
	get_formatter_ir(params: GetFormatterIRParams): Promise<string>;
	pull_diagnostics(
		params: PullDiagnosticsParams,
	): Promise<PullDiagnosticsResult>;
	pull_actions(params: PullActionsParams): Promise<PullActionsResult>;
	format_file(params: FormatFileParams): Promise<Printed>;
	format_range(params: FormatRangeParams): Promise<Printed>;
	format_on_type(params: FormatOnTypeParams): Promise<Printed>;
	fix_file(params: FixFileParams): Promise<FixFileResult>;
	rename(params: RenameParams): Promise<RenameResult>;
	destroy(): void;
}
export function createWorkspace(transport: Transport): Workspace {
	return {
		supports_feature(params) {
			return transport.request("rome/supports_feature", params);
		},
		update_settings(params) {
			return transport.request("rome/update_settings", params);
		},
		open_file(params) {
			return transport.request("rome/open_file", params);
		},
		change_file(params) {
			return transport.request("rome/change_file", params);
		},
		close_file(params) {
			return transport.request("rome/close_file", params);
		},
		get_syntax_tree(params) {
			return transport.request("rome/get_syntax_tree", params);
		},
		get_control_flow_graph(params) {
			return transport.request("rome/get_control_flow_graph", params);
		},
		get_formatter_ir(params) {
			return transport.request("rome/get_formatter_ir", params);
		},
		pull_diagnostics(params) {
			return transport.request("rome/pull_diagnostics", params);
		},
		pull_actions(params) {
			return transport.request("rome/pull_actions", params);
		},
		format_file(params) {
			return transport.request("rome/format_file", params);
		},
		format_range(params) {
			return transport.request("rome/format_range", params);
		},
		format_on_type(params) {
			return transport.request("rome/format_on_type", params);
		},
		fix_file(params) {
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
