import {naturalCompare} from "./naturalCompare";
import {test} from "rome";

test(
	"naturalCompare",
	(t) => {
		t.is(naturalCompare("1", "2"), -1);
		t.is(naturalCompare("100", "2"), 2);
		t.is(naturalCompare("-100", "2"), -5);
		t.is(naturalCompare("007", "8"), -1);
		t.is(naturalCompare("007", "7"), 2);
		t.is(naturalCompare("test1", "9000"), 59);
		t.is(naturalCompare("1testrome", "2t"), -1);
		t.is(naturalCompare("1test", "1TEST"), 0);
		t.is(naturalCompare("1test", "1TEST", false), 32);
	},
);
