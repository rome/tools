import {preserveCasing} from "./preserveCasing";
import {test} from "rome";

test(
	"preserveCasing",
	(t) => {
		t.is(preserveCasing("test", "rome"), "rome");
		t.is(preserveCasing("tEsT", "rome"), "rome");
		t.is(preserveCasing("TEST", "rome"), "ROME");
		t.is(preserveCasing("test", "ROME"), "ROME");
		t.is(preserveCasing("Test", "rome"), "Rome");
		t.is(preserveCasing("test", "Rome"), "Rome");
	},
);
