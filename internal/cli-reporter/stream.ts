import {
	ActiveElement,
	ReporterStream,
	ReporterStreamAttached,
	ReporterStreamState,
} from "./types";
import {LogOptions} from "./Reporter";
import {ansiEscapes} from "@internal/cli-layout";
import {ZeroIndexed} from "@internal/numbers";

export function getLeadingNewlineCount({state}: ReporterStreamAttached): number {
	if (!state.leadingNewline) {
		return 0;
	}

	let newlines = 1;
	for (let i = state.buffer.length - 1; i >= 0; i++) {
		const line = state.buffer[i];
		if (line === "") {
			newlines++;
		} else {
			break;
		}
	}
	return newlines;
}

export function createStreamState(): ReporterStreamState {
	return {
		currentLine: new ZeroIndexed(),
		leadingNewline: false,
		buffer: [],
	};
}

export function createActiveElementToken(): ActiveElement {
	return {
		rendered: new Set(),
	};
}

export function log(
	stream: ReporterStreamAttached,
	msg: string,
	opts: LogOptions = {},
	activeElement?: ActiveElement,
) {
	const stderr = opts.stderr === true;
	let pushBuffer = activeElement === undefined;

	// Destroy all active elements
	for (let i = 0; i < stream.activeElements.size; i++) {
		// Current line contains an active element so don't go up for the first
		if (i > 0) {
			stream.write(ansiEscapes.cursorUp(), stderr);
		}
		stream.write(ansiEscapes.cursorTo(0) + ansiEscapes.eraseLine, stderr);
	}
	stream.activeElements.clear();

	// We only push to the buffer if we have active line snapshots
	if (!pushBuffer && stream.state.buffer.length > 0) {
		stream.state.buffer = [];
	}

	if (pushBuffer) {
		if (stream.state.leadingNewline) {
			stream.state.buffer.push(msg);
		} else {
			// If we have no newline then we are a part of the previous line
			const prev = stream.state.buffer.pop() || "";
			const line = prev + msg;
			stream.state.buffer.push(line);
		}
	}

	stream.write(msg, stderr);

	if (activeElement !== undefined) {
		stream.activeElements.add(activeElement);
	}

	if (!opts.noNewline) {
		if (pushBuffer) {
			stream.state.leadingNewline = true;
			stream.state.currentLine = stream.state.currentLine.increment();
		}
		stream.write("\n", stderr);
	}
}

export function isANSICursorStream(stream: ReporterStream): boolean {
	return stream.format === "ansi" && stream.features.cursor;
}

export function clearScreen(stream: ReporterStreamAttached) {
	if (isANSICursorStream(stream)) {
		stream.write(ansiEscapes.clearScreen, false);
		stream.state = createStreamState();
	}
}

export function clearCurrentLine(stream: ReporterStreamAttached) {
	stream.write(ansiEscapes.cursorTo(0), false);
	stream.write(ansiEscapes.eraseLine, false);
}
