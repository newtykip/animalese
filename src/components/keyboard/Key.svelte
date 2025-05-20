<script lang="ts">
  import type { Key } from "$consts/keyboard";
  import { invoke } from "@tauri-apps/api/core";

  export type Props = Key & {
    active: boolean;
  };

  const { label, key, span = 2, active }: Props = $props();
  const isIcon = typeof label === "function";

  $effect(() => {
    console.log("Key", { label, key, span, active });
  });
</script>

<button
  class=" border-twig border-1 m-1 flex h-8 items-center justify-center gap-1 rounded-xl text-sm transition-colors duration-100 hover:shadow-md {active
    ? 'bg-honey shadow-md'
    : 'bg-sand'}"
  tabindex="-1"
  style="grid-column-end: span {span};"
  onclick={() => invoke("simulate_key", { key })}
>
  {#if isIcon}
    <!-- svelte-ignore svelte_component_deprecated -->
    <svelte:component this={label} />
  {:else}
    {label}
  {/if}
</button>
