import { parse_to_json } from "../pkg/nectar_lib.js"

export function evaluate(contents: string){
	try {
		const ast = JSON.parse(parse_to_json(contents))
		console.log("->", ast)
	}catch(e){
		console.error(e)
	}
}