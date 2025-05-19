<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { X, Minus } from "lucide-svelte";
    import Button from "$components/header/Button.svelte";

    const window = getCurrentWindow();
</script>

<div data-tauri-drag-region class="bg-honey flex items-center h-10">
    <h1 class="font-title mx-4 text-lg">animalese</h1>
    <div class="flex-grow"></div>
    <div class="flex items-center h-full">
        <Button Icon={Minus} onclick={() => window.minimize()} />
        <Button Icon={X} onclick={() => window.close()} />
    </div>
</div>