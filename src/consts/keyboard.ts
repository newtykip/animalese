import { platform } from "@tauri-apps/plugin-os";
import {
  ArrowBigUpDash,
  ArrowLeft,
  ArrowRightToLine,
  Command,
  CornerDownLeft,
  type Icon,
} from "lucide-svelte";

export interface Key {
  label?: string | typeof Icon;
  key: string;
  span?: number;
}

function letters(letters: string, span: number = 2): Key[] {
  return letters.split("").map((label) => ({
    label: label.toLowerCase(),
    key: `Key${label}`,
    span,
  }));
}

export const layout: Key[][] = [
  [
    { label: "`", key: "Unknown(223)" },
    ...Array.from({ length: 9 }).map((_, i) => ({
      label: (i + 1).toString(),
      key: `Num${i + 1}`,
    })),
    { label: "0", key: "Num0" },
    { label: "-", key: "Minus" },
    { label: "=", key: "Equal" },
    { label: ArrowLeft, key: "Backspace", span: 5 },
  ],
  [
    {
      label: ArrowRightToLine,
      span: 3,
      key: "Tab",
    },
    ...letters("QWERTYUIOP"),
    {
      label: "[",
      key: "LeftBracket",
    },
    {
      label: "]",
      key: "RightBracket",
    },
    {
      label: "\\",
      key: "BackSlash",
      span: 4,
    },
  ],
  [
    {
      label: "Caps",
      key: "CapsLock",
      span: 4,
    },
    ...letters("ASDFGHJKL"),
    {
      label: ";",
      key: "SemiColon",
    },
    {
      label: "'",
      key: "BackQuote",
    },
    {
      label: CornerDownLeft,
      key: "Return",
      span: 5,
    },
  ],
  [
    {
      label: ArrowBigUpDash,
      key: "ShiftLeft",
      span: 5,
    },
    ...letters("ZXCVBNM"),
    {
      label: ",",
      key: "Comma",
    },
    {
      label: ".",
      key: "Dot",
    },
    {
      label: "/",
      key: "Slash",
    },
    {
      label: ArrowBigUpDash,
      key: "ShiftRight",
      span: 6,
    },
  ],
  [
    {
      label: "Ctrl",
      key: "ControlLeft",
      span: 3,
    },
    {
      label: platform() === "windows" ? "Win" : Command,
      key: "MetaLeft",
      span: 3,
    },
    {
      label: "Alt",
      key: "Alt",
      span: 3,
    },
    {
      label: " ",
      key: "Space",
      span: 10,
    },
    {
      label: "Alt",
      key: "AltGr",
      span: 3,
    },
    {
      label: "Ctrl",
      key: "ControlRight",
      span: 3,
    },
  ],
];
