// invalid
const a3 = <div>// comment</div>;
const a4 = <div>/* comment */</div>;
const a5 = <div>/** comment */</div>;
// valid
const a = <div>{/* comment */}</div>;
const a1 = <div>{/** comment */}</div>;
const a2 = <div className={"cls" /* comment */}></div>;