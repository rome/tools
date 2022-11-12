interface Props {
	errors: string;
}

export default function LinterTab({ errors }: Props) {
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
