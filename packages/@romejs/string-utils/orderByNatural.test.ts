import {orderByNatural} from "./orderByNatural";
import {test} from "rome";

test(
	"orderByNatural",
	(t) => {
		t.looksLike(
			orderByNatural(["1test", "test", "test1", "test2", "test001", "1"]),
			["1", "1test", "test", "test1", "test001", "test2"],
		);

		t.looksLike(
			orderByNatural(["cat", "Dog", "1", "100", "-100", "007"]),
			["-100", "1", "007", "100", "cat", "Dog"],
		);

		t.looksLike(
			orderByNatural(["cat", "Dog", "1", "100", "-100", "007"], false),
			["-100", "1", "007", "100", "Dog", "cat"],
		);
	},
);
