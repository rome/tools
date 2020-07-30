import {isEscaped} from "./isEscaped";
import {ob1Coerce0} from "@internal/ob1";
import {test} from "rome";

test(
	"isEscaped",
	(t) => {
		t.false(isEscaped(ob1Coerce0(0), "\\"));

		t.false(isEscaped(ob1Coerce0(4), "test\\nrome"));

		t.true(isEscaped(ob1Coerce0(5), "test\\nrome"));

		t.false(isEscaped(ob1Coerce0(6), "test\\nrome"));

		t.false(isEscaped(ob1Coerce0(4), ""));

		t.true(isEscaped(ob1Coerce0(1), "\\\\"));

		t.false(isEscaped(ob1Coerce0(0), "\\\\"));
	},
);
