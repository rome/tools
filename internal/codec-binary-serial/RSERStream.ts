import RSERBufferWriter from "./RSERBufferWriter";
import {Event} from "@internal/events";
import {RSERValue} from "./types";
import RSERBufferParser from "./RSERBufferParser";
import {encodeValueToRSERBufferMessage} from "@internal/codec-binary-serial/index";
import RSERBufferAssembler from "./RSERBufferAssembler";
import {VERSION} from "./constants";

type State = {
	// INIT: Waiting on stream header
	// IDLE: Waiting on the next message and a full PDU length to decode
	// READ: Know the length, need to read whole content
	type: "INIT" | "IDLE" | "READ";
	writer: RSERBufferWriter;
	reader: RSERBufferParser;
};

type RSERStreamType = "client" | "server" | "file";

// 1 bit for the type + 4 bits for the int
// NOTE: bigint is 8 bits but will never appear in the positions we care about here
const MAX_INT_SIZE = 5;

// 1 header code bit + int bits
const MAX_STREAM_HEADER_SIZE = MAX_INT_SIZE + 1;
const MAX_MESSAGE_HEADER_SIZE = MAX_INT_SIZE + 1;

function createState(type: State["type"], size: number): State {
	// Max possible size of a message header
	const writer = RSERBufferWriter.allocate(size);

	return {
		type,
		reader: writer.toParser(),
		writer,
	};
}

export default class RSERStream {
	constructor(type: RSERStreamType) {
		this.type = type;
		this.state = createState("INIT", MAX_STREAM_HEADER_SIZE);
		this.overflow = [];

		this.errorEvent = new Event({
			name: "RSERStream.error",
		});

		this.valueEvent = new Event({
			name: "RSERStream.value",
		});

		this.sendEvent = new Event({
			name: "RSERStream.sendEvent",
		});
	}

	public errorEvent: Event<Error, void>;
	public sendEvent: Event<ArrayBuffer, void>;
	public valueEvent: Event<RSERValue, void>;

	private type: RSERStreamType;
	private overflow: Uint8Array[];
	private state: State;

	public sendValue(val: RSERValue) {
		this.sendBuffer(encodeValueToRSERBufferMessage(val));
	}

	public sendBuffer(buf: ArrayBuffer) {
		try {
			this.sendEvent.send(buf);
		} catch (err) {
			this.errorEvent.send(err);
		}
	}

	public append(buf: ArrayBuffer) {
		// Fast path for empty buffer
		if (buf.byteLength === 0) {
			return;
		}

		const {writer, type} = this.state;

		try {
			// Fast path for appending a full message
			if (
				type === "IDLE" &&
				writer.writeOffset === 0 &&
				buf.byteLength > MAX_MESSAGE_HEADER_SIZE
			) {
				const reader = new RSERBufferParser(new DataView(buf));
				const payloadLength = reader.maybeDecodeMessageHeader();
				if (
					payloadLength !== false &&
					payloadLength === reader.getReadableSize()
				) {
					const val = reader.decodeValue();
					this.valueEvent.send(val);
					return;
				}
			}

			// Push to overflow queue if necessary
			let arr = new Uint8Array(buf);

			// If the buffer is bigger than the current message we expect then cut it up
			const remaining = writer.getWritableSize();
			if (remaining < arr.byteLength) {
				// Slicing Node buffers is cheap since it just creates a view
				this.overflow.push(arr.slice(remaining));
				arr = arr.slice(0, remaining);
			}

			writer.appendBytes(arr);
			this.process();
		} catch (err) {
			this.errorEvent.send(err);
		}
	}

	// This marks the end of the buffer we want so add the rest of the data to the overflow so the next state receives it
	private unshiftUnreadOverflow() {
		const {reader, writer} = this.state;
		const leftover = reader.getReadableSize();

		if (leftover > 0 && reader.readOffset < writer.writeOffset) {
			const bytes = writer.bytes.slice(reader.readOffset, writer.writeOffset);
			this.overflow.unshift(bytes);
		}
	}

	private setState(state: State) {
		try {
			this.state = state;
			const {writer} = this.state;

			// Keep filling and processing the buffer with overflowed data until it's exhausted
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

				writer.appendBytes(entry);

				this.process();
			}
		} catch (err) {
			this.errorEvent.send(err);
		}
	}

	public init() {
		if (this.type === "client") {
			this.sendStreamHeader();
		}
	}

	// Send stream header
	public sendStreamHeader() {
		const assembler = new RSERBufferAssembler();
		assembler.encodeStreamHeader(VERSION);

		const buf = new RSERBufferWriter(
			new ArrayBuffer(assembler.totalSize),
			assembler,
		);
		buf.encodeStreamHeader(VERSION);
		this.sendBuffer(buf.buffer);
	}

	private process(): void {
		const {type, writer, reader} = this.state;

		// Decode stream header
		if (type === "INIT") {
			const validHeader = reader.maybeDecodeStreamHeader();
			if (validHeader) {
				this.unshiftUnreadOverflow();
				this.setState(createState("IDLE", MAX_MESSAGE_HEADER_SIZE));

				// Server always sends their header after the client bv
				if (this.type === "server") {
					this.sendStreamHeader();
				}
			}
			return;
		}

		// Waiting for message header
		if (type === "IDLE") {
			const payloadLength = reader.maybeDecodeMessageHeader();
			if (payloadLength === false) {
				return;
			}

			this.unshiftUnreadOverflow();
			this.setState(createState("READ", payloadLength));
		}

		// Reading message
		if (type === "READ") {
			if (writer.getWritableSize() > 0) {
				// Need more data
				return;
			}

			// We have enough to decode it
			const val = reader.decodeValue();
			this.valueEvent.send(val);
			this.setState(createState("IDLE", MAX_MESSAGE_HEADER_SIZE));
		}
	}
}
