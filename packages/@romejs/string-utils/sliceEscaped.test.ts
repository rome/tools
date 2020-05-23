import {sliceEscaped} from "./sliceEscaped";
import {test} from "rome";

test(
	"sliceEscaped",
	(t) => {
		t.is(sliceEscaped("\\\\", 1), "\\");

		t.is(sliceEscaped("test\\nrome", 4), "test");

		t.is(sliceEscaped("test\\nrome", 5), "test");

		t.is(sliceEscaped("test\\nrome", 6), "test\\n");

		t.is(sliceEscaped("test", 6), "test");

		t.is(sliceEscaped("", 4), "");
	},
);
