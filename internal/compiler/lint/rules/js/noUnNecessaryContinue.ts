import {createVisitor, signals, Path} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import { isFor } from "@internal/js-ast-utils";
// import { writeFileSync } from "fs";

function isInsideTheLoop(path:Path):boolean{
	const arr: Path[] = [];
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
	// for(let i=0;i<arr.length;i++){
	// 	const a = arr[i];
	// 	console.log('ppp',a.node);
	// 	if(a.node.type==="JSWhileStatement"){
	// 		console.log(a.node.body);
	// 	}
	// }
	// if(arr[0].node.type==="JSB")
	const length = arr.length;
	// for(let i=0;i<arr.length;i++){
	// 		const a = arr[i];
	// 		console.log('ppp',a.node.type);
	// }
	// console.log('length',length);
	// if(length>2&&arr[length-1].node===arr){}
	if(length===0){return false};
	const blockStatementNode = arr[length-1].node;
	const continueBlockNode = arr[0].node;
	if(length===1&&blockStatementNode.type==="JSBlockStatement"&&blockStatementNode.body[blockStatementNode.body.length-1]===path.node
	&&(continueBlockNode.type==="JSBlockStatement"&&continueBlockNode.body[continueBlockNode.body.length-1]===path.node)){
		// console.log('yayyy');
		return true;
	} 
	else if(length>1&&(blockStatementNode.type==="JSBlockStatement"&&blockStatementNode.body[blockStatementNode.body.length-1]===arr[length-2].node)&&
	(continueBlockNode.type==="JSBlockStatement"&&continueBlockNode.body[continueBlockNode.body.length-1]===path.node)){
		// console.log('yayyy1');
		return true;
	}
	else{
		return false;
	}

	// throw(Error('nope'));
	
}


export default createVisitor({
	name: "js/noUnNecessaryContinue",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSContinueStatement" 
			&& isInsideTheLoop(path)
			) 
			{
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_UN_NECESSARY_CONTINUE,
			);
		}

		return signals.retain;
	},
});
