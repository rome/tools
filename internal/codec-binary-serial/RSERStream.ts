import RSERBufferWriter from "./RSERBufferWriter";
import {Event} from "@internal/events";
import {RSERValue} from "./types";
import RSERBufferParser from "./RSERBufferParser";

type State = {
	// Waiting: Need to read and decode PDU length
	// Reading: Know the length, need to read whole content
	type: "WAITING" | "READING";
	writer: RSERBufferWriter;
};

export default class RSERStream {
	constructor() {
		this.processLaterQueued = false;
		this.state = this.createWaitingState();
		this.overflow = [];

		this.errorEvent = new Event({
			name: "RSERStream.error",
		});

		this.valueEvent = new Event({
			name: "RSERStream.value",
		});
	}

	errorEvent: Event<Error, void>;
	valueEvent: Event<RSERValue, void>;
	overflow: Array<Uint8Array>;
	processLaterQueued: boolean;
	state: State;

	createWaitingState(): State {
		return {
			type: "WAITING",
			// Max size of the message header
			writer: RSERBufferWriter.allocate(7),
		};
	}

	append(buf: Uint8Array) {
		const {writer, type} = this.state;

		try {
			// Fast path for appending a full message
			if (type === "WAITING" && writer.writeOffset === 0 && buf.byteLength > 7) {
				const reader = new RSERBufferParser(new DataView(buf));
				const payloadLength = reader.decodeHeader();
				if (
					payloadLength !== false &&
					payloadLength === reader.getReadableSize()
				) {
					this.valueEvent.send(reader.decodeValue());
					return;
				}
			}

			// Push to overflow queue if necessary
			const remaining = writer.getWritableSize();
			if (remaining < buf.byteLength) {
				// Slicing Node buffers is cheap since it just creates a view
				this.overflow.push(buf.slice(remaining));
				buf = buf.slice(0, remaining);
			}

			writer.appendArray(buf);
			this.process();
		} catch (err) {
			this.errorEvent.send(err);
		}
	}

	setState(state: State) {
		this.state = state;

		const {writer} = this.state;

		while (this.overflow.length > 0 && writer.getWritableSize() > 0) {
			let entry = this.overflow[0];
			const writableSize = writer.getWritableSize();

			const bufferSize = Buffer.byteLength(entry);
			if (bufferSize > writableSize) {
				this.overflow[0] = entry.slice(writableSize);
				entry = entry.slice(0, writableSize);
			} else {
				this.overflow.shift();
			}

			writer.appendArray(entry);

			this.process();
		}
	}

	process(): void {
		const {type, writer} = this.state;

		if (type === "WAITING") {
			if (writer.writeOffset < 2) {
				return;
			}

			const reader = new RSERBufferParser(writer.view);
			const payloadLength = reader.decodeHeader();
			if (payloadLength === false) {
				return;
			}

			// The header buffer is set to the maximum size it could be, but there could still be data left so push it on.
			const leftover = reader.getReadableSize();
			const payloadWriter = RSERBufferWriter.allocate(payloadLength + leftover);
			if (leftover > 0) {
				payloadWriter.appendArray(writer.array.slice(reader.readOffset));
			}

			this.setState({
				type: "READING",
				writer: payloadWriter,
			});
		}

		if (type === "READING") {
			if (writer.getWritableSize() > 0) {
				// Need more data
				return;
			}

			// We have enough to decode it
			const reader = new RSERBufferParser(writer.view);
			const val = reader.decodeValue();
			this.valueEvent.send(val);
			this.setState(this.createWaitingState());
		}
	}
}
