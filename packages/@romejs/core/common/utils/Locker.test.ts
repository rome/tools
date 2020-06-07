import Locker from "./Locker";
import {test} from "rome";

test(
	"Locker#getNewLock",
	async (t) => {
		const locker = new Locker<string>();

		// synchronously get a new lock
		const lock = locker.getNewLock("rome");
		t.true(locker.hasLock("rome"));

		// getNewLock throws if lock already exists
		t.throws(() => {
			locker.getNewLock("rome");
		});

		t.is(lock.locker, locker);
		t.is(lock.key, "rome");

		lock.release();
		t.false(locker.hasLock("rome"));
	},
);

test(
	"Locker#getLock",
	async (t) => {
		const locker = new Locker<string>();

		// asynchronously get a lock, waiting if that lock already exists
		const firstLock = await locker.getLock("rome");

		let foo = "before";

		async function bar() {
			// will wait here until firstLock is released
			const secondLock = await locker.getLock("rome");
			foo = "after";
			secondLock.release();
		}
		bar();

		t.is(foo, "before");

		firstLock.release();

		// will wait here until secondLock is released
		const thirdLock = await locker.getLock("rome");

		t.is(foo, "after");

		t.true(locker.hasLock("rome"));
		thirdLock.release();
		t.false(locker.hasLock("rome"));
	},
);

test(
	"Locker#waitLock",
	async (t) => {
		const locker = new Locker<string>();
		const lock = locker.getNewLock("rome");

		let foo = "before";

		async function bar() {
			// creates a lock internally that we don't have to release manually
			await locker.waitLock("rome");
			foo = "after";
		}
		bar();

		t.is(foo, "before");

		// this releases the original lock we made
		lock.release();

		// this second waitLock will wait for bar's waitLock
		const nothing = await locker.waitLock("rome");

		// waitLock returns Promise<void>
		t.is(nothing, undefined);

		t.is(foo, "after");

		// no locks because waitLock releases its own internal lock
		t.false(locker.hasLock("rome"));
	},
);
