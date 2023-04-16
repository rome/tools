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
a.b[`$c`];
a.b["_d"];
