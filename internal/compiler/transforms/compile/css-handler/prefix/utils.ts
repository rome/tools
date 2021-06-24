import {
	AnyCSSPattern,
	AnyNode,
	CSSAtRule,
	CSSBlock,
	CSSBlockValue,
	CSSDeclaration,
	CSSPseudoClassSelector,
	CSSPseudoElementSelector,
	CSSPseudoSelector,
	CSSRoot,
	CSSRule,
	CSSSelector,
	MOCK_PARENT,
	cssBlock,
	cssDeclaration,
	cssIdentifier,
	cssPseudoClassSelector,
	cssPseudoElementSelector,
	cssRoot,
	cssRule,
	cssSelector,
} from "@internal/ast";
import {RequiredProps, UnknownObject} from "@internal/typescript-helpers";
import {
	CompilerPath,
	TypedVisitor,
	Visitor,
	signals,
	transformVisitor,
} from "@internal/compiler";
import {Browser} from "@internal/browser-features/Browser";
import {ProjectConfig} from "@internal/project";
import {getBrowser} from "@internal/browser-features";
import {Builder} from "@internal/formatter";
import {printTokenToString} from "@internal/formatter/Printer";

export type CompilerPrefixerState = UnknownObject;

export function findPropertyIndex(
	node: CSSBlock,
	property: string,
): [number, CSSDeclaration | undefined] {
	if (node.value !== undefined) {
		const index = node.value.findIndex((n) =>
			n.type === "CSSDeclaration" && n.name === property
		);
		return [index, node.value[index] as CSSDeclaration | undefined];
	}
	return [-1, undefined];
}

export function findPropertyValueIndex(
	node: CSSBlock,
	property: string,
	value: string,
): [number, CSSDeclaration | undefined] {
	if (node.value !== undefined) {
		const index = node.value.findIndex((n) =>
			n.type === "CSSDeclaration" &&
			n.name === property &&
			n.value.length === 1 &&
			n.value[0]?.type === "CSSIdentifier" &&
			n.value[0].value === value
		);
		return [index, node.value[index] as CSSDeclaration | undefined];
	}
	return [-1, undefined];
}

type PrefixCSSBlockCompilerPath = CompilerPath & {
	node: RequiredProps<CSSBlock, "value">;
};

function isCssBlockAndHasValue(
	path: CompilerPath,
): path is PrefixCSSBlockCompilerPath {
	return (
		path.node.type === "CSSBlock" &&
		path.node.value !== undefined &&
		path.node.value.length > 0
	);
}

export interface PrefixCSSBlockVisitor extends TypedVisitor<
	CompilerPrefixerState,
	PrefixCSSBlockCompilerPath
> {}

export function createPrefixCSSBlockVisitor(
	visitor: PrefixCSSBlockVisitor,
): Visitor<CompilerPrefixerState> {
	return transformVisitor("css-handler/prefix", visitor, isCssBlockAndHasValue);
}

interface PrefixCSSPropertyProps {
	path: PrefixCSSBlockCompilerPath;
	propertyName: string;
	browserFeaturesKey: string;
	rename?: (propertyName: string) => string;
}

interface PrefixCSSValueProps extends PrefixCSSPropertyProps {
	value: string;
	rename?: (value: string) => string;
}

export function prefixCSSProperty(
	{
		path,
		propertyName,
		browserFeaturesKey,
		rename = (propertyName) => propertyName,
	}: PrefixCSSPropertyProps,
) {
	const {node} = path;
	const [propertyIndex, property] = findPropertyIndex(node, propertyName);
	if (property !== undefined) {
		const newDeclarations = [];
		const prefixes = purgePrefixes(
			getPrefixes(getTargets(path), browserFeaturesKey),
			path,
		);

		for (const prefix of prefixes) {
			const hasPrefix =
				findPropertyIndex(node, rename(`-${prefix}-${propertyName}`))[0] !== -1;
			if (!hasPrefix) {
				newDeclarations.push(
					cssDeclaration.create({
						name: rename(`-${prefix}-${property.name}`),
						value: property.value,
						important: property.important,
					}),
				);
			}
		}
		if (newDeclarations.length > 0) {
			const block = cssBlock.create({
				...node,
				value: [
					...node.value.slice(0, propertyIndex),
					...newDeclarations,
					property,
					...node.value.slice(propertyIndex + 1, node.value.length),
				],
			});
			return signals.replace(block);
		}
	}

	return signals.retain;
}

