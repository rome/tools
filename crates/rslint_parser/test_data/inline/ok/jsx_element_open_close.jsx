function f() {
    let [errors, setError] = useState([]);
    let color = get_text_color();
    let style = get_style();
    return <>
        <button id="btn" onClick={x => console.log(x)} {...style}>
            <ColoredText el=<span>Click Here</span> color={color} underline={style.underlined}/>
        </button>
        {errors.length == 0 && errors.map(x => <div>{x}</div>)}
    </>;
}
