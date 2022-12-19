// invalid
var a = <input className="" aria-labell="" />;
var a = <div aria-="foobar" />;
var a = <div aria-labeledby="foobar" />;
var a = <div aria-skldjfaria-klajsd="foobar" />;
var a = <div aria-skldjfaria-klajsd="foobar"  aria-skldjfaria-klajsd="foobar" />;

// valid
var a = <div />;
var a = <div></div>;
var a = <div aria="wee"></div>;
var a = <div abcARIAdef="true"></div>;
var a = <div fooaria-foobar="true"></div>;
var a = <div fooaria-hidden="true"></div>;
var a = <input type="text" aria-errormessage="foobar" />;
