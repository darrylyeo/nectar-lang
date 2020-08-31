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

	print("nectar $ ")

	for await (const line of readLines(Deno.stdin)) {
		if(line === ""){}
		else if(line === "reset")
			interpreter = new NectarInterpreter()
		else if(line === "exit" || line === "quit")
			return
		else
			interpreter.evaluate(line)

		// Print a table of the current state
		// @ts-ignore
		console.log("\n" + interpreter.scope.debug().map(([k, [v, ...vs]]) =>
			// [(k + ': ').padStart(18) + v, ...vs].join("\n" + " ".repeat(18))
			[(k + ': ').padStart(18) + v, ...vs].join("\n").replace(/\n/g, "\n" + "   ".repeat(18))
		).join("\n\n"))

		print("\nnectar $ ")
	}
}