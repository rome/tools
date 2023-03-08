// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName;
	path: RomePath;
}
export type FeatureName = "Format" | "Lint";
export interface RomePath {
	path: string;
}
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
	 * A field for the [JSON schema](https://json-schema.org/) specification
	 */
	$schema?: string;
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
	/**
	 * The configuration of the formatter
	 */
	organizeImports?: OrganizeImports;
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
	organize_imports?: JavascriptOrganizeImports;
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
export interface OrganizeImports {
	/**
	 * Enables the organization of imports
	 */
	enabled: boolean;
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
export interface JavascriptOrganizeImports {}
export interface Rules {
	a11y?: A11y;
	/**
	 * It enables ALL rules. The rules that belong to `nursery` won't be enabled.
	 */
	all?: boolean;
	complexity?: Complexity;
	correctness?: Correctness;
	nursery?: Nursery;
	performance?: Performance;
	/**
	 * It enables the lint rules recommended by Rome. `true` by default.
	 */
	recommended?: boolean;
	security?: Security;
	style?: Style;
	suspicious?: Suspicious;
}
export type QuoteProperties = "asNeeded" | "preserve";
export type QuoteStyle = "double" | "single";
export type Semicolons = "always" | "asNeeded";
export type TrailingComma = "all" | "es5" | "none";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Enforce that the accessKey attribute is not used on any HTML element.
	 */
	noAccessKey?: RuleConfiguration;
	/**
	 * Avoid the autoFocus attribute
	 */
	noAutofocus?: RuleConfiguration;
	/**
	 * Disallow target="_blank" attribute without rel="noreferrer"
	 */
	noBlankTarget?: RuleConfiguration;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: RuleConfiguration;
	/**
	 * Check that the scope attribute is only used on th elements.
	 */
	noHeaderScope?: RuleConfiguration;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image.
	 */
	useAltText?: RuleConfiguration;
	/**
	 * Enforce that anchor elements have content and that the content is accessible to screen readers.
	 */
	useAnchorContent?: RuleConfiguration;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: RuleConfiguration;
	/**
	 * Enforce that html element has lang attribute. This allows users to choose a language other than the default.
	 */
	useHtmlLang?: RuleConfiguration;
	/**
	 * Enforce to have the onClick mouse event with the onKeyUp, the onKeyDown, or the onKeyPress keyboard event.
	 */
	useKeyWithClickEvents?: RuleConfiguration;
	/**
	 * Enforce that onMouseOver/onMouseOut are accompanied by onFocus/onBlur for keyboard-only users. It is important to take into account users with physical disabilities who cannot use a mouse, who use assistive technology or screenreader.
	 */
	useKeyWithMouseEvents?: RuleConfiguration;
	/**
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: RuleConfiguration;
	/**
	 * Disallow unclear usage of multiple space characters in regular expression literals
	 */
	noMultipleSpacesInRegularExpressionLiterals?: RuleConfiguration;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: RuleConfiguration;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: RuleConfiguration;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Prevent passing of children as props.
	 */
	noChildrenProp?: RuleConfiguration;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: RuleConfiguration;
	/**
	 * Disallow returning a value from a constructor.
	 */
	noConstructorReturn?: RuleConfiguration;
	/**
	 * Disallows empty destructuring patterns.
	 */
	noEmptyPattern?: RuleConfiguration;
	/**
	 * Disallow new operators with the Symbol object
	 */
	noNewSymbol?: RuleConfiguration;
	/**
	 * Disallow literal numbers that lose precision
	 */
	noPrecisionLoss?: RuleConfiguration;
	/**
	 * Prevent the usage of the return value of React.render.
	 */
	noRenderReturnValue?: RuleConfiguration;
	/**
	 * Disallow returning a value from a setter
	 */
	noSetterReturn?: RuleConfiguration;
	/**
	 * Disallow comparison of expressions modifying the string case with non-compliant value.
	 */
	noStringCaseMismatch?: RuleConfiguration;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document
	 */
	noUndeclaredVariables?: RuleConfiguration;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUnnecessaryContinue?: RuleConfiguration;
	/**
	 * Disallow unreachable code
	 */
	noUnreachable?: RuleConfiguration;
	/**
	 * Disallow control flow statements in finally blocks.
	 */
	noUnsafeFinally?: RuleConfiguration;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: RuleConfiguration;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: RuleConfiguration;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce "for" loop update clause moving the counter in the right direction.
	 */
	useValidForDirection?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow assignments in expressions.
	 */
	noAssignInExpressions?: RuleConfiguration;
	/**
	 * Disallow certain types.
	 */
	noBannedTypes?: RuleConfiguration;
	/**
	 * Disallow reassigning class members.
	 */
	noClassAssign?: RuleConfiguration;
	/**
	 * Disallow comma operator.
	 */
	noCommaOperator?: RuleConfiguration;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: RuleConfiguration;
	/**
	 * Disallow duplicate case labels. If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.
	 */
	noDuplicateCase?: RuleConfiguration;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: RuleConfiguration;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: RuleConfiguration;
	/**
	 * Disallow unnecessary labels.
	 */
	noExtraLabels?: RuleConfiguration;
	/**
	 * Typing mistakes and misunderstandings about where semicolons are required can lead to semicolons that are unnecessary. While not technically an error, extra semicolons can cause confusion when reading code.
	 */
	noExtraSemicolons?: RuleConfiguration;
	/**
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: RuleConfiguration;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: RuleConfiguration;
	/**
	 * Disallow function and var declarations in nested blocks.
	 */
	noInnerDeclarations?: RuleConfiguration;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: RuleConfiguration;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: RuleConfiguration;
	/**
	 * Disallow reassigning function parameters.
	 */
	noParameterAssign?: RuleConfiguration;
	/**
	 * Disallow the use of parameter properties in class constructors.
	 */
	noParameterProperties?: RuleConfiguration;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: RuleConfiguration;
	/**
	 * Eliminate variables that have multiple declarations in the same scope.
	 */
	noRedeclaration?: RuleConfiguration;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: RuleConfiguration;
	/**
	 * This rule allows you to specify global variable names that you don’t want to use in your application.
	 */
	noRestrictedGlobals?: RuleConfiguration;
	/**
	 * Disallow assignments where both sides are exactly the same.
	 */
	noSelfAssign?: RuleConfiguration;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: RuleConfiguration;
	/**
	 * Enforces the usage of the title element for the svg element.
	 */
	noSvgWithoutTitle?: RuleConfiguration;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: RuleConfiguration;
	/**
	 * Ensures the super() constructor is called exactly once on every code path in a class constructor before this is accessed if the class has a superclass
	 */
	noUnreachableSuper?: RuleConfiguration;
	/**
	 * Disallow the use of optional chaining in contexts where the undefined value is not allowed.
	 */
	noUnsafeOptionalChaining?: RuleConfiguration;
	/**
	 * Disallow unused labels.
	 */
	noUnusedLabels?: RuleConfiguration;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: RuleConfiguration;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: RuleConfiguration;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: RuleConfiguration;
	/**
	 * Disallow with statements in non-strict contexts.
	 */
	noWith?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce that ARIA state and property values are valid.
	 */
	useAriaPropTypes?: RuleConfiguration;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: RuleConfiguration;
	/**
	 * Enforce camel case naming convention.
	 */
	useCamelCase?: RuleConfiguration;
	/**
	 * Enforce all dependencies are correctly specified.
	 */
	useExhaustiveDependencies?: RuleConfiguration;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: RuleConfiguration;
	/**
	 * Enforces the usage of the attribute title for the element iframe
	 */
	useIframeTitle?: RuleConfiguration;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: RuleConfiguration;
	/**
	 * Enforces that audio and video elements must have a track for captions.
	 */
	useMediaCaption?: RuleConfiguration;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: RuleConfiguration;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: RuleConfiguration;
	/**
	 * Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
	 */
	useValidLang?: RuleConfiguration;
	/**
	 * Require generator functions to contain yield.
	 */
	useYield?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow the use of the delete operator
	 */
	noDelete?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Prevent the usage of dangerous JSX props
	 */
	noDangerouslySetInnerHtml?: RuleConfiguration;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
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
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow the use of arguments
	 */
	noArguments?: RuleConfiguration;
	/**
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: RuleConfiguration;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause
	 */
	noNegationElse?: RuleConfiguration;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: RuleConfiguration;
	/**
	 * Disallow the use of constants which its value is the upper-case version of its name.
	 */
	noShoutyConstants?: RuleConfiguration;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: RuleConfiguration;
	/**
	 * Disallow the use of var
	 */
	noVar?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Requires following curly brace conventions. JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
	 */
	useBlockStatements?: RuleConfiguration;
	/**
	 * Require const declarations for variables that are never reassigned after declared.
	 */
	useConst?: RuleConfiguration;
	/**
	 * Enforce default function parameters and optional parameters to be last.
	 */
	useDefaultParameterLast?: RuleConfiguration;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: RuleConfiguration;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: RuleConfiguration;
	/**
	 * This rule enforces the use of <>...</> over <Fragment>...</Fragment>.
	 */
	useFragmentSyntax?: RuleConfiguration;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: RuleConfiguration;
	/**
	 * Prevent extra closing tags for components without children
	 */
	useSelfClosingElements?: RuleConfiguration;
	/**
	 * When expressing array types, this rule promotes the usage of T[] shorthand instead of Array<T>.
	 */
	useShorthandArrayType?: RuleConfiguration;
	/**
	 * Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.
	 */
	useSingleCaseStatement?: RuleConfiguration;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: RuleConfiguration;
	/**
	 * Template literals are preferred over string concatenation.
	 */
	useTemplate?: RuleConfiguration;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed
	 */
	useWhile?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Discourage the usage of Array index in keys.
	 */
	noArrayIndexKey?: RuleConfiguration;
	/**
	 * Disallows using an async function as a Promise executor.
	 */
	noAsyncPromiseExecutor?: RuleConfiguration;
	/**
	 * Disallow reassigning exceptions in catch clauses.
	 */
	noCatchAssign?: RuleConfiguration;
	/**
	 * Prevent comments from being inserted as text nodes
	 */
	noCommentText?: RuleConfiguration;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: RuleConfiguration;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: RuleConfiguration;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: RuleConfiguration;
	/**
	 * Require the use of === and !==
	 */
	noDoubleEquals?: RuleConfiguration;
	/**
	 * Prevents object literals having more than one property declaration for the same name. If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.
	 */
	noDuplicateObjectKeys?: RuleConfiguration;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: RuleConfiguration;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: RuleConfiguration;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: RuleConfiguration;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: RuleConfiguration;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: RuleConfiguration;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: RuleConfiguration;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: RuleConfiguration;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: RuleConfiguration;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: RuleConfiguration;
	/**
	 * Disallow sparse arrays
	 */
	noSparseArray?: RuleConfiguration;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: RuleConfiguration;
	/**
	 * This rule verifies the result of typeof $expr unary expressions is being compared to valid values, either string literals containing valid type names or other typeof expressions
	 */
	useValidTypeof?: RuleConfiguration;
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
	location: Location;
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
	| "lint/a11y/noAutofocus"
	| "lint/a11y/noBlankTarget"
	| "lint/a11y/noPositiveTabindex"
	| "lint/a11y/useAltText"
	| "lint/a11y/useAnchorContent"
	| "lint/a11y/useButtonType"
	| "lint/a11y/useKeyWithClickEvents"
	| "lint/a11y/useKeyWithMouseEvents"
	| "lint/a11y/useValidAnchor"
	| "lint/a11y/useHtmlLang"
	| "lint/a11y/noDistractingElements"
	| "lint/a11y/noHeaderScope"
	| "lint/a11y/noAccessKey"
	| "lint/complexity/noExtraBooleanCast"
	| "lint/complexity/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/complexity/noUselessFragments"
	| "lint/complexity/useFlatMap"
	| "lint/complexity/useOptionalChain"
	| "lint/complexity/useSimplifiedLogicExpression"
	| "lint/correctness/noChildrenProp"
	| "lint/correctness/noConstAssign"
	| "lint/correctness/noEmptyPattern"
	| "lint/correctness/noNewSymbol"
	| "lint/correctness/noRenderReturnValue"
	| "lint/correctness/noUndeclaredVariables"
	| "lint/correctness/noUnnecessaryContinue"
	| "lint/correctness/noUnreachable"
	| "lint/correctness/noUnusedVariables"
	| "lint/correctness/noVoidElementsWithChildren"
	| "lint/correctness/useValidForDirection"
	| "lint/correctness/noUnsafeFinally"
	| "lint/correctness/noConstructorReturn"
	| "lint/correctness/noPrecisionLoss"
	| "lint/correctness/noVoidTypeReturn"
	| "lint/correctness/noStringCaseMismatch"
	| "lint/correctness/noSetterReturn"
	| "lint/nursery/noAssignInExpressions"
	| "lint/nursery/noWith"
	| "lint/nursery/noExtraSemicolons"
	| "lint/nursery/noBannedTypes"
	| "lint/nursery/noClassAssign"
	| "lint/nursery/noCommaOperator"
	| "lint/nursery/noDuplicateCase"
	| "lint/nursery/noExtraLabels"
	| "lint/nursery/noInferrableTypes"
	| "lint/nursery/noInnerDeclarations"
	| "lint/nursery/noInvalidConstructorSuper"
	| "lint/nursery/noConfusingLabels"
	| "lint/nursery/noParameterProperties"
	| "lint/nursery/noRedundantAlt"
	| "lint/nursery/noRestrictedGlobals"
	| "lint/nursery/noSelfCompare"
	| "lint/nursery/noSelfAssign"
	| "lint/nursery/noSetterReturn"
	| "lint/nursery/noStringCaseMismatch"
	| "lint/nursery/noSwitchDeclarations"
	| "lint/nursery/noUnreachableSuper"
	| "lint/nursery/noUnusedLabels"
	| "lint/nursery/noUselessSwitchCase"
	| "lint/nursery/useAriaPropsForRole"
	| "lint/nursery/useAriaPropTypes"
	| "lint/nursery/useCamelCase"
	| "lint/nursery/useValidLang"
	| "lint/nursery/useValidAriaProps"
	| "lint/nursery/useExhaustiveDependencies"
	| "lint/nursery/useIsNan"
	| "lint/nursery/useMediaCaption"
	| "lint/nursery/useIframeTitle"
	| "lint/nursery/noNoninteractiveElementToInteractiveRole"
	| "lint/nursery/noUselessRename"
	| "lint/nursery/useValidForDirection"
	| "lint/nursery/useHookAtTopLevel"
	| "lint/nursery/noUnsafeOptionalChaining"
	| "lint/nursery/noDuplicateJsxProps"
	| "lint/nursery/noDuplicateClassMembers"
	| "lint/nursery/useYield"
	| "lint/nursery/noGlobalObjectCalls"
	| "lint/nursery/noPrototypeBuiltins"
	| "lint/nursery/noSvgWithoutTitle"
	| "lint/nursery/noUselessCatch"
	| "lint/nursery/noParameterAssign"
	| "lint/nursery/noRedeclaration"
	| "lint/nursery/useNamespaceKeyword"
	| "lint/performance/noDelete"
	| "lint/security/noDangerouslySetInnerHtml"
	| "lint/security/noDangerouslySetInnerHtmlWithChildren"
	| "lint/style/noArguments"
	| "lint/style/noImplicitBoolean"
	| "lint/style/noNegationElse"
	| "lint/style/noShoutyConstants"
	| "lint/style/noUnusedTemplateLiteral"
	| "lint/style/useBlockStatements"
	| "lint/style/useFragmentSyntax"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "lint/style/useSingleCaseStatement"
	| "lint/style/useSingleVarDeclarator"
	| "lint/style/useTemplate"
	| "lint/style/useWhile"
	| "lint/style/useExponentiationOperator"
	| "lint/style/useNumericLiterals"
	| "lint/style/useDefaultParameterLast"
	| "lint/style/useConst"
	| "lint/style/noVar"
	| "lint/style/noNonNullAssertion"
	| "lint/style/useEnumInitializers"
	| "lint/suspicious/noArrayIndexKey"
	| "lint/suspicious/noAsyncPromiseExecutor"
	| "lint/suspicious/noCatchAssign"
	| "lint/suspicious/noCommentText"
	| "lint/suspicious/noCompareNegZero"
	| "lint/suspicious/noDebugger"
	| "lint/suspicious/noDoubleEquals"
	| "lint/suspicious/noDuplicateParameters"
	| "lint/suspicious/noExplicitAny"
	| "lint/suspicious/noFunctionAssign"
	| "lint/suspicious/noImportAssign"
	| "lint/suspicious/noLabelVar"
	| "lint/suspicious/noShadowRestrictedNames"
	| "lint/suspicious/noSparseArray"
	| "lint/suspicious/noUnsafeNegation"
	| "lint/suspicious/useValidTypeof"
	| "lint/suspicious/noEmptyInterface"
	| "lint/suspicious/noExtraNonNullAssertion"
	| "lint/suspicious/noRedundantUseStrict"
	| "lint/suspicious/noConstEnum"
	| "lint/suspicious/useDefaultSwitchClauseLast"
	| "lint/suspicious/noDuplicateObjectKeys"
	| "files/missingHandler"
	| "format"
	| "internalError/io"
	| "internalError/fs"
	| "internalError/panic"
	| "parse"
	| "parse/noSuperWithoutExtends"
	| "parse/noDuplicatePrivateClassMembers"
	| "lint"
	| "lint/a11y"
	| "lint/complexity"
	| "lint/correctness"
	| "lint/nursery"
	| "lint/performance"
	| "lint/security"
	| "lint/style"
	| "lint/suspicious"
	| "lint/configuration"
	| "suppressions/parse"
	| "suppressions/unknownGroup"
	| "suppressions/unknownRule"
	| "suppressions/unused"
	| "suppressions/deprecatedSyntax"
	| "configuration"
	| "args/fileNotFound"
	| "flags/invalid"
	| "semanticTests";
export interface Location {
	path?: Resource_for_String;
	source_code?: string;
	span?: TextRange;
}
export type MarkupBuf = MarkupNodeBuf[];
/**
 * The severity to associate to a diagnostic.
 */
export type Severity = "fatal" | "error" | "warning" | "information" | "hint";
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
export type Resource_for_String = "argv" | "memory" | { file: string };
export type TextRange = [TextSize, TextSize];
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
 * Internal enum used to automatically generate bit offsets for [DiagnosticTags] and help with the implementation of `serde` and `schemars` for tags.
 */
export type DiagnosticTag =
	| "fixable"
	| "internal"
	| "unnecessaryCode"
	| "deprecatedCode";
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
	rule_name?: [string, string];
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
 * A Suggestion that is provided by Rome's linter, and can be reported to the user, and can be automatically applied if it has the right [`Applicability`].
 */
export interface CodeSuggestion {
	applicability: Applicability;
	labels: TextRange[];
	msg: MarkupBuf;
	span: TextRange;
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
	| "FixAll"
	| "None"
	| "OrganizeImports"
	| { Other: string };
/**
 * Indicates how a tool should manage this suggestion.
 */
export type Applicability = "Always" | "MaybeIncorrect";
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
	/**
	 * Source range at which this action was applied
	 */
	range: TextRange;
	/**
	 * Name of the rule group and rule that emitted this code action
	 */
	rule_name?: [string, string];
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
