const elements = {
	mobileHandle: document.getElementsByClassName("mobile-handle")[0],
	toc: document.getElementsByClassName("toc-container")[0],
	tocLinks: [...document.querySelectorAll(".toc-container a")],
	sidebar: document.getElementsByClassName("sidebar")[0],
	sidebarRight: document.querySelectorAll(".sidebar.right")[0],
	overlay: document.getElementsByClassName("overlay")[0],
	headings: [...document.querySelectorAll(".content h1, .content h2")],
	headerMobile: document.getElementsByClassName("header-mobile")[0],
	colorSchemeSwitch: document.getElementById("color-scheme-switch"),
	colorSchemeSwitchText: document.getElementById("color-scheme-switch-text"),
	teamList: document.getElementsByClassName("team-list")[0],
};

function isMobile() {
	return (
		elements.sidebar.classList.contains("visible") ||
		/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
			navigator.userAgent,
		)
	);
}

const toc = {
	currentH1Link: null,
	getMobileNavbarHeight() {
		if (isMobile()) {
			return parseFloat(
				window.getComputedStyle(elements.headerMobile).height,
				10,
			);
		}

		return 0;
	},
	highlight() {
		const scrollY = window.scrollY;

		for (let i = 0; i < elements.headings.length; i++) {
			const element = elements.headings[i];

			const id = `#${element.getAttribute("id")}`;
			const y = element.offsetTop;
			const marginTop = parseFloat(
				window.getComputedStyle(element).marginTop,
				10,
			);
			const height = parseFloat(window.getComputedStyle(element).marginTop, 10);
			const link = document.querySelectorAll(`.toc-container a[href='${id}']`)[0];

			const nextElement = elements.headings[i + 1];

			let start = y - marginTop;
			let end = y + height + marginTop;

			if (nextElement) {
				const nextMarginTop = parseFloat(
					window.getComputedStyle(nextElement).marginTop,
					10,
				);
				end = nextElement.offsetTop - nextMarginTop;
			}

			start -= toc.getMobileNavbarHeight();
			end -= toc.getMobileNavbarHeight();
			if (link) {
				if (scrollY > start && scrollY < end) {
					link.classList.add("active");
				} else {
					link.classList.remove("active");
				}
			}
		}
	},
	handleClick(event) {
		const target = event.target;
		event.preventDefault();

		const hash = target.getAttribute("href");

		if (target.hasAttribute("href")) {
			window.location.hash = hash;

			scrollToHeading(hash);

			if (isMobile()) {
				mobileToggleEvent(event);
			}
		}
	},
};

function scrollToHeading(hash) {
	const heading = document.getElementById(hash.replace(/^(#)/, ""));

	if (!heading) {
		return null;
	}

	heading.setAttribute("tabindex", "-1");
	heading.focus();

	const marginTop = parseFloat(window.getComputedStyle(heading).marginTop, 10);
	window.scrollTo(
		0,
		heading.offsetTop - toc.getMobileNavbarHeight() - (marginTop - 2),
	);
}

function handleScroll() {
	if (isMobile()) {
		return false;
	}

	toc.highlight();

	if (window.scrollY > 6) {
		elements.sidebar.style["border-top-width"] = "5.3333333333px";
	} else {
		elements.sidebar.style["border-top-width"] = "0px";
	}
}

function mobileToggleEvent(event) {
	const bodyClassList = document.body.classList;
	event.preventDefault();
	elements.sidebar.classList.toggle("visible");
	elements.sidebarRight.classList.toggle("visible");
	elements.overlay.classList.toggle("visible");
	bodyClassList.toggle("no-scroll");
	toc.highlight();
}

function modeSwitch() {
	const $doc = document.documentElement;
	let theme = $doc.getAttribute("data-theme");

	if (theme === "light") {
		elements.colorSchemeSwitchText.innerText = "Light Mode";
		theme = "dark";
	} else {
		elements.colorSchemeSwitchText.innerText = "Dark Mode";
		theme = "light";
	}

	$doc.setAttribute("data-theme", theme);
	window.localStorage.setItem("data-theme", theme);
}

function randomShuffle(array) {
	let count = array.length;
	let temp;
	let index;
	while (count) {
		index = Math.floor(Math.random() * count--);
		temp = array[count];
		array[count] = array[index];
		array[index] = temp;
	}

	return array;
}
const themeInStorage = window.localStorage.getItem("data-theme");

if (themeInStorage) {
	if (themeInStorage === "dark") {
		elements.colorSchemeSwitchText.innerText = "Light Mode";
	}
}

//remove permalinkSymbol "#" from table of contents
elements.tocLinks.forEach(function(link) {
	link.innerText = link.innerText.replace(/(\s#)$/, "");
});

window.onload = function() {
	if (window.location.hash !== "") {
		scrollToHeading(window.location.hash);
	}
	toc.highlight();
};

document.addEventListener(
	"click",
	function(event) {
		if (!event.target.matches(".header-anchor")) {
			return;
		}

		event.preventDefault();

		const hash = event.target.getAttribute("href");

		window.location.hash = hash;

		scrollToHeading(hash);
	},
	false,
);

elements.toc.addEventListener("click", toc.handleClick, false);
elements.mobileHandle.addEventListener("click", mobileToggleEvent, false);
elements.overlay.addEventListener("click", mobileToggleEvent, false);
elements.overlay.addEventListener("touchstart", mobileToggleEvent, false);
window.addEventListener("scroll", handleScroll, false);

elements.colorSchemeSwitch.addEventListener("click", modeSwitch, false);
if (elements.teamList) {
	const teamArr = Array.from(elements.teamList.getElementsByTagName("li"));
	for (const li of randomShuffle(teamArr)) {
		elements.teamList.appendChild(li);
	}
}

const homepageExample = document.querySelector(".homepage-example");
if (homepageExample != null) {
	homepageExample.addEventListener(
		"click",
		() => {
			homepageExample.classList.remove("collapsed");
		},
	);
}
