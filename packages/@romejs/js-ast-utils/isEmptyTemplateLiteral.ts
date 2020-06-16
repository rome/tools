import {JSTemplateLiteral} from "@romejs/ast";

export default function isEmptyTemplateLiteral(node: JSTemplateLiteral) {
	if (!node.quasis || node.quasis.length === 0) {
		return false;
	}

	if (node.quasis.length === 1) {
		const quasi = node.quasis[0];
		return quasi.cooked === "";
	}
	return false;
}
