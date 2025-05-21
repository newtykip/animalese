<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { X, Minus } from "lucide-svelte";
  import Button from "$components/header/Button.svelte";

  const window = getCurrentWindow();
</script>

<div class="bg-honey flex h-10">
  <div class="ml-4 flex items-center gap-2">
    <img data-tauri-drag-region src="/leaf.png" alt="Leaf" />
    <h1 data-tauri-drag-region class="text-lg">animalese</h1>
  </div>
  <div data-tauri-drag-region class="flex-grow"></div>
  <div class="flex h-full items-center">
    <Button Icon={Minus} onclick={() => window.minimize()} />
    <Button Icon={X} onclick={() => window.close()} />
  </div>
</div>
