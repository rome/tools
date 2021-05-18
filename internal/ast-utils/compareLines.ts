import {AnyNode} from "@internal/ast";

export enum LineComparison {
	Unknown,
	Same,
	Before,
	After,
}

export function compareLines(aNode: AnyNode, bNode: AnyNode): LineComparison {
	if (aNode.loc != null && bNode.loc != null) {
		const aLine = aNode.loc.end.line.valueOf();
		const bLine = bNode.loc.start.line.valueOf();
		if (aLine === bLine) {
			return LineComparison.Same;
		} else if (aLine > bLine) {
			return LineComparison.Before;
		} else {
			return LineComparison.After;
		}
	} else {
		return LineComparison.Unknown;
	}
}
