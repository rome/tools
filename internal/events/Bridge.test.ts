import {test} from "rome";
import createBridge, {createBridgeEventDeclaration} from "./createBridge";

test(
	"Bridge#handshake",
	async (t) => {
		const {server, client} = createBridge({
			debugName: "Test",
			server: {},
			client: {},
			shared: {},
		}).createFromLocal();

		async function foo() {
			test = "two";
			await server.handshake();
			test = "three";
		}
		async function bar() {
			t.is(test, "two");
			await client.handshake();
			t.is(test, "three");
		}

		let test = "one";

		foo();
		t.is(test, "two");

		await bar();
		t.is(test, "three");
	},
);

test(
	"BridgeEvent",
	async (t) => {
		const {
			server,
			client,
		} = createBridge({
			debugName: "Test",
			server: {},
			client: {},
			shared: {
				greet: createBridgeEventDeclaration<string, string>(),
			},
		}).createFromLocal();

		async function foo() {
			await server.handshake();

			const res = await server.events.greet.call("foo");
			t.is(res, "Hello, foo");
		}

		async function bar() {
			client.events.greet.subscribe((name) => {
				fooMessages.push(name);
				return `Hello, ${name}`;
			});
			await client.handshake();
		}

		let fooMessages: string[] = [];

		foo();
		await bar();

		t.looksLike(server.getSubscriptions(), ["Bridge.heartbeat"]);
		t.looksLike(client.getSubscriptions(), ["Bridge.heartbeat", "greet"]);

		t.looksLike(fooMessages, ["foo"]);

		server.events.greet.send("cat");
		await server.events.greet.call("dog");

		t.looksLike(fooMessages, ["foo", "cat", "dog"]);
	},
);

test(
	"Bridge#end",
	async (t) => {
		const {
			server,
			client,
		} = createBridge({
			debugName: "Test",
			server: {},
			client: {},
			shared: {
				greet: createBridgeEventDeclaration<string, string>(),
			},
		}).createFromLocal();

		server.handshake();
		await client.handshake();

		const greetSub = server.events.greet.subscribe((str) => `hello ${str}`);
		const res = await client.events.greet.call("rome");
		t.is(res, "hello rome");

		client.attachEndSubscriptionRemoval(greetSub);

		t.looksLike(client.getSubscriptions(), ["greet"]);
		t.true(client.events.greet.hasSubscriptions());

		await client.end("Halt!");

		t.looksLike(client.getSubscriptions(), []);
		t.false(client.events.greet.hasSubscriptions());

		t.throwsAsync(async () => {
			// Bridge is dead
			await client.events.greet.call("test");
		});

		t.throws(() => {
			client.assertAlive();
		});
	},
);
