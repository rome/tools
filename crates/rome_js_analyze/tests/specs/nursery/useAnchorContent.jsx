<>
    {/* invalid */}
    <a />
    <a><span aria-hidden="true">content</span></a>
    <a><span aria-hidden={true}>content</span></a>
		<a></a>
		<a>{null}</a>
		<a>{undefined}</a>
		<a aria-hidden>content</a>
    {/*  valid  */}
    <a>content</a>
    <a><TextWrapper /></a>
		<a><TextWrapper aria-hidden /></a>
		<a><TextWrapper aria-hidden={true} /></a>
    <a><TextWrapper aria-hidden={false} /></a>
    <a dangerouslySetInnerHTML={{ __html: "foo" }} />
    <a><div aria-hidden="true"></div>content</a>
    <a><span aria-hidden="false">content</span></a>
    <a>{content}</a>
</>
