const VERSION = "0.0.0"

import { evaluate } from "./interpreter.ts"
import { readLines } from "https://deno.land/std@0.66.0/io/bufio.ts"


function print(str: string){
	Deno.stdout.writeSync(new TextEncoder().encode(str))
}


async function repl(){
	console.log(`Welcome to the Nectar REPL! You're using version ${VERSION}.`)
	console.log("Type statements like this: @Nectar is a #language.\n")

	print("nectar $ ")
	for await (const line of readLines(Deno.stdin)) {
		evaluate(line)
		print("\nnectar $ ")
	}
}


if(Deno.args.length === 0)
	repl()

else if(Deno.args.length === 1)
	evaluate(new TextDecoder('utf-8').decode(Deno.readFileSync(Deno.args[0])))