import {
	AnyNode,
	TSBigIntLiteralTypeAnnotation,
	tsBigIntLiteralTypeAnnotation,
} from "@romefrontend/ast";

export default function TSBigIntLiteralTypeAnnotation(node: AnyNode) {
	node = tsBigIntLiteralTypeAnnotation.assert(node);
	throw new Error("unimplemented");
}
