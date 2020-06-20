export type OutputStyle = "compact" | "nested" | "compressed" | "expanded";

export interface IOptions {
  output_style: OutputStyle;
  precision: number;
  indented_syntax: boolean;
  include_paths: string[];
}
