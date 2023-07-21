<script lang="ts">
  import clsx from "clsx";
  import FileIcon from "./icons/FileIcon.svelte";
  import FolderIcon from "./icons/FolderIcon.svelte";
  import ArrowDropDownIcon from "./icons/ArrowDropDownIcon.svelte";
  import ArrowRightIcon from "./icons/ArrowRightIcon.svelte";
  import type { FileItem, FileType, FSRefreshEvent } from "../lib/ipc";
  import { project, shell } from "../lib/stores";
  import { listDir } from "../lib/ipc";
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";

  export let type: FileType;
  export let path: string;

  let expanded = path === "/";
  let files: FileItem[] = [];

  const handleClick = () => {
    if (type === "directory") {
      expanded = !expanded;
    } else {
      shell.selectFile(path);
    }
  };

  const update = async () => {
    files = await listDir(path);
  };

  onMount(() => {
    return appWindow.listen<FSRefreshEvent>("fs_refresh", ({ payload }) => {
      if (`/${payload.path}` === path) update();
    });
  });

  $: {
    if (expanded) {
      update();
    }
  }

  if (path === "/") {
    onMount(() => project.subscribe(update));
  }
</script>

{#if path !== "/"}
  <div
    class={clsx(
      "text-sm rounded-md pr-2 py-0.5 hover:bg-neutral-700/50 text-white fill-white flex items-center transition",
      $shell.selectedFile === path && "bg-neutral-700"
    )}
    style="padding-left: {path.split('/').length * 10}px"
    on:click={handleClick}
    role="button"
  >
    {#if type === "directory"}
      <svelte:component
        this={expanded ? ArrowDropDownIcon : ArrowRightIcon}
        class="w-4 h-4 inline fill-neutral-500 mr-1"
      />
    {/if}
    <svelte:component
      this={type === "file" ? FileIcon : FolderIcon}
      class={clsx("w-4 h-4 inline fill-neutral-500 mr-2", type === "file" && "ml-5")}
    />
    <span class="flex-1 truncate">
    {path === "/" ? "root" : path.slice(path.lastIndexOf("/") + 1)}
  </span>
  </div>
{/if}
{#if expanded}
  {#each files as file}
    <svelte:self
      type={file.type}
      path={path === "/" ? `${path}${file.name}` : `${path}/${file.name}`}
    />
  {/each}
{/if}
