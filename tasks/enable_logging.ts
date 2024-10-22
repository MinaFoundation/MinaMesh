
import * as path from "@std/path"

const destDir = path.join(Deno.cwd(), "sql_scripts")

await Deno.copyFile(
  path.join(path.dirname(path.fromFileUrl(import.meta.url)), "enable_logging.sh"),
  path.join(destDir, "enable_logging.sh"),
)

console.log("Copied enable_logging.sh to", destDir)
console.log("In order to apply the changes, restart the Postgres server. (deno task pg:init)")