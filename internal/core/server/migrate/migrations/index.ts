/* GENERATED:START(hash:6801705fbec141c1de139dfe26cd8c6f0dcf9394,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/migrations` to update. */
import renameIgnorePaths from "./renameIgnorePaths";
import {SemverVersion} from "@internal/codec-semver";
import {Migration} from "../Migration";

export const migrations: Map<SemverVersion, Migration> = new Map();
migrations.set(renameIgnorePaths.addedVersion, renameIgnorePaths);
/* GENERATED:END(id:main) */
