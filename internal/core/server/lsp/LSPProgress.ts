import {
	Reporter,
	ReporterProgressBase,
	ReporterProgressOptions,
} from "@internal/cli-reporter";
import {LSPTransport} from "./protocol";
import {markupToJoinedPlainText} from "@internal/cli-layout";

let progressTokenCounter = 0;

// The server can sometimes emit progress bars to all connected clients.
// This allows us to send those same progress events over the LSP.

export default class LSPProgress extends ReporterProgressBase {
	constructor(
		transport: LSPTransport,
		reporter: Reporter,
		opts?: ReporterProgressOptions,
	) {
		super(reporter, opts);
		this.transport = transport;
		this.token = progressTokenCounter++;
		this.lastRenderKey = "";
		this.hasBegun = false;

		// Progress notifications disabled for now because of performance impact
		this.pause();
	}

	private lastRenderKey: string;
	private token: number;
	private transport: LSPTransport;
	private hasBegun: boolean;

	private begin(percentage: number) {
		this.transport.write({
			method: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "begin",
					cancellable: false,
					title: this.title,
					percentage,
				},
			},
		});
	}

	public render() {
		if (this.paused) {
			return;
		}
		const total = this.total === undefined ? 0 : this.total;
		const percentage = Math.floor(100 / total * this.current);

		// Make sure we don't send pointless duplicate messages
		const renderKey = `percent:${percentage},text:${this.text}`;
		if (this.lastRenderKey === renderKey) {
			return;
		}

		this.lastRenderKey = renderKey;

		if (this.hasBegun) {
			this.report(percentage);
		} else {
			this.begin(percentage);
			this.hasBegun = true;
		}
	}

	public end() {
		if (!this.hasBegun) {
			return;
		}

		this.transport.write({
			method: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "end",
				},
			},
		});
	}

	private report(percentage: number) {
		this.transport.write({
			method: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "report",
					cancellable: false,
					message: this.text === undefined
						? ""
						: markupToJoinedPlainText(this.text),
					percentage,
				},
			},
		});
	}
}
