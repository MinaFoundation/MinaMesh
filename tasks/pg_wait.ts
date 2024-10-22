// TODO: would we rather use a TypeScript equivalent?

import pg from "pg"
import "@std/dotenv/load"
import { assertExists } from "@std/assert"

const connectionString = Deno.env.get("DATABASE_URL")
assertExists(connectionString)
const client = new pg.Client({ connectionString })

const MAX_RETRIES = 200
const RETRY_INTERVAL_MS = 10_000

let connected = false
let attempts = 0

while (!connected && attempts < MAX_RETRIES) {
  try {
    await client.connect()
    connected = true
  } catch (err: unknown) {
    if (err instanceof Error && err.message.startsWith("Client has already been connected.")) {
      connected = true
      break
    }
    attempts++
    console.log("Waiting for database to be ready.")
    console.error(err)
    await new Promise((resolve) => setTimeout(resolve, RETRY_INTERVAL_MS))
  }
}

if (connected) {
  await client.end()
  console.log("Database ready at", connectionString)
} else throw 0
