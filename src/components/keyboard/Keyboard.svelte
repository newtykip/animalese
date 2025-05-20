<script lang="ts">
  import Key from "$components/keyboard/Key.svelte";
  import { layout } from "$consts/keyboard";
  import { listen } from "@tauri-apps/api/event";
  import { SvelteSet } from "svelte/reactivity";
  import {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    type Icon,
  } from "lucide-svelte";

  let activeKeys = new SvelteSet<string>();

  $effect(() => {
    // key handlers
    const press = listen<string>("KeyPress", (e) => activeKeys.add(e.payload));
    const release = listen<string>("KeyRelease", (e) =>
      activeKeys.delete(e.payload),
    );

    // cleanup
    return () => {
      press.then((unlisten) => unlisten());
      release.then((unlisten) => unlisten());
    };
  });
</script>

<div class="grid w-1/2 gap-2">
  {#each layout as row, i}
    <div
      class="grid-cols-36 grid h-8 {i === layout.length - 1
        ? '**:h-9'
        : undefined}"
    >
      {#each row as { key, label, span }}
        {@const active = activeKeys.has(key)}
        <Key {active} {key} {label} {span} />
      {/each}
      {#if i === layout.length - 1}
        <div class="col-span-4">
          <div class="**:!h-4 **:w-7 grid grid-cols-3 gap-x-7 gap-y-1">
            <div></div>
            {@render arrow("UpArrow", ArrowUp)}
            <div></div>
            {@render arrow("LeftArrow", ArrowLeft)}
            {@render arrow("DownArrow", ArrowDown)}
            {@render arrow("RightArrow", ArrowRight)}
          </div>
        </div>
      {/if}
    </div>
  {/each}
</div>

{#snippet arrow(key: string, icon: typeof Icon)}
  <div>
    <Key active={activeKeys.has(key)} {key} label={icon} />
  </div>
{/snippet}
