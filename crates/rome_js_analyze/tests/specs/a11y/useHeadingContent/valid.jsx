<>
	<div />
	<h1>heading</h1>
	<h2>heading</h2>
	<h3>heading</h3>
	<h4>heading</h4>
	<h5>heading</h5>
	<h6>heading</h6>
	<h1>
		<Bar />
	</h1>
	<h1>
		<Bar {...props} />
	</h1>
	<h1>
		<Bar aria-hidden />
	</h1>
	<h1>
		<Typography.Heading />
	</h1>
	<h1>{foo}</h1>
	<h1>{foo.bar}</h1>
	<h1>
		<div aria-hidden="true"></div>visible content
	</h1>
	<h1>
		<div aria-hidden="true" {...props}></div>
	</h1>
	<h1>
		<div aria-hidden="true" {...props} />
	</h1>
	<h1 dangerouslySetInnerHTML={{ __html: "heading" }} />
	<h1>
		<div aria-hidden />
		visible content
	</h1>
	<h1 dangerouslySetInnerHTML={{ __html: "heading" }}></h1>
	<h1 children={children} />
	<h1 children={"heading"} />
	<h1 aria-hidden="true" {...props} />
	<h1 aria-hidden="true" {...props} />
	<h1 {...props} />
	<h1>
		<div aria-hidden="true"></div>visible content
	</h1>
	<h1>
		<>heading</>
	</h1>
</>;
