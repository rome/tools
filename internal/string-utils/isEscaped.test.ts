import {isEscaped} from "./isEscaped";
import {test} from "rome";
import {ZeroIndexed} from "@internal/math";

test(
	"isEscaped",
	(t) => {
		t.false(isEscaped(new ZeroIndexed(), "\\"));

		t.false(isEscaped(new ZeroIndexed(4), "test\\nrome"));

		t.true(isEscaped(new ZeroIndexed(5), "test\\nrome"));

		t.false(isEscaped(new ZeroIndexed(6), "test\\nrome"));

		t.false(isEscaped(new ZeroIndexed(4), ""));

		t.true(isEscaped(new ZeroIndexed(1), "\\\\"));

		t.false(isEscaped(new ZeroIndexed(), "\\\\"));
	},
);
