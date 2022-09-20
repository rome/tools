<div>{...a}</div>;

<div>{...a /* comment */}</div>;

<div>{/* comment */...a}</div>;

// rome-ignore format: Instability issue
<div>{...a //comment
}</div>;

<div>{...a
  //comment
}</div>;

<div>{
  //comment
  ...a
}</div>;

// rome-ignore format: Instability issue
<div>{//comment
  ...a // comment
}</div>;
