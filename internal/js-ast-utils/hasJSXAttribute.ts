import {JSXElement} from "@internal/ast";
import {getJSXAttribute} from "./getJSXAttribute";

export function hasJSXAttribute(
	tag: JSXElement,
	name: string,
	allowEmpty: boolean = false,
): boolean {
	return getJSXAttribute(tag, name, allowEmpty) !== undefined;
}
