<script lang="ts">

  import clsx from "clsx";
  import ExplorerNode from "./ExplorerNode.svelte";
  import AddIcon from "./icons/AddIcon.svelte";
  import { project, shell } from "$lib/stores";
  import { createFile } from "$lib/ipc";

  const handleCreate = () => {
    shell.createModal({
      type: "input",
      title: "Create file",
      callback: (path) => {
        if (path) {
          createFile(path);
        }
      }
    });
  };
</script>

<div class={clsx("border-r border-neutral-700 p-3 select-none flex flex-col", $$props.class)}>
  {#if $project}
    <div class="flex flex-row mx-2 mt-1 mb-3 items-center">
      <span class="text-lg font-bold block flex-1">Project</span>
      <div class="flex flex-row rounded-md border border-neutral-700 overflow-clip">
        <button class="p-1 transition-colors hover:bg-neutral-700" on:click={handleCreate}>
          <AddIcon class="w-4 h-4" />
        </button>
      </div>
    </div>
    <div class="flex-1 min-h-0 overflow-auto">
      <ExplorerNode type="directory" path="/" />
    </div>
  {/if}
</div>
