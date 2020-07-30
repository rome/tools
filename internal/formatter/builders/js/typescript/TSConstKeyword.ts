import {TSConstKeyword} from "@internal/ast";
import {Token} from "@internal/formatter";

export default function TSConstKeyword(): Token {
	return "const";
}
