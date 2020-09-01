const VERSION = "0.0.0"

import { NectarInterpreter } from "./interpreter.ts"
import { readLines } from "https://deno.land/std@0.66.0/io/bufio.ts"


function print(str: string){
	Deno.stdout.writeSync(new TextEncoder().encode(str))
}


export async function repl(){
	console.log(`Welcome to the Nectar REPL! You're using version ${VERSION}.`)
	console.log("Type statements like this: @Nectar is a #language.\n")

	let interpreter = new NectarInterpreter()

	print(`${interpreter.scope.nestedName} $ `)

	let match
	for await (const line of readLines(Deno.stdin)) {
		if(line === "")
			interpreter.debug()
		else if(line === "reset")
			interpreter = new NectarInterpreter()
		else if(line === "exit" || line === "quit")
			return
		else if(match = line.trim().match(/^([a-z][a-z0-9_-]*)\s*{$/i))
			interpreter.pushScope(match[1])
		else if(line.trim() === "}"){
			interpreter.popScope()
			console.log()
			interpreter.debug()
		}
		else try {
			const results = interpreter.evaluate(line)
			if(results.length)
				for(const result of results)
					console.log("->", result)
			else{
				console.log()
				interpreter.debug()
			}
		}
		catch(e){
			console.error(e)
		}

		print(`\n${interpreter.scope.nestedName} $ `)
	}
}