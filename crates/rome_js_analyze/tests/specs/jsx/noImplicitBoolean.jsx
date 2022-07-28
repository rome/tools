//valid
<input disabled={false} />;
<input disabled={""} />;
<input disabled={0} />;
<input disabled={undefined} />;
<input disabled="false" />;
// https://github.com/rome/tools/issues/2944
<div className={asdf asdf} />;

//invalid
<input disabled />;
<input accept/** some comment */ />;
<input /** some comment */ accept />;
