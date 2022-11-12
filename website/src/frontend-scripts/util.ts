export type ThemeName = "dark" | "light";

export const matchesDark = window.matchMedia("(prefers-color-scheme: dark)");

export function getCurrentTheme(): ThemeName {
	let currentScheme = window.localStorage.getItem("data-theme");
	if (currentScheme == null) {
		currentScheme = matchesDark.matches ? "dark" : "light";
	}
	return currentScheme === "dark" ? "dark" : "light";
}

export function setCurrentTheme(theme: ThemeName) {
	document.documentElement.setAttribute("data-theme", theme);
}
