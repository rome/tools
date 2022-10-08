// invalid

<>
	<img src="foo" />
	<img {...props} />
	<img {...props} alt={undefined} />
	<img src="foo" role="presentation" />
	<img src="foo" role="none" />
	<img alt />
</>

// valid

<>
	<img {...props} alt />
	<img src="foo" alt="Foo eating a sandwich." />
	<img src="foo" alt={"Foo eating a sandwich."} />
	<img src="foo" alt={altText} />
    <img src="foo" alt={<><span class="token punctuation">${</span><span class="token variable">person</span><span class="token punctuation">}</span><span class="token string"> smiling</span><span class="token string"></>} />
    <img src="foo" alt="" />

</>;
