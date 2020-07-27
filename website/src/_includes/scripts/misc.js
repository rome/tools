//# Responsive width

const mobileMatchMedia = matchMedia("(max-width: 768px)");
let isMobile = mobileMatchMedia.matches;

mobileMatchMedia.addEventListener("change", (e) => {
	isMobile = e.matches;

	// Close the mobile sidebar when switching from mobile to desktop
	if (isMobileSidebarVisible && !isMobile && isMobileSidebarVisible) {
		toggleMobileSidebar();
	}
});

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

const teamList = document.querySelector(".team-list");
const teamArr = document.querySelectorAll(".team-list li");
if (teamArr.length > 0) {
	for (const li of randomShuffle(Array.from(teamArr))) {
		teamList.appendChild(li);
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

//# Hide WIP banner after scrolling

const wipBanner = document.querySelector(".wip-banner");
if (wipBanner) {
	let hasScrolled = false;

	window.addEventListener(
		"scroll",
		() => {
			if (hasScrolled) {
				return;
			}

			if (window.scrollY > 0) {
				hasScrolled = true;
				setTimeout(
					() => {
						wipBanner.classList.add("hidden");
					},
					2_000,
				);
			}
		},
		{
			passive: true,
		},
	);
}

//# Color scheme switcher

function toggleColorSchemeSwitch() {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme === undefined) {
		const prefersDarkMode = matchMedia("(prefers-color-scheme: dark)").matches;
		currentScheme = prefersDarkMode ? "dark" : "light";
	}

	const newScheme = currentScheme === "dark" ? "light" : "dark";
	window.localStorage.setItem("data-theme", newScheme);
	document.documentElement.setAttribute("data-theme", newScheme);
}

const colorSchemeSwitcher = document.querySelector(".color-scheme-switch");
colorSchemeSwitcher.addEventListener("click", toggleColorSchemeSwitch, false);

//# Mobile navigation

const mobileSidebarHandle = document.querySelector(".mobile-handle");
let isMobileSidebarVisible = false;
function toggleMobileSidebar() {
	isMobileSidebarVisible = !isMobileSidebarVisible;
	mobileSidebarHandle.classList.toggle("active");
	sidebar.classList.toggle("visible");
	document.body.classList.toggle("no-scroll");
}
mobileSidebarHandle.addEventListener("click", (event) => {
	event.preventDefault();
	toggleMobileSidebar();
}, false);

//# Docsearch
// Only initialize on focus

const docsearchInput = document.querySelector("#docsearch");
docsearchInput.addEventListener("focus", () => {
	// Stylesheet
	const link = document.createElement("link")
	link.href = "/docsearch.css";
	link.rel = "stylesheet";
	document.body.appendChild(link);

	// Script
	const script = document.createElement("script");
	script.src = "/docsearch.js";
	script.async = true;
	script.defer = true;
	script.addEventListener(
		"load",
		() => {
			return window.docsearch({
				apiKey: "66db1ad366d458c6acded7cbc23dba7e",
				indexName: "romefrontend",
				inputSelector: "#docsearch",
				debug: false, // Set debug to true if you want to inspect the dropdown
			});
		},
	);
	document.body.appendChild(script);
}, {once: true});
