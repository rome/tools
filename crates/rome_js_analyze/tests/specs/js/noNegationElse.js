// valid
if (!true) {consequent;};
true ? consequent : alternate;
// invalid
if (!true) {
  consequent;
} else {
  alternate;
}
!condition ? consequent : alternate;

let a = !test ? c : d;
if (!true) {
	consequent;
} else something();
//valid https://github.com/rome/tools/issues/2999 
if (!/^NCT/.test(input)) {
	messages.push("NCT Number must start with NCT");
} else if (!/^NCT\d{8}$/.test(input)) {
	messages.push("NCT Number must have exactly 8 digits after NCT");
}