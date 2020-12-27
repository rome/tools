import {ISO} from "@internal/compiler/lint/utils/constants";

const countries = ISO.get("countries")
const languages = ISO.get("languages")


export function langSupported(lang: string): boolean {
	const [language, country] = lang.split("-");

	if (language && country && countries && languages) {
		return languages.has(language) && countries.has(country);
	}

	if (language && languages) {
		return languages.has(language);
	}

	return false;
}