export function prefixCSSValue(
	{
		path,
		propertyName,
		value,
		browserFeaturesKey,
		rename = (value) => value,
	}: PrefixCSSValueProps,
) {
	const {node} = path;

	const [propertyIndex, property] = findPropertyValueIndex(
		node,
		propertyName,
		value,
	);

	if (property !== undefined) {
		const newDeclarations = [];
		const prefixes = purgePrefixes(
			getPrefixes(getTargets(path), browserFeaturesKey),
			path,
		);

		for (const prefix of prefixes) {
			const hasPrefix =
				findPropertyValueIndex(
					node,
					propertyName,
					rename(`-${prefix}-${value}`),
				)[0] !== -1;
			if (!hasPrefix) {
				newDeclarations.push(
					cssDeclaration.create({
						name: property.name,
						value: [
							cssIdentifier.create({value: rename(`-${prefix}-${value}`)}),
						],
						important: property.important,
					}),
				);
			}
		}
		if (newDeclarations.length > 0) {
			const block = cssBlock.create({
				...node,
				value: [
					...node.value.slice(0, propertyIndex),
					...newDeclarations,
					property,
					...node.value.slice(propertyIndex + 1, node.value.length),
				],
			});
			return signals.replace(block);
		}
	}

	return signals.retain;
}

// Regular expression for matching prefixes
const PREFIX_REG_EXPR = /-[a-z]+-/g;

// Regular expression for matching just the prefix name
const PREFIX_NAME_REG_EXPR = /-([a-z]+)-/;

/**
 * Restrict the prefixes in the block to the prefixes
 * present in the rule's prelude (if it's the case).
 */
function purgePrefixes(prefixes: Set<string>, path: PrefixCSSBlockCompilerPath) {
	if (isCSSRule(path.parent)) {
		const allowedPrefixes = collectExistingPrefixesFromRule(path.parent);
		if (allowedPrefixes.size > 0) {
			return new Set([...prefixes].filter((x) => allowedPrefixes.has(x)));
		}
	}
	return prefixes;
}

function collectExistingPrefixesFromRule(rule: CSSRule) {
	const existingPrefixes = new Set<string>();
	for (const selector of rule.prelude) {
		for (const pattern of selector.patterns) {
			if (isPseudoSelector(pattern)) {
				const name = getPrefixName(pattern.value);
				if (name !== undefined) {
					existingPrefixes.add(name);
				}
			}
		}
	}
	return existingPrefixes;
}

function getPrefixName(value: string) {
	const matched = value.match(PREFIX_NAME_REG_EXPR);
	if (matched === null) {
		return undefined;
	}
	return matched[1];
}

export type PrefixCSSRootCompilerPath = CompilerPath & {
	node: CSSRoot;
};

export interface PrefixCSSRootVisitor extends TypedVisitor<
	CompilerPrefixerState,
	PrefixCSSRootCompilerPath
> {}

export function createPrefixCSSRootVisitor(
	visitor: PrefixCSSRootVisitor,
): Visitor<CompilerPrefixerState> {
	return transformVisitor(
		"css-handler/prefix",
		visitor,
		(path): path is PrefixCSSRootCompilerPath => path.node.type === "CSSRoot",
	);
}

interface PrefixPseudoSelectorsInRootProps {
	path: PrefixCSSRootCompilerPath;
	namesToFeatures: Map<string, string>;
}

/**
 * Iterate over all rules in the root and prefix their
 * pseudo-selectors if it is the case. The root
 * is replaced only if we prefixed at least one
 * rule.
 */
export function prefixPseudoSelectorsInRoot(
	{path, namesToFeatures}: PrefixPseudoSelectorsInRootProps,
): signals.EnterSignal {
	const hasRules = path.node.body.some(isCSSRule);
	if (!hasRules) {
		return signals.retain;
	}

	const targets = getTargets(path);

	// The cast was necessary for a generic implementation
	// Templates are not flexible enough in this case
	const newBody = prefixRulesInArray(path.node.body, namesToFeatures, targets) as (
		| CSSRule
		| CSSAtRule)[];

	if (newBody.length > path.node.body.length) {
		const newRoot = cssRoot.create({
			...path.node,
			body: newBody,
		});
		return signals.replace(newRoot);
	}
	return signals.retain;
}

