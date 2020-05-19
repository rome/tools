import {JSXElement} from "@romejs/ast";
import getJSXAttribute from "./getJSXAttribute";

export default function hasJSXAttribute(
	tag: JSXElement,
	name: string,
	allowEmpty: boolean = false,
): boolean {
	return getJSXAttribute(tag, name, allowEmpty) !== undefined;
}
