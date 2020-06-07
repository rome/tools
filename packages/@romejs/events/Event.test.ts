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
		t.is(foo, event.rootSubscription);

		event.send("hello");
		event.send("rome");

		event.clear();
		t.false(event.hasSubscriptions());

		// send and callOptional don't throw if there are no subscriptions
		event.send("ok");
		await event.callOptional("ok");

		// call and callSync do throw if there are no subscriptions
		t.throwsAsync(async () => {
			await event.call("error");
		});
		t.throws(() => {
			event.callSync("error");
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
	"Event#callSync with promise subscription",
	async (t) => {
		const event = new Event<string, string>({name: "testEvent"});
		const foo: Callback<string, string> = (param) => {
			return "foo returns";
		};
		const bar: Callback<string, string> = (param) => {
			return Promise.resolve("bar returns a promise");
		};

		event.subscribe(foo);
		event.subscribe(bar);
		const ret = await event.call("rome");

		// callSync throws if any subscription returns a promise
		t.throws(() => {
			event.callSync("test");
		});
		t.is(ret, "foo returns");
	},
);

test(
	"Event#onError",
	async (t) => {
		const errors: Array<Error> = [];
		const event = new Event<string, string>({
			name: "testEvent",
			onError: (err) => {
				errors.push(err);
			},
		});

		const testError = new Error("oops");

		// this calls the error handler
		event.onError(testError);

		t.throws(() => {
			event.callSync("no subscriptions");
		});

		t.is(errors[0], testError);
		t.is(errors.length, 2);
	},
);

test(
	"Event#wait",
	async (t) => {
		const event = new Event<string, string>({name: "testEvent"});
		let foo: string;

  await new Promise((resolve) => {
		setTimeout(
			() => {
				foo = event.callSync("wait for this");
				t.is(foo, "wait arg");
        resolve();
			},
			100,
		);
  });

		const waitValue = await event.wait("wait arg");
		t.is(waitValue, "wait for this");

		t.throwsAsync(async () => {
			await event.wait("will timeout", 0);
		});
	},
);