interface PrefixPseudoSelectorsInBlockProps {
	path: PrefixCSSBlockCompilerPath;
	namesToFeatures: Map<string, string>;
}

/** {@link prefixPseudoSelectorsInRoot}*/
export function prefixPseudoSelectorsInBlock(
	{path, namesToFeatures}: PrefixPseudoSelectorsInBlockProps,
): signals.EnterSignal {
	const hasRules = path.node.value.some(isCSSRule);
	if (!hasRules) {
		return signals.retain;
	}

	const targets = getTargets(path);
	const newValue = prefixRulesInArray(path.node.value, namesToFeatures, targets) as CSSBlockValue;

	if (newValue.length > path.node.value.length) {
		const newBlock = cssBlock.create({
			...path.node,
			value: newValue,
			loc: path.node.loc,
		});
		return signals.replace(newBlock);
	}
	return signals.retain;
}

interface RuleOccurrence {
	original: boolean;
	prefixed: boolean;
}

/**
 * Prefixes any rule with pseudo-selectors if it is the case.
 * It detects if a rule already has a prefixed version or if it
 * is prefixed itself and ignores it.
 */
function prefixRulesInArray<T extends AnyNode>(
	array: (T | CSSRule)[],
	namesToFeatures: Map<string, string>,
	targets: Browser[],
): (T | CSSRule)[] {
	const result: (T | CSSRule)[] = [];
	const stringCache = new WeakMap<CSSRule, string>();
	const occurrenceCache = new Map<string, RuleOccurrence>();

	// build the caches for rules' occurrences
	for (const node of array) {
		if (isCSSRule(node)) {
			// convert the rule prelude to a string
			const {text, prefixed} = rulePreludeToString(node);
			stringCache.set(node, text);

			if (!occurrenceCache.has(text)) {
				occurrenceCache.set(text, {original: false, prefixed: false});
			}

			// remember if it is an "original" rule or
			// a "prefixed" one
			const occurrence = occurrenceCache.get(text)!;
			if (prefixed && !occurrence.prefixed) {
				occurrenceCache.set(
					text,
					{
						...occurrence,
						prefixed: true,
					},
				);
			} else if (!(prefixed || occurrence.original)) {
				occurrenceCache.set(
					text,
					{
						...occurrence,
						original: true,
					},
				);
			}
		}
	}

	// build the result array
	for (const node of array) {
		// keep the original node
		result.push(node);

		if (isCSSRule(node)) {
			// get the corresponding occurrence, we are sure it exists
			const text = stringCache.get(node)!;
			const occurrence = occurrenceCache.get(text)!;

			// if the rule has no prefixed version, prefix it
			if (occurrence.original && !occurrence.prefixed) {
				const prefixes = collectCSSRulePrefixes(node, namesToFeatures, targets);
				if (prefixes.size === 0) {
					continue;
				}

				for (const prefix of prefixes) {
					result.push(
						prefixCSSRulePrelude({rule: node, prefix, namesToFeatures, targets}),
					);
				}
			}
		}
	}

	return result;
}

interface RuleRepresentation {
	text: string;
	prefixed: boolean;
}

/**
 * If we want to check if a rule is already prefixed we
 * need a distinctive representation for different rules
 * but indistinctive for variations of the same rule.
 *
 * eg. .example:any-link != .example:read-only
 *     .example:any-link == .example:-moz-any-link
 *
 * The function below converts the rule's prelude to a string
 * and removes all occurrences of prefixes. It also returns if
 * the rule was already containing a prefix.
 */
function rulePreludeToString(rule: CSSRule): RuleRepresentation {
	const builder = new Builder({
		allowInterpreterDirective: false,
		typeAnnotations: false,
	});
	const token = builder.tokenizeStatementList(rule.prelude, MOCK_PARENT);
	const {code} = printTokenToString(
		token,
		{
			indentString: "",
			printWidth: Infinity,
			rootIndent: 0,
			tabWidth: 2,
		},
	);
	return {
		text: code.replace(PREFIX_REG_EXPR, ""),
		prefixed: PREFIX_REG_EXPR.test(code),
	};
}

function collectCSSRulePrefixes(
	rule: CSSRule,
	namesToFeatures: Map<string, string>,
	targets: Browser[],
) {
	const allPrefixes = new Set<string>();

	for (const selector of rule.prelude) {
		const newPrefixes = collectCSSSelectorPrefixes(
			selector,
			namesToFeatures,
			targets,
		);
		for (const newPrefix of newPrefixes) {
			allPrefixes.add(newPrefix);
		}
	}

	return allPrefixes;
}

