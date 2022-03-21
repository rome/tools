<Foo />;

<Foo           />;

<LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />;
<IndentStyleSelect indentWidth={indentWidth} setIndentWidth={setIndentWidth} indentStyle={indentStyle} setIndentStyle={setIndentStyle}
/>;
<SourceTypeSelect
	isTypeScript={isTypeScript}

	setIsTypeScript={setIsTypeScript}

	isJsx={isJsx}

	setIsJsx={setIsJsx}
/>;

<CodeEditor
	value={code}
	language="js"
	placeholder="Enter JS here"
	onChange={(evn) => {
		setCode(evn.target.value);
	}}
	style={{ fontSize: 12, height: "100vh", fontFamily:
			"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
	}}
/>
