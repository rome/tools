import {
	ReporterStream,
	ReporterStreamAttached,
	ReporterStreamLineSnapshot,
	ReporterStreamState,
} from "./types";
import {LogOptions} from "./Reporter";
import {ansiEscapes} from "@internal/cli-layout";
import {
	Number0,
	ob1Add,
	ob1Dec,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Sub,
} from "@internal/ob1";

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

function calculateBufferPosition(
	{state}: ReporterStreamAttached,
	targetLine: Number0,
): {
	lineDiff: number;
	bufferIndex: number;
} {
	let lineDiff = ob1Get0(ob1Sub(state.currentLine, targetLine));
	let bufferIndex = state.buffer.length - lineDiff;

	if (!state.leadingNewline) {
		bufferIndex--;
	}

	return {lineDiff, bufferIndex};
}

export function removeLine(
	stream: ReporterStreamAttached,
	snapshot: ReporterStreamLineSnapshot,
	stderr: boolean = false,
) {
	// See if this stream is included in the snapshot
	const targetLine = stream.state.lineSnapshots.get(snapshot);
	if (targetLine === undefined) {
		return;
	}

	// We only care about ansi streams with cursor support
	if (!isANSICursorStream(stream)) {
		return;
	}

	// Position from where we currently are to the target line
	const {lineDiff, bufferIndex} = calculateBufferPosition(stream, targetLine);

	// If the line to delete is the one we're on then just erase it
	if (lineDiff === 0) {
		stream.state.buffer[bufferIndex] = "";
		clearCurrentLine(stream);
		return;
	}

	// Remove line from the buffer
	stream.state.buffer.splice(bufferIndex, 1);

	// Update snapshots
	for (const [snapshot, line] of stream.state.lineSnapshots) {
		stream.state.lineSnapshots.set(snapshot, ob1Dec(line));
	}

	// Update line since we've shifted at least one
	stream.state.currentLine = ob1Dec(stream.state.currentLine);

	// Go to the line right above where we want to remove
	stream.write(ansiEscapes.cursorUp(lineDiff + 1), stderr);
	stream.write(ansiEscapes.cursorTo(0), stderr);

	// Sweeping down, starting rendering the next line
	for (let i = 1; i <= lineDiff; i++) {
		const line = stream.state.buffer[stream.state.buffer.length - lineDiff + i];
		stream.write(ansiEscapes.cursorDown(), stderr);
		stream.write(ansiEscapes.cursorTo(0), stderr);
		stream.write(ansiEscapes.eraseLine, stderr);
		if (line !== undefined) {
			stream.write(line, stderr);
		}
	}
}

export function createStreamState(): ReporterStreamState {
	return {
		lineSnapshots: new Map(),
		currentLine: ob1Number0,
		leadingNewline: false,
		buffer: [],
		nextLineInsertLeadingNewline: false,
	};
}

export function log(
	stream: ReporterStreamAttached,
	msg: string,
	opts: LogOptions = {},
	lineOffset: number = 0,
) {
	const stderr = opts.stderr === true;
	let pushBuffer = true; //stream.state.lineSnapshots.size >= 0;

	const {replaceLineSnapshot} = opts;
	if (replaceLineSnapshot !== undefined) {
		let replaceLine = stream.state.lineSnapshots.get(replaceLineSnapshot);
		if (replaceLine === undefined) {
			// If we were given a replace line snapshot that we weren't a part of then the next line is considered
			// the owner
			stream.state.lineSnapshots.set(
				replaceLineSnapshot,
				stream.state.currentLine,
			);
			pushBuffer = true;
		} else if (isANSICursorStream(stream)) {
			replaceLine = ob1Add(replaceLine, lineOffset);
			logReplace(stream, msg, replaceLine, stderr);
			return;
		}
	}

	// We only push to the buffer if we have active line snapshots
	if (!pushBuffer && stream.state.buffer.length > 0) {
		stream.state.buffer = [];
	}

	if (stream.state.nextLineInsertLeadingNewline) {
		stream.state.nextLineInsertLeadingNewline = false;
		newline(stream, stderr);
	}

	if (stream.state.leadingNewline) {
		if (pushBuffer) {
			stream.state.buffer.push(msg);
		}
	} else {
		// If we have no newline then we are a part of the previous line
		const prev = stream.state.buffer.pop() || "";
		const line = prev + msg;
		if (pushBuffer) {
			stream.state.buffer.push(line);
		}
	}

	stream.write(msg, stderr);

	if (opts.preferNoNewline) {
		stream.state.nextLineInsertLeadingNewline = true;
	}

	// If a newline was requested consider us moved
	if (!opts.noNewline && !opts.preferNoNewline) {
		newline(stream, stderr);
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

function clearCurrentLine(stream: ReporterStreamAttached) {
	stream.write(ansiEscapes.cursorTo(0), false);
	stream.write(ansiEscapes.eraseLine, false);
	stream.state.nextLineInsertLeadingNewline = false;
}

function newline(stream: ReporterStreamAttached, stderr: boolean) {
	stream.state.leadingNewline = true;
	stream.state.currentLine = ob1Inc(stream.state.currentLine);
	stream.write("\n", stderr);
}

function logReplace(
	stream: ReporterStreamAttached,
	msg: string,
	targetLine: Number0,
	stderr: boolean,
) {
	const {lineDiff, bufferIndex} = calculateBufferPosition(stream, targetLine);

	// Easy modification when it's just the current line
	if (lineDiff === 0) {
		clearCurrentLine(stream);
		stream.state.buffer[bufferIndex] = msg;
		stream.write(msg, stderr);
		return;
	}

	// Advance to and replace the target line
	stream.state.buffer[bufferIndex] = msg;
	stream.write(ansiEscapes.cursorUp(lineDiff), false);
	stream.write(ansiEscapes.cursorTo(0), false);
	stream.write(ansiEscapes.eraseLine, false);
	stream.write(msg, false);

	// Advance back to the bottom
	if (lineDiff > 0) {
		stream.write(ansiEscapes.cursorDown(lineDiff), false);
		stream.write(ansiEscapes.cursorTo(0), false);
	}
}
