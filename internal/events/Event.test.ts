import Event from "./Event";
import {test} from "rome";

type Callback<Param, Ret> = (param: Param) => Ret | Promise<Ret>;

test(
	"Event basic",
	async (t) => {
		const event = new Event<string, void>({name: "testEvent"});
		const fooCalls: Array<string> = [];
		const foo: Callback<string, void> = (param) => {
			fooCalls.push(param);
		};

		t.false(event.hasSubscribers());
		t.false(event.hasSubscriptions());

		event.subscribe(foo);

		t.true(event.hasSubscriptions());

		event.send("hello");
		event.send("rome");

		event.clear();
		t.false(event.hasSubscriptions());

		// send and callOptional don't throw if there are no subscriptions
		event.send("ok");
		await event.callOptional("ok");

		// send with a required param does throw
		t.throws(() => {
			event.send("ok", true);
		});

		// call does throw if there are no subscriptions
		t.throwsAsync(async () => {
			await event.call("error");
		});

		t.looksLike(fooCalls, ["hello", "rome"]);
	},
);

test(
	"Event send void",
	async (t) => {
		const event = new Event<string, string>({name: "testEvent"});
		const fooCalls: Array<string> = [];
		const foo: Callback<string, string> = (param) => {
			fooCalls.push(param);
			return "foo returns";
		};

		event.subscribe(foo);
		const callRet = await event.call("hello");
		const sendRet = event.send("rome");

		// return value from event.call is return value of rootSubscription
		t.is(callRet, "foo returns");

		// send has no return value
		t.is(sendRet, undefined);

		t.looksLike(fooCalls, ["hello", "rome"]);
	},
);

test(
	"Event subscription order",
	async (t) => {
		const event = new Event<string, string>({name: "testEvent"});
		const fooCalls: Array<string> = [];
		const foo: Callback<string, string> = (param) => {
			fooCalls.push(param);
			return "foo returns";
		};
		const barCalls: Array<string> = [];
		const bar: Callback<string, string> = (param) => {
			barCalls.push(param);
			return "bar returns";
		};

		// foo becomes the rootSubscription
		const fooSub = event.subscribe(foo);
		const first = await event.call("hello");

		// foo is still the rootSubscription
		event.subscribe(bar);
		const second = await event.call("rome");

		// bar becomes the rootSubscription
		fooSub.unsubscribe();
		const third = await event.call("test");

		// make foo the rootSubscription
		event.subscribe(foo, true);
		const fourth = await event.call("hi");

		// return value from event.call is return value of rootSubscription
		t.is(first, "foo returns");
		t.is(second, "foo returns");
		t.is(third, "bar returns");
		t.is(fourth, "foo returns");

		t.looksLike(fooCalls, ["hello", "rome", "hi"]);
		t.looksLike(barCalls, ["rome", "test", "hi"]);
	},
);

test(
	"Event#wait",
	async (t) => {
		const event = new Event<string, string>({name: "testEvent"});

		const waitPromise = event.wait("wait arg");

		const foo = await event.call("wait for this");
		t.is(foo, "wait arg");

		t.is(await waitPromise, "wait for this");

		t.throwsAsync(async () => {
			await event.wait("will timeout", 0);
		});
	},
);
