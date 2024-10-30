import pg from "pg";
import "@std/dotenv/load";
import { assertExists } from "@std/assert";
import { parseArgs } from "@std/cli";
import * as path from "@std/path";

const connectionString = Deno.env.get("DATABASE_URL");
assertExists(connectionString);

const args = parseArgs(Deno.args, {
  flags: {
    apply: { type: "boolean" },
    drop: { type: "boolean" },
  },
});

if (!args.apply && !args.drop) {
  console.error("Allowed parameters: --apply | --drop");
  Deno.exit(1);
}

const scriptPath = args.apply
  ? path.join(path.dirname(path.fromFileUrl(import.meta.url)), "../sql/migrations/apply_search_tx_optimizations.sql")
  : path.join(path.dirname(path.fromFileUrl(import.meta.url)), "../sql/migrations/drop_search_tx_optimizations.sql");

const client = new pg.Client({ connectionString });
await client.connect();

try {
  const sqlScript = await Deno.readTextFile(scriptPath);
  console.log(`Executing ${args.apply ? "apply" : "drop"} script...`);
  await client.query(sqlScript);
  console.log(`Successfully executed ${args.apply ? "apply" : "drop"} script.`);
} catch (error) {
  console.error("Error executing SQL script:", error);
} finally {
  await client.end();
}
