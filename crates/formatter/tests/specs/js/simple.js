function foo() {
	let [ref, setRef] = useState();

	useEffect(() => {
		setRef()
	});

	return ref;
}