function collectCSSSelectorPrefixes(
	selector: CSSSelector,
	namesToFeatures: Map<string, string>,
	targets: Browser[],
) {
	const allPrefixes = new Set<string>();

	for (const pattern of selector.patterns) {
		if (isPseudoSelector(pattern)) {
			const browserFeature = namesToFeatures.get(pattern.value);
			if (browserFeature === undefined) {
				continue;
			}
			const newPrefixes = getPrefixes(targets, browserFeature);
			for (const newPrefix of newPrefixes) {
				allPrefixes.add(newPrefix);
			}
		}
	}

	return allPrefixes;
}

interface PrefixCSSSelectorProps {
	selector: CSSSelector;
	prefix: string;
	namesToFeatures: Map<string, string>;
	targets: Browser[];
}

function prefixCSSSelector(
	{
		selector,
		prefix,
		namesToFeatures,
		targets,
	}: PrefixCSSSelectorProps,
): CSSSelector {
	const newPatterns = selector.patterns.map((pattern) => {
		if (isPseudoSelector(pattern)) {
			const browserFeature = namesToFeatures.get(pattern.value);
			if (browserFeature === undefined) {
				return pattern;
			}
			const prefixes = getPrefixes(targets, browserFeature);
			if (!prefixes.has(prefix)) {
				return pattern;
			}

			// custom case for ::placeholder
			let renamedPrefix = prefix;
			if (pattern.value === "placeholder") {
				renamedPrefix = prefix.replace("webkit", "webkit-input");
			}

			if (isPseudoClassSelector(pattern)) {
				return cssPseudoClassSelector.create({
					...pattern,
					value: `-${renamedPrefix}-${pattern.value}`,
				});
			} else {
				return cssPseudoElementSelector.create({
					...pattern,
					value: `-${renamedPrefix}-${pattern.value}`,
				});
			}
		}
		return pattern;
	});

	return cssSelector.create({
		...selector,
		patterns: newPatterns,
	});
}

interface PrefixCSSRulePreludeProps {
	rule: CSSRule;
	prefix: string;
	namesToFeatures: Map<string, string>;
	targets: Browser[];
}

function prefixCSSRulePrelude(
	{rule, prefix, namesToFeatures, targets}: PrefixCSSRulePreludeProps,
) {
	const newPrelude = rule.prelude.map((selector) =>
		prefixCSSSelector({selector, prefix, namesToFeatures, targets})
	);
	return cssRule.create({
		...rule,
		prelude: newPrelude,
	});
}

function isPseudoClassSelector(
	node: AnyCSSPattern,
): node is CSSPseudoClassSelector {
	return node.type === "CSSPseudoClassSelector";
}

function isPseudoElementSelector(
	node: AnyCSSPattern,
): node is CSSPseudoElementSelector {
	return node.type === "CSSPseudoElementSelector";
}

function isPseudoSelector(node: AnyCSSPattern): node is CSSPseudoSelector {
	return isPseudoClassSelector(node) || isPseudoElementSelector(node);
}

function isCSSRule(node: AnyNode): node is CSSRule {
	return node.type === "CSSRule";
}

const projectConfigToTargets: WeakMap<ProjectConfig, Browser[]> = new WeakMap();

function getTargets(path: CompilerPath): Browser[] {
	const projectConfig = path.context.project.config;

	if (!projectConfigToTargets.has(projectConfig)) {
		const propsTargets =
			projectConfig.targets.get(path.context.options.target ?? "default") ?? [];
		const targets = propsTargets.map((props) => getBrowser(props));

		projectConfigToTargets.set(projectConfig, targets);
	}

	return projectConfigToTargets.get(projectConfig)!;
}

const prefixCache: Map<string, Set<string>> = new Map();

function getPrefixes(
	targets: Browser[],
	browserFeaturesKey: string,
): Set<string> {
	if (!prefixCache.has(browserFeaturesKey)) {
		const prefixes = new Set<string>();
		for (const browser of targets) {
			if (browser.cssFeatureRequiresPrefix(browserFeaturesKey)) {
				prefixes.add(browser.getPrefix());
			}
		}

		prefixCache.set(browserFeaturesKey, prefixes);
	}
	// `!` thanks Typescript
	return prefixCache.get(browserFeaturesKey)!;
}
