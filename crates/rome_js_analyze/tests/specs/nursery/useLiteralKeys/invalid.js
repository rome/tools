a["b"];
a.b["c"];
a.b["c"].d.e["f"];
a.b[`c`];
a.b[c["d"]];
a["b"] = "something";
a.b["c"] = "something";
a.b["c"].d.e["f"] = "something";
a.b[`c`] = "something";
a.b[c["d"]] = "something";
a = {
	['b']: d
};
a = {
	[`b`]: d
};
a = {
	"b": d
};
a.b[`$c`];
a.b["_d"];
class C { ["a"] = 0 }
class C { "a" = 0 }
class C { ["a"](){} }
class C { "a"(){} }
class C { get ["a"](){} }
class C { get "a"(){} }
class C { set ["a"](x){} }
class C { set "a"(x){} }
a = {
	["1+1"]: 2
}
a = {
	[`1+1`]: 2
}