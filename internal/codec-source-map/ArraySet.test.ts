import {test} from "rome";
import ArraySet from "./ArraySet";

test(
	"strings added to the set exist",
	(t) => {
		const set = new ArraySet();
		set.add("test string");
		t.true(set.indexOf("test string") === 0);
	},
);

test(
	"indexOf returns index of first match when allowing duplicates ",
	(t) => {
		const set = new ArraySet();
		set.add("test string");
		set.add("in the middle");
		set.add("test string", true);
		t.true(set.indexOf("test string") === 0);
	},
);

test(
	"indexOf throws when a string is not present",
	(t) => {
		const set = new ArraySet();
		t.throws(() => void set.indexOf("missing string"));
	},
);

test(
	"toArray provides a new copy of the underlying data",
	(t) => {
		const set = new ArraySet();
		set.add("test string");
		const array = set.toArray();
		array.push("missing string");
		t.throws(() => void set.indexOf("missing string"));
	},
);
