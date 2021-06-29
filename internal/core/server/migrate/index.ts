import lint from "./migrations/renameIgnorePaths";
import {SemverVersion} from "@internal/codec-semver";
import {Migration} from "@internal/core/server/migrate/Migration";

export const migrations: Map<SemverVersion, Migration> = new Map();

migrations.set(lint.addedVersion, lint);
