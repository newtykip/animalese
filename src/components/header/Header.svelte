<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { X, Minus } from "lucide-svelte";
  import Button from "$components/header/Button.svelte";

  const window = getCurrentWindow();
</script>

<div data-tauri-drag-region class="bg-honey flex h-10 items-center">
  <h1 class="mx-4 text-lg">animalese</h1>
  <div class="flex-grow"></div>
  <div class="flex h-full items-center">
    <Button Icon={Minus} onclick={() => window.minimize()} />
    <Button Icon={X} onclick={() => window.close()} />
  </div>
</div>
