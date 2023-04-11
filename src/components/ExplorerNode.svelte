<script lang="ts">

  import clsx from "clsx";
  import FileIcon from "./icons/FileIcon.svelte";
  import FolderIcon from "./icons/FolderIcon.svelte";
  import ArrowDropDownIcon from "./icons/ArrowDropDownIcon.svelte";
  import ArrowRightIcon from "./icons/ArrowRightIcon.svelte";
  import { invoke } from "@tauri-apps/api";
  import type { FileEntry, FileType, FsListResponse } from "../lib/ipc";
  import { shell } from "../lib/stores";

  export let type: FileType;
  export let path: string;

  let expanded = false;
  let files: FileEntry[] = [];

  const handleClick = () => {
    if (type === "directory") {
      expanded = !expanded;
    } else {
      shell.selectFile(path);
    }
  };

  $: {
    if (expanded) {
      invoke<FsListResponse>("fs_list", { path })
        .then(res => {
          files = res.files;
        });
    }
  }
</script>

<div
  class={clsx(
    "text-sm rounded-md pr-2 py-0.5 hover:bg-neutral-700/50 text-white fill-white flex items-center transition",
    path === "" && "font-bold",
    $shell.selectedFile === path && "bg-neutral-700")
  }
  style="padding-left: {path === '' ? 8 : 8 + path.split('/').length * 10}px"
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
  {path === "" ? "root" : path.slice(path.lastIndexOf("/") + 1)}
</div>
{#if expanded}
  {#each files as file}
    <svelte:self type={file.type} path={path === "" ? `${path}${file.name}` : `${path}/${file.name}`} />
  {/each}
{/if}
