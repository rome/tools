import {AssembledBundle} from "@internal/core";
import {AbsoluteFilePath} from "@internal/path";

export function serializeAssembled(
	assembled: AssembledBundle,
	getCode: (path: AbsoluteFilePath) => undefined | string,
): string {
	return assembled.map((item) => {
		if (item[0] === 0) {
			return item[1];
		} else {
			const path = item[1];
			const compileResult = getCode(path);
			if (compileResult === undefined) {
				throw new Error("Compiled file not found");
			} else {
				return compileResult;
			}
		}
	}).join("\n");
}
