//valid
<input disabled={false} />;
<input disabled={""} />;
<input disabled={0} />;
<input disabled={undefined} />;
<input disabled="false" />;

//invalid
<input disabled />;
<input accept/** some comment */ />;
<input /** some comment */ accept />;
