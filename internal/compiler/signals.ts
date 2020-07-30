import {AnyNode} from "@internal/ast";

export type SkipSignal = {
	type: "SKIP";
};

export type RemoveSignal = {
	type: "REMOVE";
};

export type ReplaceSignal = {
	type: "REPLACE";
	value: AnyNode | Array<AnyNode>;
};

export type ParentSignal = {
	type: "PARENT";
	parent: AnyNode;
	signal: ExitSignal;
};

export type RetainSignal = {
	type: "RETAIN";
};

export type EnterSignal = SkipSignal | ExitSignal;

export type ExitSignal =
	| RetainSignal
	| RemoveSignal
	| ReplaceSignal
	| ParentSignal;

export const skip: SkipSignal = {
	type: "SKIP",
};

export const remove: RemoveSignal = {
	type: "REMOVE",
};

export const retain: RetainSignal = {
	type: "RETAIN",
};

export function maybeReplace(
	old: AnyNode,
	node: AnyNode,
): ReplaceSignal | RetainSignal {
	if (old === node) {
		return retain;
	} else {
		return {
			type: "REPLACE",
			value: node,
		};
	}
}

export function replace(node: AnyNode | Array<AnyNode>): ReplaceSignal {
	return {
		type: "REPLACE",
		value: node,
	};
}

export function parent(node: AnyNode, signal: ExitSignal): ExitSignal {
	return {
		type: "PARENT",
		parent: node,
		signal,
	};
}
