import {removeCarriageReturn} from "./removeCarriageReturn";
import {test} from "rome";

test(
	"removeCarriageReturn",
	(t) => {
		t.is(removeCarriageReturn("test\rrome"), "testrome");
		t.is(removeCarriageReturn("test\nrome"), "test\nrome");
		t.is(removeCarriageReturn("test\r\nrome"), "test\nrome");
	},
);
