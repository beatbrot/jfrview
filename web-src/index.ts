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

const includeNativeToggle = refreshGraphOnChange("includeNativeToggle");
const threadsToggle = refreshGraphOnChange("threadsToggle");

const details = document.getElementById("details") as HTMLSpanElement;

configureEl("chart", (el) => {
  el.ondrop = (e) => {
    e.preventDefault();
    fileSelected(e.dataTransfer!!.files[0]);
  };
  el.ondragover = (ev) => ev.preventDefault();
});

let activeBytes: Uint8Array | null = null;

configureHmr();

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
  const result = parse(
    activeBytes,
    includeNativeToggle.checked,
    threadsToggle.checked,
  );
  console.timeEnd("flamegraph");

  const chart = fg
    .flamegraph()
    .width(960)
    .minFrameSize(1)
    .setColorMapper((data, orig) => {
      return data.data.kind === "Thread" ? "#ff0000" : orig;
    })
    .onHover((d) => {
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

function refreshGraphOnChange(id: string): HTMLInputElement {
  return configureEl(id, (el: HTMLInputElement) => {
    el.onchange = refresh_graph;
  });
}

/**
 * Restore the current state of the flamegraph after hot module reload
 */
function configureHmr() {
  if (module.hot) {
    module.hot.dispose((data: any) => {
      data.activeBytes = activeBytes;
    });
    module.hot.accept((_: any) => {
      activeBytes = module.hot.data.activeBytes;
      refresh_graph();
    });
  }
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

declare const module: {
  hot: any;
};

declare module "d3-flame-graph" {
  interface FlameGraph {
    onHover(handler: (data: Data) => void): FlameGraph;
  }
}
