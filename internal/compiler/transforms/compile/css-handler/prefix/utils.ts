import {
	CSSBlock,
	CSSDeclaration,
	cssBlock,
	cssDeclaration,
	cssIdentifier,
} from "@internal/ast";
import {UnknownObject} from "@internal/typescript-helpers";
import {
	EnterSignal,
	ExitSignal,
	CompilerPath,
	createVisitor,
	signals,
} from "@internal/compiler";
import {
	VisitorStateEnter,
	VisitorStateExit,
} from "@internal/compiler/lib/VisitorState";
import { Browser } from "@internal/browser-features/Browser";

export function nodeHasPrefixedProperty(
	node: CSSBlock,
	property: string,
	prefix: string,
): boolean {
	if (node.value) {
		return node.value.some((n) =>
			n.type === "CSSDeclaration" && n.name === `-${prefix}-${property}`
		);
	}
	return false;
}

export function nodeHasPrefixedPropertyValue(
	node: CSSBlock,
	property: string,
	value: string,
	prefix: string,
): boolean {
	if (node.value) {
		return node.value.some((n) =>
			n.type === "CSSDeclaration" &&
			n.name === property &&
			n.value.length === 1 &&
			n.value[0]?.type === "CSSIdentifier" &&
			n.value[0].value === `-${prefix}-${value}`
		);
	}
	return false;
}

export function nodePropertyIndex(node: CSSBlock, property: string): number {
	return node.value
		? node.value.findIndex((n) =>
				n.type === "CSSDeclaration" && n.name === property
			)
		: -1;
}

export function nodePropertyValueIndex(
	node: CSSBlock,
	property: string,
	value: string,
): number {
	return node.value
		? node.value.findIndex((n) =>
				n.type === "CSSDeclaration" &&
				n.name === property &&
				n.value.length === 1 &&
				n.value[0]?.type === "CSSIdentifier" &&
				n.value[0].value === value
			)
		: -1;
}
export interface PrefixVisitor<State extends UnknownObject> {
	name: string;
	enter?: (
		path: CompilerPath,
		targets: Browser[],
		state: VisitorStateEnter<State>,
	) => EnterSignal;
	exit?: (
		path: CompilerPath,
		targets: Browser[],
		state: VisitorStateExit<State>,
	) => ExitSignal;
}

export function createPrefixVisitor<State extends UnknownObject>(
	visitor: PrefixVisitor<State>,
) {
	return {
		name: `css-handler/prefix/${visitor.name}`,
		enter: visitor.enter,
		exit: visitor.exit,
	};
}

export function wrapPrefixVisitor<State extends UnknownObject>(
	visitor: PrefixVisitor<State>,
	targets: Browser[],
) {
	return createVisitor<State>({
		name: visitor.name,
		enter: visitor.enter
			? (path, state) => visitor.enter!(path, targets, state)
			: undefined,
		exit: visitor.exit
			? (path, state) => visitor.exit!(path, targets, state)
			: undefined,
	});
}

export function prefixCSSProperty(
	path: CompilerPath,
	propertyName: string,
	browserFeaturesPropertyName: string,
	targets: Browser[]
) {
	const {node} = path;
	if (node.type === "CSSBlock") {
		if (node.value && node.value.length > 0) {
			const propertyIndex = nodePropertyIndex(node, propertyName);
			if (propertyIndex > -1) {
				const property = node.value[propertyIndex] as CSSDeclaration;
				const newDeclarations = [];
				const prefixes = new Set(targets.filter((browser) => browser.cssFeatureRequiresPrefix(browserFeaturesPropertyName)).map((browser) => browser.getPrefix()));
				for (const prefix of prefixes) {
					const hasPrefix = nodeHasPrefixedProperty(node, propertyName, prefix);
					if (!hasPrefix) {
						newDeclarations.push(
							cssDeclaration.create({
								name: `-${prefix}-${property.name}`,
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
							property,
							...newDeclarations,
							...node.value.slice(propertyIndex + 1, node.value.length),
						],
					});
					return signals.replace(block);
				}
			}
		}
	}

	return signals.retain;
}

export function prefixCSSValue(
	path: CompilerPath,
	propertyName: string,
	value: string,
	browserFeaturesPropertyName: string,
	targets: Browser[]
) {
	const {node} = path;
	if (node.type === "CSSBlock") {
		if (node.value && node.value.length > 0) {
			const propertyIndex = nodePropertyValueIndex(node, propertyName, value);
			if (propertyIndex > -1) {
				const property = node.value[propertyIndex] as CSSDeclaration;
				const newDeclarations = [];
				const prefixes = new Set(targets.filter((browser) => browser.cssFeatureRequiresPrefix(browserFeaturesPropertyName)).map((browser) => browser.getPrefix()));
				for (const prefix of prefixes) {
					const hasPrefix = nodeHasPrefixedPropertyValue(
						node,
						propertyName,
						value,
						prefix,
					);
					if (!hasPrefix) {
						newDeclarations.push(
							cssDeclaration.create({
								name: property.name,
								value: [cssIdentifier.create({value: `-${prefix}-${value}`})],
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
							property,
							...newDeclarations,
							...node.value.slice(propertyIndex + 1, node.value.length),
						],
					});
					return signals.replace(block);
				}
			}
		}
	}

	return signals.retain;
}
