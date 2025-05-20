<script lang="ts">
  import Key, { type Props as KeyProps } from "$components/keyboard/Key.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { platform } from "@tauri-apps/plugin-os";
  import {
    ArrowBigUp,
    ArrowBigUpDash,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowRightToLine,
    ArrowUp,
    ChevronUp,
    Command,
    CornerDownLeft,
    Option,
    SquareMenu,
  } from "lucide-svelte";
  import { writable } from "svelte/store";

  type KeyInfo = Omit<KeyProps, "active"> & { rdev: string };

  function letters(letters: string, span: number = 2): KeyInfo[] {
    return letters.split("").map((label) => ({
      label: label.toLowerCase(),
      rdev: `Key${label}`,
      span,
    }));
  }

  const layout: (KeyInfo | null)[][] = [
    [
      { label: "`", rdev: "Unknown(223)" },
      ...Array.from({ length: 9 }).map((_, i) => ({
        label: (i + 1).toString(),
        rdev: `Num${i + 1}`,
      })),
      { label: "0", rdev: "Num0" },
      { label: "-", rdev: "Minus" },
      { label: "=", rdev: "Equal" },
      { icon: ArrowLeft, rdev: "Backspace", span: 5 },
    ],
    [
      {
        icon: ArrowRightToLine,
        span: 3,
        rdev: "Tab",
      },
      ...letters("QWERTYUIOP"),
      {
        label: "[",
        rdev: "LeftBracket",
      },
      {
        label: "]",
        rdev: "RightBracket",
      },
      {
        label: "\\",
        rdev: "BackSlash",
        span: 4,
      },
    ],
    [
      {
        label: "Caps",
        rdev: "CapsLock",
        span: 4,
      },
      ...letters("ASDFGHJKL"),
      {
        label: ";",
        rdev: "SemiColon",
      },
      {
        label: "'",
        rdev: "BackQuote",
      },
      {
        icon: CornerDownLeft,
        rdev: "Return",
        span: 5,
      },
    ],
    [
      {
        icon: ArrowBigUpDash,
        rdev: "ShiftLeft",
        span: 5,
      },
      ...letters("ZXCVBNM"),
      {
        label: ",",
        rdev: "Comma",
      },
      {
        label: ".",
        rdev: "Dot",
      },
      {
        label: "/",
        rdev: "Slash",
      },
      {
        icon: ArrowBigUpDash,
        rdev: "ShiftRight",
        span: 6,
      },
    ],
    [
      {
        label: "Ctrl",
        rdev: "ControlLeft",
        span: 3,
      },
      {
        label: platform() === "windows" ? "Win" : undefined,
        icon: platform() === "windows" ? undefined : Command,
        rdev: "MetaLeft",
        span: 3,
      },
      {
        label: "Alt",
        rdev: "Alt",
        span: 3,
      },
      {
        label: " ",
        rdev: "Space",
        span: 10,
      },
      {
        label: "Alt",
        rdev: "AltGr",
        span: 3,
      },
      {
        label: "Ctrl",
        rdev: "ControlRight",
        span: 3,
      },
    ],
  ];

  const activeMap = writable(
    new Map(
      layout
        .flat()
        .filter(Boolean)
        .map((key) => [key!.rdev, false]),
    ),
  );

  function setMap(rdev: string, value: boolean) {
    console.log(rdev, value);
    activeMap.update((map) => {
      map.set(rdev, value);
      return new Map(map);
    });
  }

  $effect(() => {
    // key handlers
    listen<string>("KeyPress", (e) => setMap(e.payload, true));
    listen<string>("KeyRelease", (e) => setMap(e.payload, false));

    // release all keys on blur
    // listen("tauri://blur", () => {
    //   activeMap.update((map) => {
    //     for (const key of map.keys()) {
    //       map.set(key, false);
    //     }
    //     return new Map(map);
    //   });
    // });
  });
</script>

<div class="grid w-1/2 gap-2">
  {#each layout as row, i}
    <div
      class="grid-cols-36 grid h-8 {i === layout.length - 1
        ? '**:h-9'
        : undefined}"
    >
      {#each row as key}
        {#if key}
          <Key
            active={$activeMap.get(key.rdev) ?? false}
            label={key.label}
            icon={key.icon}
            span={key.span}
          />
        {:else}
          <div></div>
        {/if}
      {/each}
      {#if i === layout.length - 1}
        <div class="col-span-4">
          <div class="**:!h-4 **:w-7 grid grid-cols-3 gap-x-7 gap-y-1">
            <div></div>
            <div><Key active={$activeMap.get("UpArrow")} icon={ArrowUp} /></div>
            <div></div>
            <div>
              <Key active={$activeMap.get("LeftArrow")} icon={ArrowLeft} />
            </div>
            <div>
              <Key active={$activeMap.get("DownArrow")} icon={ArrowDown} />
            </div>
            <div>
              <Key active={$activeMap.get("RightArrow")} icon={ArrowRight} />
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/each}
</div>
