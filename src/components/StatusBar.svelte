<script lang="ts">
  import { PreviewState, shell } from "$lib/stores";
  import { XCircleIcon, Disc3Icon } from "lucide-svelte";
</script>

<div
  class="flex flex-row items-center h-7 border-t border-neutral-700 text-neutral-200 font-medium text-sm"
>
  <div class="flex-1" />
  {#if $shell.previewState === PreviewState.CompileError}
    <div class="flex flex-row items-center px-2 bg-red-500 h-full gap-2">
      <XCircleIcon class="w-4 h-4" />
      Compile error
    </div>
  {:else if $shell.previewState === PreviewState.Compiling}
    <div class="flex flex-row items-center px-2 h-full gap-2">
      <Disc3Icon class="w-4 h-4 animate-spin" />
      Compiling...
    </div>
  {/if}
</div>
