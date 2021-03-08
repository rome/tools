import {VERSION} from "@internal/core";
import {PUBLIC_PACKAGES, execDev} from "./_utils";

export async function main(args: string[]) {
	await execDev(["bundle", PUBLIC_PACKAGES.append("rome").join(), ...args, "--set-version", VERSION]);
}
