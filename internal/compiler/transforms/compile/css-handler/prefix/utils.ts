import {
	CSSRoot,
	CSSRule,
	CSSSelector,
	CSSPseudoClassSelector,
	CSSBlock,
	CSSDeclaration,
	cssBlock,
	cssDeclaration,
	cssIdentifier,
	AnyCSSPattern, AnyNode, cssPseudoClassSelector, cssRule, cssSelector, cssRoot
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

export interface PrefixCSSBlockVisitor<State extends UnknownObject> extends __TypedVisitor<
	State,
	PrefixCSSBlockCompilerPath
> {}

export function createPrefixCSSBlockVisitor<State extends UnknownObject>(
	visitor: PrefixCSSBlockVisitor<State>,
): Visitor<State> {
	return __visitorTransformer<State, PrefixCSSBlockCompilerPath>(
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
	node: CSSRoot
}

export interface PrefixCSSRootVisitor<State extends UnknownObject> extends __TypedVisitor<
	State,
	PrefixCSSRootCompilerPath
> {}

export function createPrefixCSSRootVisitor<State extends UnknownObject>(
	visitor: PrefixCSSRootVisitor<State>,
): Visitor<State> {
	return __visitorTransformer<State, PrefixCSSRootCompilerPath>(
		"css-handler/prefix",
		visitor,
		(path): path is PrefixCSSRootCompilerPath => path.node.type === "CSSRoot",
	);
}

function isPseudoClass(node: AnyCSSPattern): node is CSSPseudoClassSelector {
	return node.type === "CSSPseudoClassSelector";
}

function collectCSSSelectorPrefixes(selector: CSSSelector, namesToFeatures: Map<string, string>, targets: Browser[]) {
	return selector.patterns.reduce((allPrefixes, pattern) => {
		if (isPseudoClass(pattern)) {
			const browserFeature = namesToFeatures.get(pattern.value);
			if (browserFeature === undefined) return allPrefixes;
			const newPrefixes = getPrefixes(targets, browserFeature);
			if (newPrefixes.size === 0) return allPrefixes;
			return new Set([...allPrefixes, ...newPrefixes]);
		}
		return allPrefixes;
	}, new Set<string>());
}

function collectCSSRulePrefixes(rule: CSSRule, namesToFeatures: Map<string, string>, targets: Browser[]) {
	return rule.prelude.reduce((allPrefixes, selector) => {
		const newPrefixes = collectCSSSelectorPrefixes(selector, namesToFeatures, targets);
		if (newPrefixes.size === 0) return allPrefixes;
		return new Set([...allPrefixes, ...newPrefixes]);
	}, new Set<string>());
}

function isCSSRule(node: AnyNode): node is CSSRule {
	return node.type === "CSSRule";
}

function prefixCSSSelector(selector: CSSSelector, prefix: string, namesToFeatures: Map<string, string>, targets: Browser[]) {
	const newPatterns = selector.patterns.map((pattern) => {
		if (isPseudoClass(pattern)) {
			const browserFeature = namesToFeatures.get(pattern.value);
			if (browserFeature === undefined) return pattern;
			const prefixes = getPrefixes(targets, browserFeature);
			if (!prefixes.has(prefix)) return pattern;
			return cssPseudoClassSelector.create({
				...pattern,
				value: `-${prefix}-${pattern.value}`
			});
		}

		return pattern;
	});
	return cssSelector.create({
		...selector,
		patterns: newPatterns,
	})
}

function prefixCSSRule(rule: CSSRule, prefix:string, namesToFeatures: Map<string, string>, targets: Browser[]) {
	// COMMENT: here we are sure that at least one selector will be changed
	// so I think that's the most efficient way to write it
	const newSelectors = rule.prelude.map((selector) => prefixCSSSelector(selector, prefix, namesToFeatures, targets));
	return cssRule.create({
		...rule,
		prelude: newSelectors,
	})
}

export function prefixPseudoInCSSRoot(path: PrefixCSSRootCompilerPath, namesToFeatures: Map<string, string>) {
	const targets = getTargets(path);

	// COMMENT: Maybe write this in a more efficient manner?
	// If there are no prefixes to be added, newBody is still created :P
	const newBody = [];
	for (const node of path.node.body) {
		newBody.push(node);

		if (!isCSSRule(node)) continue;
		const prefixes = collectCSSRulePrefixes(node, namesToFeatures, targets);
		if (prefixes.size === 0) continue;

		for (const prefix of prefixes) {
			newBody.push(prefixCSSRule(node, prefix, namesToFeatures, targets));
		}
	}
	
	if (newBody.length !== path.node.body.length) {
		const newRoot = cssRoot.create({
			...path.node,
			body: newBody,
		});
		return signals.replace(newRoot);
	}
	return signals.retain;
}

export function prefixPseudoInCSSBlock(path: PrefixCSSBlockCompilerPath, namesToFeatures: Map<string, string>) {
	const targets = getTargets(path);

	// COMMENT: same performance problem as above
	// also, it's pretty duplicate. would it worth the effort to make it one function?
	const newValue = [];
	for (const node of path.node.value) {
		newValue.push(node);

		if (!isCSSRule(node)) continue;
		const prefixes = collectCSSRulePrefixes(node, namesToFeatures, targets);
		if (prefixes.size === 0) continue;

		for (const prefix of prefixes) {
			newValue.push(prefixCSSRule(node, prefix, namesToFeatures, targets));
		};
	}

	if (newValue.length !== path.node.value.length) {
		const newBlock = cssBlock.create({
			...path.node,
			value: newValue,
		})
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

// COMMENT: this is temporary naming, I'm open to suggestions, I couldn't find something 
// that I really like
export interface __TypedVisitor<
	State extends UnknownObject,
	PathType extends CompilerPath
> {
	name: string;
	enter?: (path: PathType, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: PathType, state: VisitorStateExit<State>) => ExitSignal;
}

// vistitor for (CompilerPath, State) => visitor for (PathType, State)
export function __visitorTransformer<
	State extends UnknownObject,
	PathType extends CompilerPath
>(
	name: string,
	visitor: __TypedVisitor<State, PathType>,
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
