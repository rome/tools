import {createVisitor, signals, Path} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import { isFor } from "@internal/js-ast-utils";
import { writeFileSync } from "fs";

function isInsideTheLoop(path:Path):boolean{
	const arr:Path[] = [];
	const parentPath = path.findAncestry((p)=> {
		if(isFor(p.node)||p.node.type==="JSWhileStatement"){
			return true;
		}
		else{
			arr.push(p);
			return false;
		}
	});
	if(parentPath===undefined){return false;}
	let k:string = "JSContinueStatement"; 
	let flag = 1;
	for(let i=0;i<arr.length;i++){
		const {node} = arr[i];
		// console.log('node', node.type, arr[i].node.loc);
		writeFileSync('./test.txt',arr[i]);
		console.log(node.loc);
		if(node.type==="JSBlockStatement"){
			if(node.body[node.body.length-1].type!==k){
				flag=0;
				k=node.type;
				break;
			}
		}
	}
	if(flag===0){
		return false;
	}
	else{
		return true;
	}
	// const p=path.ancestryPaths.map(({node})=> node.type);
	// const y = p.join("");
	// throw(Error(y));
	// path.ancestryPaths.map(({node})=>{
	// 	if(node.type === "JSBlockStatement"){
	// 	}
	// });
	// return parentPath!==undefined;
}


export default createVisitor({
	name: "js/noUnNecessaryContinue",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSContinueStatement" &&
			isInsideTheLoop(path)
			) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_UN_NECESSARY_CONTINUE,
			);
		}

		return signals.retain;
	},
});
