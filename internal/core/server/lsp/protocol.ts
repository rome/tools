import {Event} from "@internal/events";
import {JSONObject, JSONPropertyValue} from "@internal/codec-json";
import {Consumer, consumeUnknown} from "@internal/consume";
import {LSPRequestMessage, LSPResponseMessage} from "./types";
import {Reporter} from "@internal/cli-reporter";
import {AnyMarkup, markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";

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
		this.requestIdCounter = 0;
		this.requestCallbacks = new Map();

		this.requestEvent = new Event({name: "request"});
		this.notificationEvent = new Event({name: "notification"});
		this.writeEvent = new Event({name: "write"});
		this.errorEvent = new Event({name: "error"});
	}

	private nextHeaders: undefined | Headers;
	private status: Status;
	private buffer: string;
	private bufferLength: number;
	private reporter: Reporter;
	private requestIdCounter: number;
	private requestCallbacks: Map<
		number,
		{
			resolve: (data: Consumer) => void;
			reject: (err: Consumer) => void;
		}
	>;

	public notificationEvent: Event<MessageEvent, void>;
	public errorEvent: Event<Error, void>;
	public requestEvent: Event<MessageEvent, JSONPropertyValue>;
	public writeEvent: Event<string, void>;

	public write(res: JSONObject) {
		const json = JSON.stringify(res);
		const out = `Content-Length: ${Buffer.byteLength(json)}${HEADERS_END}${json}`;
		this.writeEvent.send(out);
	}

	public async request(req: Omit<LSPRequestMessage, "id">): Promise<Consumer> {
		return new Promise((resolve, reject) => {
			const id = ++this.requestIdCounter;
			this.requestCallbacks.set(id, {resolve, reject});
			this.write({...req, id});
		});
	}

	private normalizeMessage(content: string): undefined | Consumer {
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

	private async onMessage(headers: Headers, content: string) {
		const consumer = this.normalizeMessage(content);
		if (consumer === undefined) {
			return;
		}

		if (!consumer.has("method")) {
			if (!consumer.has("id")) {
				return;
			}
			const id = consumer.get("id").asNumber();
			const callbacks = this.requestCallbacks.get(id);
			this.requestCallbacks.delete(id);
			if (callbacks === undefined) {
				return;
			}
			if (consumer.has("result")) {
				callbacks.resolve(consumer.get("result"));
			} else if (consumer.has("error")) {
				callbacks.reject(consumer.get("error"));
			}
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

	private log(message: AnyMarkup) {
		this.reporter.info(markup` ${message}`);
	}

	private setStatus(status: Status) {
		this.status = status;
		this.log(markup`Status: ${status}`);
	}

	private process() {
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
					this.log(
						markup`Headers for next message: ${prettyFormat(this.nextHeaders)}`,
					);

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
					this.onMessage(headers, content).catch((err) => {
						this.errorEvent.send(err, true);
					});
					this.log(markup`Received message content: ${content}`);

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

	public append(data: string) {
		this.buffer += data;
		this.bufferLength += Buffer.byteLength(data);
		this.process();
	}
}
