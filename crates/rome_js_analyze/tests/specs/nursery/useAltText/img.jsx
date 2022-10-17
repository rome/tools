// invalid

<>
	<img src="foo" />
	<img src="foo" role="presentation" />
	<img src="foo" role="none" />
	<img alt />
</>

// valid

<>
	<img {...props} alt />
	<img {...props} /> {/* Skipping*/}
	<img {...props} alt={undefined} /> {/* Skipping*/}
	<img src="foo" alt="Foo eating a sandwich." />
	<img src="foo" alt={"Foo eating a sandwich."} />
	<img src="foo" alt={altText} />
    <img src="foo" alt={<><span class="token punctuation">${</span><span class="token variable">person</span><span class="token punctuation">}</span><span class="token string"> smiling</span><span class="token string"></>} />
    <img src="foo" alt="" />

</>;
