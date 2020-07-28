if (!location.href.includes("veryfunny")) {
	const style = document.createElement("style");
	style.textContent = `
		html {
			background-image: url("/img/under-construction-1.gif");
		}

		body {
			font-family: "Comic Sans MS", cursive, sans-serif;
		}

		h1, h2, h3, h4, h5, h6 {
			font-family: Impact, Charcoal, sans-serif;
		}

		.content .header-anchor {
			display: inline-block;
			background-image: url("/img/under-construction-2.gif");
			width: 60px;
			height: 60px;
			vertical-align: middle;
			color: transparent;
		}

		svg {
			background-image: url("/img/under-construction-3.gif");
			background-size: contain;
		}

		svg path {
			fill: transparent !important;
		}

		.hero {
			background-image: url("/img/under-construction-4.gif");
			text-shadow: 1px 1px red;
		}
	`;
	document.head.appendChild(style);
	const {documentElement} = document;
}
