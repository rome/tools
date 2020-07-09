import {Event} from "@romefrontend/events";
import {JSONObject, JSONPropertyValue} from "@romefrontend/codec-json";
import {Consumer, consumeUnknown} from "@romefrontend/consume";
import {LSPResponseMessage} from "./types";
import {Reporter} from "@romefrontend/cli-reporter";

type Status = "IDLE" | "WAITING_FOR_HEADERS_END" | "WAITING_FOR_RESPONSE_END";

type Headers = {
	expectedLength: number;
	extra: Map<string, string>;
};

const HEADERS_END = "\r\n\r\n";

function parseHeaders(buffer: string): Headers {
	const headers: Map<string, string> = new Map();

	for (const line of buffer.split("\n")) {
		const clean = line.trim();
		const match = clean.match(/^(.*?): (.*?)$/);
		if (match == null) {
			throw new Error(`Invalid header: ${clean}`);
		}

		const [, key, value] = match;
		headers.set(key.toLowerCase(), value);
	}

	const length = headers.get("content-length");
	if (length === undefined) {
		throw new Error("Expected Content-Length");
	}
	headers.delete("content-length");

	return {
		expectedLength: Number(length),
		extra: headers,
	};
}

type MessageEvent = {
	method: string;
	params: Consumer;
};

export class LSPTransport {
	constructor(reporter: Reporter) {
		this.status = "IDLE";
		this.nextHeaders = undefined;
		this.buffer = "";
		this.bufferLength = 0;
		this.reporter = reporter;

		this.requestEvent = new Event({name: "request"});
		this.notificationEvent = new Event({name: "notification"});
		this.writeEvent = new Event({name: "write"});
	}

	nextHeaders: undefined | Headers;
	status: Status;
	buffer: string;
	bufferLength: number;
	reporter: Reporter;

	notificationEvent: Event<MessageEvent, void>;
	requestEvent: Event<MessageEvent, JSONPropertyValue>;
	writeEvent: Event<string, void>;

	write(res: JSONObject) {
		const json = JSON.stringify(res);
		const out = `Content-Length: ${Buffer.byteLength(json)}${HEADERS_END}${json}`;
		this.writeEvent.send(out);
	}

	normalizeMessage(content: string): undefined | Consumer {
		try {
			const data = JSON.parse(content);
			const consumer = consumeUnknown(data, "lsp/parse");
			return consumer;
		} catch (err) {
			if (err instanceof SyntaxError) {
				console.error("JSON parse error", content);
				return undefined;
			} else {
				throw err;
			}
		}
	}

	async onMessage(headers: Headers, content: string) {
		const consumer = this.normalizeMessage(content);
		if (consumer === undefined) {
			return;
		}

		if (!consumer.has("method")) {
			return;
		}

		const method: string = consumer.get("method").asString();
		const params = consumer.get("params");

		if (consumer.has("id")) {
			const id = consumer.get("id").asNumber();

			try {
				let result = null;
				if (this.requestEvent.hasSubscriptions()) {
					result = await this.requestEvent.call({method, params});
				}
				const res: LSPResponseMessage = {
					id,
					result,
				};
				this.write(res);
			} catch (err) {
				const res: LSPResponseMessage = {
					id,
					error: {
						code: -32_603,
						message: err.message,
					},
				};
				this.write(res);
			}
		} else {
			await this.notificationEvent.callOptional({method, params});
		}
	}

	verboseLog(message: string, ...args: Array<unknown>) {
		this.reporter.verbose(`[LSPServer][Transport] ${message}`, ...args);
	}

	setStatus(status: Status) {
		this.status = status;
		this.verboseLog(`Status: ${status}`);
	}

	process() {
		switch (this.status) {
			case "IDLE": {
				if (this.bufferLength > 0) {
					this.setStatus("WAITING_FOR_HEADERS_END");
					this.process();
				}
				break;
			}

			case "WAITING_FOR_HEADERS_END": {
				const endIndex = this.buffer.indexOf(HEADERS_END);
				if (endIndex !== -1) {
					// Parse headers
					const rawHeaders = this.buffer.slice(0, endIndex);
					this.nextHeaders = parseHeaders(rawHeaders);
					this.verboseLog("Headers for next message:", this.nextHeaders);

					// Process rest of the buffer
					this.setStatus("WAITING_FOR_RESPONSE_END");
					this.buffer = this.buffer.slice(endIndex + HEADERS_END.length);
					this.process();
				}
				break;
			}

			case "WAITING_FOR_RESPONSE_END": {
				const headers = this.nextHeaders;
				if (headers === undefined) {
					throw new Error("Expected headers due to our status");
				}
				if (this.bufferLength >= headers.expectedLength) {
					const content = this.buffer.slice(0, headers.expectedLength);
					this.onMessage(headers, content);
					this.verboseLog("Received message content:", content);

					// Reset headers and trim content
					this.nextHeaders = undefined;
					this.buffer = this.buffer.slice(headers.expectedLength);
					this.bufferLength = Buffer.byteLength(this.buffer);

					// Process rest of the buffer
					this.setStatus("IDLE");
					this.process();
				}
				break;
			}
		}
	}

	append(data: string) {
		this.buffer += data;
		this.bufferLength += Buffer.byteLength(data);
		this.process();
	}
}
