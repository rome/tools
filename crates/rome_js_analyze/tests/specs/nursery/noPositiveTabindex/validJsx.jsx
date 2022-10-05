let test;

test = <div />;

test = <div tabIndex={func} />;

test = <div tabIndex={func()} />;

test = <div tabIndex={null} />;

test = <div tabIndex={undefined} />;

test = <div tabIndex={"abc"} />;

test = <div tabIndex={0} />;

test = <div tabIndex={-1} />;

test = <div tabIndex={"0"} />;

test = <div tabIndex={'-5'} />;

// string literals are skipped
test = <div tabIndex={`-1`} />;
test = <div tabIndex={`1`} />;

test = <div tabIndex="-1" />;

test = <div tabIndex="abc" />;

test = <div>foo</div>;

test = <div tabIndex={-1}>foo</div>;

test = <div tabIndex={"-1"}>foo</div>;

test = <div tabIndex={'-5'}>foo</div>;

test = <div tabIndex="-1">foo</div>;

// string literals are skipped
test = <div tabIndex={`-1`}>foo</div>;
test = <div tabIndex={`1`}>foo</div>;

test = <div tabIndex={`abc`}>foo</div>;

test = <div tabIndex={null}>foo</div>;

test = <div tabIndex={undefined}>foo</div>;

test = <div tabIndex={func()}>foo</div>;

test = <div tabIndex={func}>foo</div>;

