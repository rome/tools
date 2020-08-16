import {default as ParserCore} from "./ParserCore";
import {
	ParserCoreImplementation,
	ParserCoreTypes,
	ParserOptions,
	Position,
	SourceLocation,
} from "./types";
import {catchDiagnosticsSync} from "@internal/diagnostics";

export function isDigit(char: undefined | string): boolean {
	return char !== undefined && /[0-9]/.test(char);
}

export function isAlpha(char: undefined | string): boolean {
	return char !== undefined && /[A-Za-z]/.test(char);
}

export function isHexDigit(char: undefined | string): boolean {
	return char !== undefined && /[0-9A-Fa-f]/.test(char);
}

export function isESIdentifierChar(char: undefined | string): boolean {
	return char !== undefined && /[A-F0-9a-z_$]/.test(char);
}

export function isESIdentifierStart(char: undefined | string): boolean {
	return char !== undefined && /[A-Fa-z_$]/.test(char);
}

export function readUntilLineBreak(char: string): boolean {
	return char !== "\n";
}

export function createParser<Types extends ParserCoreTypes>(
	impl: ParserCoreImplementation<Types>,
): (opts: Types["options"], meta: Types["meta"]) => ParserCore<Types> {
	return (opts: Types["options"], meta: Types["meta"]) => {
		return new ParserCore(impl, opts, meta);
	};
}

export function tryParseWithOptionalOffsetPosition<
	Opts extends ParserOptions,
	Ret
>(
	parserOpts: Opts,
	opts: {
		getOffsetPosition: () => Position;
		parse: (opts: Opts) => Ret;
	},
): Ret {
	const {value} = catchDiagnosticsSync(() => {
		return opts.parse(parserOpts);
	});

	if (value === undefined) {
		// Diagnostics must be present
		opts.parse({
			...parserOpts,
			offsetPosition: opts.getOffsetPosition(),
		});
		throw new Error("Expected error");
	} else {
		return value;
	}
}

/**
 * -1: left < right
 * 0: left === right
 * 1: left > right
 */
export function comparePositions(
	left: undefined | Position,
	right: undefined | Position,
): -1 | 0 | 1 {
	if (left === undefined && right !== undefined) {
		return -1;
	}

	if (left !== undefined && right === undefined) {
		return 1;
	}

	if (left === undefined || right === undefined) {
		return 0;
	}

	if (left.line === right.line) {
		if (left.column < right.column) {
			return -1;
		}

		if (left.column > right.column) {
			return 1;
		}

		return 0;
	}

	if (left.line < right.line) {
		return -1;
	}

	if (left.line > right.line) {
		return 1;
	}

	throw new Error(
		"Not a possible condition...? All possible states of a.line and b.line should have been handled above",
	);
}

export function derivePositionKey(pos: Position): string {
	return `${String(pos.line)}:${String(pos.column)}`;
}

// Utility methods for dealing with nodes
export function extractSourceLocationRangeFromNodes(
	nodes: Array<{
		loc?: SourceLocation;
	}>,
): undefined | SourceLocation {
	if (nodes.length === 0) {
		return undefined;
	}

	let filename: undefined | string = undefined;
	let start: undefined | Position = undefined;
	let end: undefined | Position = undefined;

	for (const node of nodes) {
		const {loc} = node;
		if (loc === undefined) {
			continue;
		}

		if (start === undefined || comparePositions(loc.start, start) === -1) {
			start = loc.start;
		}

		if (end === undefined || comparePositions(loc.end, end) === 1) {
			end = loc.end;
		}

		if (filename === undefined) {
			filename = loc.filename;
		} else if (filename !== loc.filename) {
			throw new Error(
				`Mixed filenames in node, expected ${filename} but got ${loc.filename}`,
			);
		}
	}

	if (start === undefined || end === undefined) {
		return undefined;
	}

	return {
		filename,
		start,
		end,
	};
}
