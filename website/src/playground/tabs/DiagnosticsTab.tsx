interface Props {
  errors: string;
}

export default function DiagnosticsTab({errors}: Props) {
  if (errors === "") {
    return <span>No diagnostics present</span>;
  }

  return <pre className="language-shellsession">
    <code
      // rome-ignore lint(security/noDangerouslySetInnerHtml): the HTML is sanitized by our diagnostic printer
      dangerouslySetInnerHTML={{ __html: errors }}
    />
  </pre>;
}
