import {TypedVisitor, Visitor} from "./types";
import {UnknownObject} from "@internal/typescript-helpers";
import {CompilerPath, signals} from ".";

export function createVisitor<State extends UnknownObject>(
	visitor: Visitor<State>,
) {
	return visitor;
}

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
