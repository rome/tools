import {ZeroIndexed} from "@internal/math";
import {WorkerBufferPatch} from "@internal/core";

export function applyWorkerBufferPatch(
	original: string,
	{range: {start, end}, text}: WorkerBufferPatch,
) {
	let currLine: ZeroIndexed = new ZeroIndexed();
	let currChar: ZeroIndexed = new ZeroIndexed();
	let cursor: ZeroIndexed = new ZeroIndexed();

	let buffer: string | undefined;

	// Offset based on UTF-16 code units
	while (cursor.valueOf() <= original.length) {
		// Start position
		if (currLine.equal(start.line) && currChar.equal(start.character)) {
			// Include anything before the start of the patch range
			const preText = original.slice(0, cursor.valueOf());
			buffer = preText + text;
		}

		// End position
		if (currLine.equal(end.line) && currChar.equal(end.character)) {
			if (buffer === undefined) {
				// Start position was not encountered
				return undefined;
			}

			// Append anything after the end of the patch range
			const postText = original.slice(cursor.valueOf());
			buffer += postText;
			return buffer;
		}

		switch (original[cursor.valueOf()]) {
			case "\n": {
				currLine = currLine.increment();
				currChar = new ZeroIndexed();
				break;
			}

			case "\r": {
				currLine = currLine.increment();
				currChar = new ZeroIndexed();

				//  \r\n should only advance 1 line
				if (original[cursor.valueOf() + 1] === "\n") {
					cursor = cursor.increment();
				}
				break;
			}

			default: {
				currChar = currChar.increment();
			}
		}

		cursor = cursor.increment();
	}

	// End position was not encountered
	return undefined;
}
