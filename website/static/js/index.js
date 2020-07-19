// @ts-check
/**
 * @type {{
 *   mobileHandleTOC: HTMLElement;
 *   mobileHandleNav: HTMLElement;
 *   toc: HTMLElement;
 *   tocLinks: NodeListOf<HTMLElement>;
 *   sidebarNav: HTMLElement;
 *   sidebarTOC: HTMLElement;
 *   headings: NodeListOf<HTMLElement>;
 *   headerMobile: HTMLElement;
 *   colorSchemeSwitcher: HTMLElement;
 *   teamList: HTMLElement;
 * }}
 */
const elements = {
	mobileHandleTOC: document.querySelector(".mobile-handle-toc"),
	mobileHandleNav: document.querySelector(".mobile-handle-nav"),
	toc: document.querySelector(".toc-container"),
	tocLinks: document.querySelectorAll(".toc-container a"),
	sidebarNav: document.querySelector(".sidebar.nav"),
	sidebarTOC: document.querySelector(".sidebar.toc"),
	headings: document.querySelectorAll(".content h1, .content h2"),
	headerMobile: document.querySelector(".header-mobile"),
	colorSchemeSwitcher: document.querySelector(".color-scheme-switch"),
	teamList: document.querySelector(".team-list"),
};

/**
 * @returns {boolean}
 */
function isMobile() {
	return (
		elements.sidebarNav.classList.contains("visible") ||
		/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
			navigator.userAgent,
		)
	);
}

const toc = {
	/**
	 * @returns {number}
	 */
	getMobileNavbarHeight() {
		if (isMobile()) {
			return parseFloat(window.getComputedStyle(elements.headerMobile).height);
		}

		return 0;
	},

	highlight() {
		const scrollY = window.scrollY;

		for (let i = 0; i < elements.headings.length; i++) {
			const element = elements.headings[i];

			const id = `#${element.getAttribute("id")}`;
			const y = element.offsetTop;
			const marginTop = parseFloat(window.getComputedStyle(element).marginTop);
			const height = parseFloat(window.getComputedStyle(element).marginTop);
			const link = document.querySelectorAll(`.toc-container a[href='${id}']`)[0];

			const nextElement = elements.headings[i + 1];

			let start = y - marginTop;
			let end = y + height + marginTop;

			if (nextElement) {
				const nextMarginTop = parseFloat(
					window.getComputedStyle(nextElement).marginTop,
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

	/**
	 * @param {MouseEvent} event
	 */
	handleClick(event) {
		const target = event.target;
		event.preventDefault();
		if (!(target instanceof HTMLElement)) {
			return;
		}

		const hash = target.getAttribute("href");

		if (target.hasAttribute("href")) {
			window.location.hash = hash;

			scrollToHeading(hash);

			if (isMobile()) {
				toggleMobileNav(event);
			}
		}
	},
};

/**
 * @param {string} hash 
 */
function scrollToHeading(hash) {
	const heading = document.getElementById(hash.replace(/^(#)/, ""));
	if (!heading) {
		return;
	}

	heading.setAttribute("tabindex", "-1");
	heading.focus();

	const marginTop = parseFloat(window.getComputedStyle(heading).marginTop);
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
}

/**
 * @type {undefined | "toc" | "nav"}
 */
let mobileSidebarActive;

function toggleMobileNav(event) {
	if (mobileSidebarActive === "nav") {
		mobileSidebarActive = undefined;
	} else {
		if (mobileSidebarActive === "toc") {
			toggleMobileTOC(event);
		}
		mobileSidebarActive = "nav";
	}

	event.preventDefault();
	elements.sidebarNav.classList.toggle("visible");
	document.body.classList.toggle("no-scroll");
}

function toggleMobileTOC(event) {
	if (mobileSidebarActive === "toc") {
		mobileSidebarActive = undefined;
	} else {
		if (mobileSidebarActive === "nav") {
			toggleMobileTOC(event);
		}
		mobileSidebarActive = "toc";
	}

	event.preventDefault();
	elements.sidebarTOC.classList.toggle("visible");
	document.body.classList.toggle("no-scroll");
	toc.highlight();
}

// Remove permalinkSymbol "#" from table of contents
for (const link of elements.tocLinks) {
	link.innerText = link.innerText.replace(/(\s#)$/, "");
}

window.onload = function() {
	if (window.location.hash !== "") {
		scrollToHeading(window.location.hash);
	}
	toc.highlight();
};

document.addEventListener(
	"click",
	function(event) {
		if (!(event.target instanceof HTMLElement)) {
			return;
		}

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

if (elements.toc) {
	elements.toc.addEventListener("click", toc.handleClick, false);
}

elements.mobileHandleNav.addEventListener("click", toggleMobileNav, false);
elements.mobileHandleTOC.addEventListener("click", toggleMobileTOC, false);
window.addEventListener("scroll", handleScroll, false);

//# Color scheme switcher

function toggleColorSchemeSwitch() {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme === undefined) {
		const prefersDarkMode = matchMedia('(prefers-color-scheme: dark)').matches;
		currentScheme = prefersDarkMode ? 'dark' : 'light';
	}

	const newScheme = currentScheme === "dark" ? "light" : "dark";
	window.localStorage.setItem("data-theme", newScheme);
	document.documentElement.setAttribute("data-theme", newScheme);
}

elements.colorSchemeSwitcher.addEventListener(
	"click",
	toggleColorSchemeSwitch,
	false,
);

//# Team list shuffle

/**
 * @template T
 * @param {Array<T>} array
 * @returns {Array<T>}
 */
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

const teamArr = document.querySelectorAll(".team-list li");
if (teamArr.length > 0) {
	for (const li of randomShuffle(Array.from(teamArr))) {
		elements.teamList.appendChild(li);
	}
}

//# Homepage example expander

const homepageExample = document.querySelector(".homepage-example");
if (homepageExample != null) {
	homepageExample.addEventListener(
		"click",
		() => {
			homepageExample.classList.remove("collapsed");
		},
	);
}
