import {Visitor} from "./types";
import {UnknownObject} from "@internal/typescript-helpers";

export function createVisitor<State extends UnknownObject>(
	visitor: Visitor<State>,
) {
	return visitor;
}
