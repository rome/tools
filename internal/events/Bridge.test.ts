import Bridge from "./Bridge";
import {test} from "rome";

test(
	"Bridge#handshake",
	async (t) => {
		const fooBridge = new Bridge({
			type: "server",
			sendMessage: (msg) => {
				barBridge.handleMessage(msg);
			},
		});

		const barBridge = new Bridge({
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

		let fooMessages: Array<string> = [];

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

		t.throwsAsync(async () => {
			// callOptional not allowed on BridgeEvents
			fooGreet.callOptional();
		});
	},
);

test(
	"Bridge server&client",
	async (t) => {
		const bridge = new Bridge({
			type: "server&client",
			sendMessage: (msg) => {
				bridge.handleMessage(msg);
			},
		});
		const fooGreet = bridge.createEvent<string, string>({
			name: "fooGreet",
			direction: "server<-client",
		});

		const barGreet = bridge.createEvent<string, string>({
			name: "barGreet",
			direction: "server->client",
		});

		const wrongGreet = bridge.createEvent<string, string>({
			name: "wrongGreet",
			direction: "server<->client",
		});

		t.throws(() => {
			// can't subscribe to server<->client event on a server&client bridge
			wrongGreet.subscribe((str) => str);
		});
		t.throwsAsync(async () => {
			// can't call a server<->client event on a server&client bridge
			await wrongGreet.call("wrong");
		});

		async function foo() {
			fooGreet.subscribe((str) => `hey, ${str}`);
			await bridge.handshake();

			const res = await barGreet.call("foo");
			t.is(res, "greetings, foo");
		}

		async function bar() {
			barGreet.subscribe((str) => `greetings, ${str}`);
			await bridge.handshake();

			const res = await fooGreet.call("bar");
			t.is(res, "hey, bar");
		}

		foo();
		await bar();
	},
);

test(
	"Bridge#end",
	async (t) => {
		const bridge = new Bridge({
			type: "server&client",
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

		bridge.end("Halt!");

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
