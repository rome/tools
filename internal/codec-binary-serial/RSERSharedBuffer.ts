import {RSERValue} from "./types";
import {RSERBufferObserver} from "./index";
import {Event} from "@internal/events";
import RSERBufferParser from "./RSERBufferParser";
import RSERBufferWriter from "./RSERBufferWriter";

type Buffers = {
	input: SharedArrayBuffer;
	output: SharedArrayBuffer;
};

export default class RSERSharedBuffer {
	constructor(buffers: Buffers) {
		this.input = buffers.input;
		this.inputReader = new RSERBufferParser(new DataView(this.input));
		this.inputLock = new Int32Array(this.input, 0, 1);

		this.output = buffers.output;
		this.outputWriter = new RSERBufferWriter(this.output);
		this.outputLock = new Int32Array(this.output, 0, 1);

		this.valueEvent = new Event({
			name: "RSERStream.value",
		});

		this.sendEvent = new Event({
			name: "RSERStream.sendEvent",
		});
	}

	valueEvent: Event<RSERValue, void>;
	sendEvent: Event<ArrayBuffer, void>;

	input: SharedArrayBuffer;
	inputReader: RSERBufferParser;
	inputLock: Int32Array;

	output: SharedArrayBuffer;
	outputWriter: RSERBufferWriter;
	outputLock: Int32Array;

	static create(inputSize: number, outputSize: number) {
		return new RSERSharedBuffer({
			input: new SharedArrayBuffer(inputSize),
			output: new SharedArrayBuffer(outputSize),
		});
	}

	getFlippedBuffers(): Buffers {
		return {
			output: this.input,
			input: this.output,
		};
	}

	processOutBand(shared: SharedArrayBuffer) {
		const parser = new RSERBufferParser(new DataView(shared));

		const payloadLength = parser.decodeHeader();
		if (payloadLength === false) {
			throw new Error("No header payload length. Expected for a full message.");
		}
		if (parser.getReadableSize() !== payloadLength) {
			throw new Error(
				`Payload length ${payloadLength} should match the rest of the buffer ${parser.getReadableSize()}`,
			);
		}

		this.valueEvent.send(parser.decodeValue());
	}

	send(val: RSERValue) {
		const {payloadLength, messageLength} = RSERBufferObserver.measure(val);
		// Need a dedicated out band message as it exceeds our buffer
		const buf = RSERBufferWriter.allocate(messageLength);
		buf.encodeHeader(payloadLength);
		buf.encodeValue(val);
		this.sendEvent.send(buf.buffer);
		return;

		// TODO Without Atomics.waitAsync I don't think we'll be able to signal communication over the shared buffer
		/*if (messageLength < this.output.byteLength) {
			const {outputWriter} = this;
			outputWriter.writeOffset = 0;
			outputWriter.encodeHeader(payloadLength);
			outputWriter.encodeValue(val);
			Atomics.notify(this.outputLock, 0)
		}*/
	}
}
