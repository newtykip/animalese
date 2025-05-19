<script lang="ts">
    import type { Icon as IconType } from "lucide-svelte";

    interface Props {
        class?: string;
        Icon: typeof IconType;
        onclick?: () => void;
    }

    let { class: clazz, Icon, onclick }: Props = $props();
</script>

<button class="px-2 h-full flex items-center group {clazz}" {onclick} >
    <Icon class="transition-transform duration-150 group-hover:scale-115" />
</button>