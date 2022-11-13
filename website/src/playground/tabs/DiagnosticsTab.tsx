interface Props {
	errors: string;
    lintingEnabled: boolean;
}

export default function DiagnosticsTab({ errors, lintingEnabled }: Props) {
	if (!lintingEnabled) {
	return <div className="empty-panel">Linter is currently disabled</div>;
	}
	if (errors === "") {
		return <div className="empty-panel">No diagnostics present</div>;
	}

	return (
		<pre className="language-shellsession">
			<code
				// rome-ignore lint(security/noDangerouslySetInnerHtml): the HTML is sanitized by our diagnostic printer
				dangerouslySetInnerHTML={{ __html: errors }}
			/>
		</pre>
	);
}
