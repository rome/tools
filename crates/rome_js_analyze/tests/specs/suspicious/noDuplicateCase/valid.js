var a = 1;
switch (a) {
	case 1:
		break;
	case 2:
		break;
	default:
		break;
}

var a = 1;
switch (a) {
	case 1:
		break;
	case "1":
		break;
	default:
		break;
}
var a = 1;
switch (a) {
	case 1:
		break;
	case true:
		break;
	default:
		break;
}
var a = 1;
switch (a) {
	default:
		break;
}
var a = 1,
	p = { p: { p1: 1, p2: 1 } };
switch (a) {
	case p.p.p1:
		break;
	case p.p.p2:
		break;
	default:
		break;
}
var a = 1,
	f = function (b) {
		return b ? { p1: 1 } : { p1: 2 };
	};
switch (a) {
	case f(true).p1:
		break;
	case f(true, false).p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(a + 1).p1:
		break;
	case f(a + 2).p1:
		break;
	default:
		break;
}
var a = 1,
	f = function (s) {
		return { p1: s };
	};
switch (a) {
	case f(a == 1 ? 2 : 3).p1:
		break;
	case f(a === 1 ? 2 : 3).p1:
		break;
	default:
		break;
}
var a = 1,
	f1 = function () {
		return { p1: 1 };
	},
	f2 = function () {
		return { p1: 2 };
	};
switch (a) {
	case f1().p1:
		break;
	case f2().p1:
		break;
	default:
		break;
}
var a = [1, 2];
switch (a.toString()) {
	case [1, 2].toString():
		break;
	case [1].toString():
		break;
	default:
		break;
}
switch (a) {
	case a:
		break;
}
switch (a) {
	case a:
		break;
}
switch (a) {
	case toString:
		break;
}
switch (a) {
  case "'":
	return '&apos;';
  case '"':
	return '&quot;';
}