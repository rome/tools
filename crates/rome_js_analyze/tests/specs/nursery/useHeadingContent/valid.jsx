<>
	<h1>heading</h1>
	<h1>
		<div aria-hidden="true"></div>visible content
	</h1>
	<h1 dangerouslySetInnerHTML={{ __html: "heading" }} />
	<h1>
		<div aria-hidden />
		visible content
	</h1>
	<h1 dangerouslySetInnerHTML={{ __html: "heading" }}></h1>
</>;
