// valid
<div />;
<div>child</div>;
<Component />;
<Component>child</Component>;
<Foo.bar />;
<Foo.bar>child</Foo.bar>;

// invalid
<div               ></div>;
<Component></Component>;
<Foo.bar></Foo.bar>;
<div

></div>;

<div ></div> /* comment */;
/* comment */ <div ></div>;
<Generic<true>></Generic>;