import {CSSBlock} from "@internal/ast";
import {UnknownObject} from "@internal/typescript-helpers";
import {createVisitor, EnterSignal, ExitSignal, Path} from "@internal/compiler";
import {VisitorStateEnter, VisitorStateExit} from "@internal/compiler/lib/VisitorState";

export function nodeHasPrefix(
	node: CSSBlock,
	prefix: string,
): boolean {
	if (node.value) {
		return node.value.some((n) =>
			n.type === "CSSDeclaration" && n.name.startsWith(prefix)
		);
	}
	return false;
}

export function nodeValueHasPrefix(
	node: CSSBlock,
	name: string,
	prefix: string,
): boolean {
	if (node.value) {
		return node.value.some((n) =>
			n.type === "CSSDeclaration" && n.name === name && n.value.some((v) =>
				v?.type === "CSSString" && v.value.startsWith(prefix)
			)
		);
	}
	return false;
}

// TODO make an actual implementation
interface PrefixConfig {
	target: "modern" | string
}
export function matchBrowser(target: PrefixConfig["target"], query: PrefixConfig["target"]) { return true }


export interface PrefixVisitor<State extends UnknownObject> {
	name: string;
	enter?: (path: Path, prefixConfig: PrefixConfig, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: Path, prefixConfig: PrefixConfig, state: VisitorStateExit<State>) => ExitSignal;
}

export function createPrefixVisitor<State extends UnknownObject> (
	visitor: PrefixVisitor<State>,
) {
	return visitor;
}

export function wrapPrefixVisitor<State extends UnknownObject>(
	visitor: PrefixVisitor<State>,
	prefixConfig: PrefixConfig
) {
	return createVisitor<State>({
		name: visitor.name,
		enter: visitor.enter ? (path, state) => visitor.enter!(path, prefixConfig, state) : undefined,
		exit: visitor.exit ? (path, state) => visitor.exit!(path, prefixConfig, state) : undefined,
	})
}
