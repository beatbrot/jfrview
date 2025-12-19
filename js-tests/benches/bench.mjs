import { withCodSpeed } from "@codspeed/tinybench-plugin";
import fs from "node:fs/promises";
import { Bench } from "tinybench";
import init, { interpret_jfr } from "jfrview";

async function initWasm() {
  const wasm_module = await fs.readFile("node_modules/jfrview/jfrview_bg.wasm");

  await init({ module_or_path: wasm_module });
}

await initWasm();

const bench = withCodSpeed(new Bench());

const heavy = await fs.readFile("../test-data/heavy.jfr");

bench.add("Node: Parse Heavy", () => {
  interpret_jfr(heavy);
});

await bench.run();

console.table(bench.table());
