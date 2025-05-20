<script lang="ts">
  import type { Icon as IconType } from "lucide-svelte";

  interface Props {
    class?: string;
    Icon: typeof IconType;
    onclick?: () => void;
  }

  let { class: clazz, Icon, onclick }: Props = $props();
</script>

<button class="group flex h-full items-center px-2 {clazz}" {onclick}>
  <Icon class="group-hover:scale-115 transition-transform duration-150" />
</button>
