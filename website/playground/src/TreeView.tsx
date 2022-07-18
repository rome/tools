interface Props {
	tree: string;
}

export default function TreeView({ tree }: Props) {
	return <div className="overflow-scroll"><pre>{tree}</pre></div>;
}
