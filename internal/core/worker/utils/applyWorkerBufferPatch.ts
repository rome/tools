import {Number0, ob1Get0, ob1Inc, ob1Number0} from "@internal/ob1";
import {WorkerBufferPatch} from "@internal/core/common/bridges/WorkerBridge";

export function applyWorkerBufferPatch(
	original: string,
	{range: {start, end}, text}: WorkerBufferPatch,
) {
	let currLine: Number0 = ob1Number0;
	let currChar: Number0 = ob1Number0;
	let cursor: Number0 = ob1Number0;

	let buffer: string | undefined;

	// Offset based on UTF-16 code units
	while (ob1Get0(cursor) <= original.length) {
		// Start position
		if (currLine === start.line && currChar === start.character) {
			// Include anything before the start of the patch range
			const preText = original.slice(0, ob1Get0(cursor));
			buffer = preText + text;
		}

		// End position
		if (currLine === end.line && currChar === end.character) {
			if (buffer === undefined) {
				// Start position was not encountered
				return undefined;
			}
			// Append anything after the end of the patch range
			const postText = original.slice(ob1Get0(cursor));
			buffer += postText;
			return buffer;
		}

		switch (original[ob1Get0(cursor)]) {
			case "\n": {
				currLine = ob1Inc(currLine);
				currChar = ob1Number0;
				break;
			}
			case "\r": {
				currLine = ob1Inc(currLine);
				currChar = ob1Number0;

				//  \r\n should only advance 1 line
				if (original[ob1Get0(cursor) + 1] === "\n") {
					cursor = ob1Inc(cursor);
				}
				break;
			}
			default: {
				currChar = ob1Inc(currChar);
			}
		}

		cursor = ob1Inc(cursor);
	}

	// End position was not encountered
	return undefined;
}
