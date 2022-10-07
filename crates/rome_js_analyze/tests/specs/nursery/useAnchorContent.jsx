<>
    {/* invalid */}
    <a />
		<a><TextWrapper aria-hidden /></a>
		<a><TextWrapper aria-hidden={true} /></a>
    <a><span aria-hidden="true">anchor content</span></a>
		<a></a>
		<a>{null}</a>
		<a>{undefined}</a>
    {/*  valid  */}
    <a>Anchor Content!</a>
    <a><TextWrapper /></a>
    <a dangerouslySetInnerHTML={{ __html: "foo" }} />
    <a><TextWrapper aria-hidden={true} /> visible content</a>
    <a><TextWrapper aria-hidden={false} /></a>
    <a><div aria-hidden="true"></div>visible content</a>
    <a><span aria-hidden="false">visible content</span></a>
    <a>{anchorContent}</a>
</>
