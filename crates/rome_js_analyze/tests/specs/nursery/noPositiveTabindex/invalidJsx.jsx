let test;

test = <div tabIndex={1} />;

test = <div tabIndex={"1"} />;

test = <div tabIndex={'5'} />;

// not working - inner_string_text can't capture the number
test = <div tabIndex="1" />;

test = <div tabIndex={1}>foo</div>;

test = <div tabIndex={"1"}>foo</div>;

test = <div tabIndex={'5'}>foo</div>;

test = <div tabIndex="1">foo</div>;
