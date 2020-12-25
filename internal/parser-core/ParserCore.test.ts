import {test} from "rome";
import {parseJS} from "@internal/js-parser";

function helper(input: string) {
	return parseJS({
		path: "unknown",
		input,
	});
}

test(
	"verify parsing",
	(t) => {
		t.snapshot(helper("1 && 2 ?? 3"));

		t.snapshot(helper("import().then(doThat);"));

		t.snapshot(helper("for (var [p]=0 in q);"));

		t.snapshot(helper("(x,y,z,) => 0"));

		t.snapshot(helper("{ const x = 42 }"));

		t.snapshot(helper("<img width={320}/>"));

		t.snapshot(helper("class A {get a(){} set b(c){};}"));

		t.snapshot(helper("x * y ** -z"));

		t.snapshot(helper("var o = {one: function() {} two:2};"));

		t.snapshot(helper("{ doThis(); doThat(); }"));

		t.snapshot(helper(`"\\x";`));

		t.snapshot(helper(`var source = '{0\x0A1\x0D2\u20283\u20294}';`));
	},
);
