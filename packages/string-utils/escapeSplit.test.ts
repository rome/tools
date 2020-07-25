import {escapeSplit} from "./escapeSplit";
import {test} from "rome";

test(
	"escapeSplit",
	(t) => {
		t.looksLike(escapeSplit("test-rome", "-"), ["test", "rome"]);

		t.looksLike(escapeSplit("test-rome", "&"), ["test-rome"]);

		t.looksLike(escapeSplit("test-rome\\-test", "-"), ["test", "rome-test"]);

		t.looksLike(escapeSplit("test\nrome\\\ntest", "\n"), ["test", "rome\ntest"]);
	},
);
