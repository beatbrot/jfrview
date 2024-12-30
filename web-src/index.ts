import init, { parse } from "../pkg/jfrview";
import * as fg from "d3-flame-graph";
import { select } from "d3-selection";

const btn = configureEl("fileBtn", (input) => {
  const el = document.createElement("input");
  el.type = "file";
  el.multiple = false;
  input.onclick = () => el.click();
  el.onchange = () => {
    return fileSelected(el.files!![0]);
  };
});

const includeNativeToggle = configureEl(
  "includeNativeToggle",
  (el: HTMLInputElement) => {
    el.onchange = refresh_graph;
  },
);

const details = document.getElementById("details") as HTMLSpanElement;

configureEl("chart", (el) => {
  el.ondrop = (e) => {
    e.preventDefault();
    fileSelected(e.dataTransfer!!.files[0]);
  };
  el.ondragover = (ev) => ev.preventDefault();
});

let activeBytes: Uint8Array | null = null;

function fileSelected(data: File) {
  const fr = new FileReader();
  fr.onloadend = (e) => {
    activeBytes = new Uint8Array(e.target!!.result as ArrayBuffer);
    refresh_graph();
  };
  fr.readAsArrayBuffer(data);
  btn.innerText = data.name;
}

async function refresh_graph() {
  if (activeBytes == null) {
    return;
  }
  console.time("flamegraph");
  await init();
  const result = parse(activeBytes, includeNativeToggle.checked);
  console.timeEnd("flamegraph");

  const chart = fg
    .flamegraph()
    .width(960)
    .minFrameSize(1)
    .onHover((d: Data) => {
      details.innerText = `${d.data.name} (${d.data.value} samples)`;
    });

  select("#chart").datum(result).call(chart);
}

function configureEl<T extends HTMLElement>(
  id: string,
  func: (el: T) => void,
): T {
  const el = document.getElementById(id) as T;
  func(el);
  return el;
}

interface Data {
  /**
   * The payload
   */
  readonly data: {
    readonly name: string;
    readonly value: number;
  };
}

declare module "d3-flame-graph" {
  interface FlameGraph {
    onHover(handler: (arg0: any) => void): FlameGraph;
  }
}
