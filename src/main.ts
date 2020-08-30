const VERSION = "0.0.0"

import { NectarInterpreter } from "./interpreter.ts"
import { readLines } from "https://deno.land/std@0.66.0/io/bufio.ts"


function print(str: string){
	Deno.stdout.writeSync(new TextEncoder().encode(str))
}


async function repl(){
	console.log(`Welcome to the Nectar REPL! You're using version ${VERSION}.`)
	console.log("Type statements like this: @Nectar is a #language.\n")

	let interpreter = new NectarInterpreter()

	print("nectar $ ")

	for await (const line of readLines(Deno.stdin)) {
		if(line === "reset")
			interpreter = new NectarInterpreter()
		else if(line === "exit" || line === "quit")
			return
		else
			interpreter.evaluate(line)

		print("\nnectar $ ")
	}
}


if(Deno.args.length === 0)
	repl()

else if(Deno.args.length === 1)
	new NectarInterpreter().evaluate(
		new TextDecoder('utf-8').decode(
			Deno.readFileSync(Deno.args[0])
		)
	)