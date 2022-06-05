// valid
if (!true) {consequent;};
true ? consequent : alternate;
// invalid
if (true) {
  alternate;
} else {
  consequent;
}
!condition ? consequent : alternate;