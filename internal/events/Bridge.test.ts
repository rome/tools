import Bridge from "./Bridge";
import {test} from "rome";

test(
	"Bridge#handshake",
	async (t) => {
		const fooBridge = new Bridge({
			debugName: "foo",
			type: "server",
			sendMessage: (msg) => {
				barBridge.handleMessage(msg);
			},
		});

		const barBridge = new Bridge({
			debugName: "bar",
			type: "client",
			sendMessage: (msg) => {
				fooBridge.handleMessage(msg);
			},
		});

		async function foo() {
			test = "two";
			await fooBridge.handshake();
			test = "three";
		}
		async function bar() {
			t.is(test, "two");
			await barBridge.handshake();
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
		const fooBridge = new Bridge({
			debugName: "foo",
			type: "server",
			sendMessage: (msg) => {
				barBridge.handleMessage(msg);
			},
		});
		const fooGreet = fooBridge.createEvent<string, string>({
			name: "greet",
			direction: "server<-client",
		});

		const barBridge = new Bridge({
			debugName: "bar",
			type: "client",
			sendMessage: (msg) => {
				fooBridge.handleMessage(msg);
			},
		});
		const barGreet = barBridge.createEvent<string, string>({
			name: "greet",
			direction: "server<-client",
		});

		async function foo() {
			await fooBridge.handshake();

			const res = await fooGreet.call("foo");
			t.is(res, "Hello, foo");
		}

		async function bar() {
			barGreet.subscribe((name) => {
				fooMessages.push(name);
				return `Hello, ${name}`;
			});
			await barBridge.handshake();
		}

		let fooMessages: string[] = [];

		foo();
		await bar();

		t.looksLike(fooBridge.getSubscriptions(), ["Bridge.heartbeat"]);
		t.looksLike(barBridge.getSubscriptions(), ["Bridge.heartbeat", "greet"]);

		t.looksLike(fooMessages, ["foo"]);

		fooGreet.send("cat");
		await fooGreet.call("dog");

		t.looksLike(fooMessages, ["foo", "cat", "dog"]);

		t.throws(() => {
			// can't create duplicate event on a bridge
			fooBridge.createEvent<string, string>({
				name: "greet",
				direction: "server<-client",
			});
		});

		t.throws(() => {
			// server bridges can't subscribe to server<-client events
			fooGreet.subscribe((str) => str);
		});

		t.throwsAsync(async () => {
			// client bridges can't call server<-client events
			await barGreet.call("bar");
		});
	},
);

test(
	"Bridge#end",
	async (t) => {
		const bridge = new Bridge({
			debugName: "foo",
			type: "server",
			sendMessage: (msg) => {
				bridge.handleMessage(msg);
			},
		});
		const greet = bridge.createEvent<string, string>({
			name: "greet",
			direction: "server<-client",
		});

		bridge.handshake();
		await bridge.handshake();

		const greetSub = greet.subscribe((str) => `hello ${str}`);
		const res = await greet.call("rome");
		t.is(res, "hello rome");

		bridge.attachEndSubscriptionRemoval(greetSub);

		t.looksLike(bridge.getSubscriptions(), ["greet"]);
		t.true(greet.hasSubscriptions());

		await bridge.end("Halt!");

		t.looksLike(bridge.getSubscriptions(), []);
		t.false(greet.hasSubscriptions());

		t.throwsAsync(async () => {
			// Bridge is dead
			await greet.call("test");
		});

		t.throws(() => {
			bridge.assertAlive();
		});
	},
);
