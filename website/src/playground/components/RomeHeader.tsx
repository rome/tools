import romeIcon from "../../svg/logomark.svg";
import romeDarkIcon from "../../svg/logomark_white_yellow.svg";
import { useTheme } from "../utils";

export default function RomeHeader() {
	const theme = useTheme();

	return (
		<>
			<img src={theme === "dark" ? romeDarkIcon : romeIcon} />
			<span>Rome</span>
		</>
	);
}
