import {test} from "rome";
import {createIntegrationTest} from "@internal/test-helpers";
import LSPServer from "@internal/core/server/lsp/LSPServer";
import {JSONObject} from "@internal/codec-json";
import {Consumer, consumeUnknown} from "@internal/consume";

function makeRange(
	startLine: number,
	startChar: number,
	endLine: number,
	endChar: number,
) {
	return {
		start: {
			line: startLine,
			character: startChar,
		},
		end: {
			line: endLine,
			character: endChar,
		},
	};
}

function createMessage(res: JSONObject) {
	const json = JSON.stringify(res);
	return `Content-Length: ${Buffer.byteLength(json)}\r\n\r\n${json}`;
}

function consumeMessage(msg: string): Consumer {
	const content = msg.split("\r\n\r\n", 2)[1];
	const data = JSON.parse(content);
	return consumeUnknown(data, "lsp/parse");
}

test(
	"LSPServer",
	createIntegrationTest(
		{
			files: {"foo.ts": "let foo\nif (foo == 2) foo"},
		},
		async (t, h) => {
			const req = await h.createRequest();
			const lsp = new LSPServer(req);
			const {transport} = lsp;

			await h.server.projectManager.assertProject(h.cwd);

			function sendMessage(
				msg: JSONObject,
				callback: (c: Consumer, resolve: () => void) => void,
			) {
				const out = createMessage(msg);
				const promise = new Promise((resolve) => {
					transport.writeEvent.subscribe((response) => {
						const consumer = consumeMessage(response);
						callback(consumer, resolve);
					});
				});
				transport.append(out);
				return promise;
			}

			await sendMessage(
				{
					id: 10,
					method: "initialize",
					params: {rootUri: h.cwd.join()},
				},
				(msg, resolve) => {
					if (msg.get("id").asNumberOrVoid() === 10) {
						t.namedSnapshot("init", msg.asJSONObject());
						resolve();
					}
				},
			);
			await lsp.watchProjectEvent.wait();

			await sendMessage(
				{
					id: 20,
					method: "textDocument/formatting",
					params: {
						textDocument: {
							uri: h.cwd.append("foo.ts").join(),
						},
					},
				},
				(msg, resolve) => {
					if (msg.get("id").asNumberOrVoid() === 20) {
						t.namedSnapshot("formatting", msg.get("result").asJSONArray());
						resolve();
					}
				},
			);

			await sendMessage(
				{
					id: 30,
					method: "textDocument/codeAction",
					params: {
						textDocument: {
							uri: h.cwd.append("foo.ts").join(),
						},
						range: makeRange(0, 0, 2, 0),
					},
				},
				(msg, resolve) => {
					if (msg.get("id").asNumberOrVoid() === 30) {
						const codeActions = msg.get("result").asMappedArray<string>((c) =>
							c.get("title").asString()
						);
						t.namedSnapshot("code actions", codeActions);
						resolve();
					}
				},
			);

			const didOpen = createMessage({
				method: "textDocument/didOpen",
				params: {
					textDocument: {
						uri: h.cwd.append("foo.ts").join(),
						languageId: "typescript",
						version: 1,
						text: "const foo = () => {}",
					},
				},
			});
			transport.append(didOpen);

			await sendMessage(
				{
					id: 40,
					method: "textDocument/formatting",
					params: {
						textDocument: {
							uri: h.cwd.append("foo.ts").join(),
						},
					},
				},
				(msg, resolve) => {
					if (msg.get("id").asNumberOrVoid() === 40) {
						t.namedSnapshot(
							"formatting after didOpen",
							msg.get("result").asJSONArray(),
						);
						resolve();
					}
				},
			);

			await sendMessage(
				{
					id: 50,
					method: "workspace/executeCommand",
					params: {
						command: "rome.check.apply",
						arguments: [h.cwd.append("foo.ts").join()],
					},
				},
				(msg, resolve) => {
					if (msg.get("method").asStringOrVoid() === "workspace/applyEdit") {
						const edits = msg.get("params").get("edit").get("documentChanges").getIndex(
							0,
						).get("edits").asJSONArray();
						t.namedSnapshot("edits", edits);
						const response = createMessage({
							id: msg.get("id").asNumber(),
							result: {
								applied: true,
							},
						});
						transport.append(response);
						resolve();
					}
				},
			);

			await sendMessage(
				{
					id: 60,
					method: "shutdown",
				},
				(msg, resolve) => {
					if (msg.get("id").asNumberOrVoid() === 60) {
						resolve();
					}
				},
			);
		},
	),
);
