import {test} from "rome";
import {splitEscapedObjectPath} from "./escapeObjectPaths";

test(
	"split object paths correctly",
	(t) => {
		t.looksLike(splitEscapedObjectPath(""), []);
		t.looksLike(splitEscapedObjectPath("abc"), ["abc"]);
		t.looksLike(splitEscapedObjectPath("a.b.c"), ["a", "b", "c"]);
		// repeated '.' or placed at edges should be ignored
		t.looksLike(splitEscapedObjectPath("a..b.c"), ["a", "b", "c"]);
		t.looksLike(splitEscapedObjectPath("a....b....c"), ["a", "b", "c"]);
		t.looksLike(splitEscapedObjectPath("a.b.c."), ["a", "b", "c"]);
		t.looksLike(splitEscapedObjectPath(".a.b.c"), ["a", "b", "c"]);
		// escaped '.' should be ignored
		t.looksLike(splitEscapedObjectPath("a\\.b.c"), ["a\\.b", "c"]);
		t.looksLike(splitEscapedObjectPath("a.b.c\\."), ["a", "b", "c\\."]);
	},
);
