<script>
  import FolderIcon from "./icons/FolderIcon.svelte";
  import ExplorerTree from "./ExplorerTree.svelte";
  import clsx from "clsx";

  // TODO: Implement a modular side panel system?
  let explorerVisible = true;

  const toggleExplorer = () => {
    explorerVisible = !explorerVisible;
  };
</script>

<div class="p-1.5 border-r border-neutral-700">
  <button
    class={clsx("rounded-md p-1 transition-colors hover:bg-neutral-700/60", explorerVisible && "bg-neutral-700/80")}
    on:click={toggleExplorer}
  >
    <FolderIcon class="w-5 h-5 text-neutral-300" />
  </button>
</div>
{#if explorerVisible}
  <ExplorerTree class="w-80" />
{/if}

