import { matchesDark, getCurrentTheme, setCurrentTheme } from "./util";
import "./mobile";
import "./toc";
import "./package-manager-commands";

//# Team list shuffle

/**
 * @template T
 * @param {Array<T>} array
 * @returns {Array<T>}
 */
function randomShuffle<T>(array: T[]): T[] {
	let count = array.length;
	let index;
	while (count) {
		index = Math.floor(Math.random() * count--);
		const temp = array[count]!;
		array[count] = array[index]!;
		array[index] = temp;
	}
	return array;
}

const creditsPeopleLists = document.querySelectorAll(".credits-people-list");
for (const list of creditsPeopleLists) {
	const items = list.querySelectorAll("li");
	for (const li of randomShuffle(Array.from(items))) {
		list.appendChild(li);
	}
}

//# Code expanders

const collapsed = document.querySelectorAll("pre.collapsed");
for (const elem of collapsed) {
	elem.addEventListener("click", () => {
		elem.classList.remove("collapsed");
	});
}

//# Color scheme switcher

function toggleColorSchemeSwitch(evt: Event) {
	const currentScheme = getCurrentTheme();
	const newScheme = currentScheme === "dark" ? "light" : "dark";

	if (evt.currentTarget instanceof Element) {
		evt.currentTarget.setAttribute(
			"aria-checked",
			String(newScheme === "dark"),
		);
	}

	document.documentElement.classList.add("transition");
	window.localStorage.setItem("data-theme", newScheme);
	setCurrentTheme(newScheme);
	onColorSchemeChange();
}

function onColorSchemeChange() {
	window.dispatchEvent(new Event("colorschemechange"));
}

const colorSchemeSwitcher = document.querySelector(".color-scheme-switch");
if (colorSchemeSwitcher != null) {
	colorSchemeSwitcher.addEventListener("click", toggleColorSchemeSwitch, false);
}

if (matchesDark !== undefined) {
	matchesDark.addEventListener("change", () => {
		onColorSchemeChange();
	});
}

//# Header scrolls to top
const tocSidebar: HTMLElement = document.querySelector(".toc-sidebar")!;
let topAnchors = Array.from(document.querySelectorAll("[href='#top']"));
if (location.pathname === "/") {
	topAnchors = [...topAnchors, ...document.querySelectorAll(".logo")];
}
for (const elem of topAnchors) {
	elem.addEventListener("click", (e) => {
		if (window.scrollY > 0) {
			e.preventDefault();

			if (tocSidebar != null) {
				tocSidebar.scrollTop = 0;
			}

			window.scrollTo(0, 0);

			// Remove the hash
			history.pushState(
				"",
				document.title,
				window.location.pathname + window.location.search,
			);
		}
	});
}
