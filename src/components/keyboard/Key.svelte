<script lang="ts">
  import type { Icon as IconType } from "lucide-svelte";

  export interface Props {
    span?: number;
    label?: string;
    icon?: typeof IconType;
    active?: boolean;
  }

  let { span = 2, label, icon: Icon, active }: Props = $props();
  const activeClass = $derived(active ? "bg-honey shadow-md" : "bg-sand");
</script>

<button
  class="{activeClass} border-twig border-1 m-1 flex h-8 items-center justify-center gap-1 rounded-xl text-sm transition-colors duration-100 hover:shadow-md"
  tabindex="-1"
  style="grid-column-end: span {span};"
>
  {#if Icon}
    <Icon />
  {/if}
  {#if label}
    {label}
  {/if}
</button>
