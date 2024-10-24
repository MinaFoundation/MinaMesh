import pg from "pg";
import "@std/dotenv/load";
import { assertExists } from "@std/assert";
import * as path from "@std/path";

const connectionString = Deno.env.get("DATABASE_URL");
assertExists(connectionString);

const args = Deno.args;

if (args.length !== 1 || (args[0] !== "--apply" && args[0] !== "--drop")) {
  console.error("Allowed parameters: --apply | --drop");
  Deno.exit(1);
}

const scriptPath = args[0] === "--apply"
  ? path.join(path.dirname(path.fromFileUrl(import.meta.url)), "../sql/apply_search_tx_optimizations.sql")
  : path.join(path.dirname(path.fromFileUrl(import.meta.url)), "../sql/drop_search_tx_optimizations.sql");

const client = new pg.Client({ connectionString });
await client.connect();

try {
  const sqlScript = await Deno.readTextFile(scriptPath);
  console.log(`Executing ${args[0].slice(2)} script...`);
  await client.query(sqlScript);
  console.log(`Successfully executed ${args[0].slice(2)} script.`);
} catch (error) {
  console.error("Error executing SQL script:", error);
} finally {
  await client.end();
}
