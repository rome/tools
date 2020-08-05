import React from "./react";
// function fail() {
// 	return <svg>
// 		<desc>
// 			Example rect01 - rectangle with sharp corners
// 		</desc>
// 		<rect />
// 		<rect />
// 		<g>
// 			<circle />
// 			<circle />
// 			<g>
// 				<circle />
// 				<circle />
// 			</g>
// 		</g>
// 	</svg>;
// }
// function pass() {
// 	return <svg>
// 		<title>
// 			Pass
// 		</title>
// 		<desc>
// 			Example rect01 - rectangle with sharp corners
// 		</desc>
// 		<rect />
// 		<rect />
// 		<g>
// 			<circle />
// 			<circle />
// 			<g>
// 				<circle />
// 				<circle />
// 			</g>
// 		</g>
// 	</svg>;
// }
function passNested() {
	return <svg>
		<desc>
			Example rect01 - rectangle with sharp corners
		</desc>
		<rect />
		<rect />
		<g>
			<circle />
			<circle />
			<g>
				<title>
					Pass
				</title>
				<circle />
				<circle />
			</g>
		</g>
	</svg>;
}
export default passNested;
