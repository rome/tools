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
	lines: string[],
	opts: LogOptions = {},
	activeElement?: ActiveElement,
) {
	if (lines.length === 0) {
		return;
	}

	const stderr = opts.stderr === true;
	let pushBuffer = activeElement === undefined;

	let buff = "";

	// Destroy all active elements
	for (let i = 0; i < stream.activeElements.size; i++) {
		// Current line contains an active element so don't go up for the first
		if (i > 0) {
			buff += ansiEscapes.cursorUp();
		}
		buff += ansiEscapes.cursorTo(0) + ansiEscapes.eraseLine;
	}
	stream.activeElements.clear();

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];

		if (pushBuffer) {
			if (stream.state.leadingNewline) {
				stream.state.buffer.push(line);
			} else {
				// If we have no newline then we are a part of the previous line
				const prev = stream.state.buffer.pop() || "";
				const line = prev + lines[0];
				stream.state.buffer.push(line);
			}
		}

		buff += line;
		
		let noNewline = i === lines.length - 1 && opts.noNewline;
		if (!noNewline) {
			if (pushBuffer) {
				stream.state.leadingNewline = true;
				stream.state.currentLine = stream.state.currentLine.increment();
			}
			buff += "\n";
		}
	}

	if (activeElement !== undefined) {
		stream.activeElements.add(activeElement);
	}

	stream.write(buff, stderr);
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
