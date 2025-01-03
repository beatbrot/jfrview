// See https://coolors.co/user/palettes/6778041fba6e95000b08d65d
export const palette = {
  exec: "#F7B267",
  other: "#F25C54",
};

export function pick_color(data: any): string {
  return data.kind === "Exec" ? palette.exec : palette.other;
}
