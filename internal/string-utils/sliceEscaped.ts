import {ZeroIndexed} from "@internal/math";
import {isEscaped} from "./isEscaped";

// When slicing a string the last character could have been a truncated escape
// This removes trailing slashes that aren't escaped
export function sliceEscaped(str: string, end: number): string {
	if (str.length > end) {
		let sliced = str.slice(0, end);
		while (
			sliced[sliced.length - 1] === "\\" &&
			!isEscaped(new ZeroIndexed(str.length - 1), str)
		) {
			sliced = sliced.slice(0, -1);
		}
		return sliced;
	} else {
		return str;
	}
}
