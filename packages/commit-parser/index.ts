import {ParserOptions, TokenValues} from "@romefrontend/parser-core";
import {Tokens} from "./types";
import {createCommitParser} from "./parse";
import {CommitRoot} from "@romefrontend/ast";

export function parseCommit(opts: ParserOptions): CommitRoot {
	return createCommitParser(opts).parse();
}

export function tokenizeCommit(opts: ParserOptions): Array<TokenValues<Tokens>> {
	return createCommitParser(opts).tokenizeAll();
}
