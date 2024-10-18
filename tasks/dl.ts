import { dedent } from "@qnighy/dedent"
import { assert, assertExists } from "@std/assert"
import { parseArgs } from "@std/cli"
import { format } from "@std/datetime"
import * as fs from "@std/fs"
import * as path from "@std/path"

const MINA_ARCHIVE_DUMP_URL = "https://storage.googleapis.com/mina-archive-dumps"

const { network, "dump-time": dumpTime } = parseArgs(Deno.args, {
  string: ["network", "dump-time"],
  default: {
    network: "devnet",
    "dump-time": "0000",
  },
})

assert(
  ({
    devnet: true,
    mainnet: true,
  } as Record<string, boolean>)[network],
)

const date = format(
  (() => { // 3d ago
    const d = new Date()
    d.setDate(d.getDate() - 3)
    return d
  })(),
  "yyyy-MM-dd",
)
const dumpUrl = `${MINA_ARCHIVE_DUMP_URL}/${network}-archive-dump-${date}_${dumpTime}.sql.tar.gz`
const destDir = path.join(Deno.cwd(), "sql_scripts")
const dest = path.join(destDir, "archive.tar")

console.log(`Downloading ${dumpUrl} to ${dest}`)

await fs.emptyDir(destDir)

const dumpTarStream = await fetch(dumpUrl).then((v) => v.body)
assertExists(dumpTarStream)

const file = await Deno.open(dest, {
  createNew: true,
  append: true,
})
await dumpTarStream.pipeThrough(new DecompressionStream("gzip")).pipeTo(file.writable)

// TODO: utilize `UntarStream` from `@std/tar`
await new Deno.Command("tar", {
  args: ["-xf", "archive.tar"],
  stdout: "inherit",
  stderr: "inherit",
  cwd: destDir,
}).output()

// TODO: automatically enable? (thoughts piotr-iohk?)
// await Deno.copyFile(
//   import.meta.resolve("./enable_logging.sh"),
//   path.join(destDir, "enable_logging.sh"),
// )
