import {
	CSSBlock,
	CSSDeclaration,
	cssBlock,
	cssDeclaration,
	cssIdentifier,
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

type PrefixCompilerPath = CompilerPath & {
	node: RequiredProps<CSSBlock, "value">;
};

function isCssBlockAndHasValue(path: CompilerPath): path is PrefixCompilerPath {
	return (
		path.node.type === "CSSBlock" &&
		path.node.value !== undefined &&
		path.node.value.length > 0
	);
}

export interface PrefixVisitor<State extends UnknownObject> {
	name: string;
	enter?: (
		path: PrefixCompilerPath,
		state: VisitorStateEnter<State>,
	) => EnterSignal;
	exit?: (
		path: PrefixCompilerPath,
		state: VisitorStateExit<State>,
	) => ExitSignal;
}

export function createPrefixVisitor<State extends UnknownObject>(
	visitor: PrefixVisitor<State>,
): Visitor<State> {
	return {
		name: `css-handler/prefix/${visitor.name}`,
		enter: (path: CompilerPath, state: VisitorStateEnter<State>) => {
			if (visitor.enter !== undefined && isCssBlockAndHasValue(path)) {
				return visitor.enter(path, state);
			} else {
				return signals.retain;
			}
		},
		exit: (path: CompilerPath, state: VisitorStateExit<State>) => {
			if (visitor.exit !== undefined && isCssBlockAndHasValue(path)) {
				return visitor.exit(path, state);
			} else {
				return signals.retain;
			}
		},
	};
}

interface PrefixCSSPropertyProps {
	path: PrefixCompilerPath;
	propertyName: string;
	browserFeaturesKey: string;
	rename?: (propertyName: string) => string;
}

interface PrefixCSSValueProps extends PrefixCSSPropertyProps {
	value: string;
	rename?: (value: string) => string;
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
