import {ConsumePath} from "./types";

export function serializeConsumePath(path: ConsumePath): string {
	return path.map((part) => JSON.stringify(String(part))).join(".");
}
