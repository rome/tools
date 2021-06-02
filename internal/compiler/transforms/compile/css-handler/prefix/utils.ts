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
	EnterSignal,
	ExitSignal,
	Visitor,
	signals,
} from "@internal/compiler";
import {Browser} from "@internal/browser-features/Browser";
import {ProjectConfig} from "@internal/project";
import {getBrowser} from "@internal/browser-features";
import {
	VisitorStateEnter,
	VisitorStateExit,
} from "@internal/compiler/lib/VisitorState";

// TODO: This will be changed in further commits to allow
// implementing other features which require state
// eg. purging unnecessary prefixes
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
	return transformVisitor<CompilerPrefixerState, PrefixCSSBlockCompilerPath>(
		"css-handler/prefix",
		visitor,
		isCssBlockAndHasValue,
	);
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

		for (const prefix of getPrefixes(getTargets(path), browserFeaturesKey)) {
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
		for (const prefix of getPrefixes(getTargets(path), browserFeaturesKey)) {
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
	return transformVisitor<CompilerPrefixerState, PrefixCSSRootCompilerPath>(
		"css-handler/prefix",
		visitor,
		(path): path is PrefixCSSRootCompilerPath => path.node.type === "CSSRoot",
	);
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
			newPrefixes.forEach((newPrefix) => allPrefixes.add(newPrefix));
		}
	}

	return allPrefixes;
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
		newPrefixes.forEach((newPrefix) => allPrefixes.add(newPrefix));
	}

	return allPrefixes;
}

function isCSSRule(node: AnyNode): node is CSSRule {
	return node.type === "CSSRule";
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
) {
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

			if (isPseudoClassSelector(pattern)) {
				return cssPseudoClassSelector.create({
					...pattern,
					value: `-${prefix}-${pattern.value}`,
				});
			} else {
				return cssPseudoElementSelector.create({
					...pattern,
					value: `-${prefix}-${pattern.value}`,
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

export function prefixPseudoSelectorInCSSRoot(
	path: PrefixCSSRootCompilerPath,
	namesToFeatures: Map<string, string>,
): signals.EnterSignal {
	const targets = getTargets(path);

	const newBody: (CSSRule | CSSAtRule)[] = [];
	for (const node of path.node.body) {
		newBody.push(node);

		if (!isCSSRule(node)) {
			continue;
		}
		const prefixes = collectCSSRulePrefixes(node, namesToFeatures, targets);
		if (prefixes.size === 0) {
			continue;
		}

		for (const prefix of prefixes) {
			newBody.push(
				prefixCSSRulePrelude({rule: node, prefix, namesToFeatures, targets}),
			);
		}
	}

	if (newBody.length > path.node.body.length) {
		const newRoot = cssRoot.create({
			...path.node,
			body: newBody,
		});
		return signals.replace(newRoot);
	}
	return signals.retain;
}

export function prefixPseudoSelectorInCSSBlock(
	path: PrefixCSSBlockCompilerPath,
	namesToFeatures: Map<string, string>,
): signals.EnterSignal {
	const targets = getTargets(path);

	const newValue: CSSBlockValue = [];
	for (const node of path.node.value) {
		newValue.push(node);

		if (!isCSSRule(node)) {
			continue;
		}
		const prefixes = collectCSSRulePrefixes(node, namesToFeatures, targets);
		if (prefixes.size === 0) {
			continue;
		}

		for (const prefix of prefixes) {
			newValue.push(
				prefixCSSRulePrelude({rule: node, prefix, namesToFeatures, targets}),
			);
		}
	}

	if (newValue.length > path.node.value.length) {
		const newBlock = cssBlock.create({
			...path.node,
			value: newValue,
		});
		return signals.replace(newBlock);
	}
	return signals.retain;
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

// TODO: this should be moved outside utils as it is generic, that's
// why I'm still using State extends UnknownObject
export interface TypedVisitor<
	State extends UnknownObject,
	PathType extends CompilerPath
> {
	name: string;
	enter?: (path: PathType, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: PathType, state: VisitorStateExit<State>) => ExitSignal;
}

// vistitor for (CompilerPath, State) => visitor for (PathType, State)
export function transformVisitor<
	State extends UnknownObject,
	PathType extends CompilerPath
>(
	name: string,
	visitor: TypedVisitor<State, PathType>,
	isPathType: (path: CompilerPath) => path is PathType,
): Visitor<State> {
	return {
		name: `${name}/${visitor.name}`,
		enter: (path, state) => {
			if (visitor.enter !== undefined && isPathType(path)) {
				return visitor.enter(path, state);
			}
			return signals.retain;
		},
		exit: (path, state) => {
			if (visitor.exit !== undefined && isPathType(path)) {
				return visitor.exit(path, state);
			}
			return signals.retain;
		},
	};
}
