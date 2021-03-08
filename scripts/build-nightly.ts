import {ROOT} from "./_utils";
import {main as updateVersion} from "./update-version";
import {main as buildRelease} from "./build-release";
import {VERSION} from "@internal/core";

export async function main() {
  // Build a version number with a tag unique to the current day
  const [version] = VERSION.split("-");
  const date = new Date();
  const dateParts = [date.getFullYear(), date.getMonth(), date.getDate()];
  const dateTag = dateParts.map(num => {
    const str = String(num);
    if (str.length === 1) {
      return `0${str}`;
    } else {
      return str;
    }
  }).join(".");
  const newVersion = `${version}-nightly.${dateTag}`;
  await updateVersion([newVersion]);

  // Build a release to the dist folder in the root
  await buildRelease([ROOT.append("dist").join()]);
}
