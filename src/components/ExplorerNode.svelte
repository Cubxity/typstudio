<script lang="ts">
  import clsx from "clsx";
  import FileIcon from "./icons/FileIcon.svelte";
  import FolderIcon from "./icons/FolderIcon.svelte";
  import ArrowDropDownIcon from "./icons/ArrowDropDownIcon.svelte";
  import ArrowRightIcon from "./icons/ArrowRightIcon.svelte";
  import {
    createFile,
    createFolder,
    type FileItem,
    type FileType,
    type FSRefreshEvent,
  } from "../lib/ipc";
  import { project, shell } from "../lib/stores";
  import { listDir, deleteByPath } from "../lib/ipc";
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import CreateNewFolder from "./icons/CreateNewFolder.svelte";
  import AddIcon from "./icons/AddIcon.svelte";
  import FileAdd from "./icons/FileAdd.svelte";
  import Delete from "./icons/Delete.svelte";

  export let type: FileType;
  export let path: string;

  let expanded = path === "";
  let files: FileItem[] = [];

  const handleClick = (e: MouseEvent) => {
    e.stopPropagation();
    if (type === "directory") {
      expanded = !expanded;
    } else {
      shell.selectFile(path);
    }
  };
  const handleCreateFile = (e: MouseEvent) => {
    e.stopPropagation();
    shell.createModal({
      type: "input",
      title: "Create file",
      initialText: path + "/",
      callback: (path) => {
        if (path && path !== path + "/") {
          expanded = true;
          createFile(path);
        }
      },
    });
  };
  const handleCreateFolder = (e: MouseEvent) => {
    e.stopPropagation();
    expanded = true;
    shell.createModal({
      type: "input",
      title: "Create folder",
      initialText: path + "/",
      callback: (path) => {
        if (path) {
          expanded = true;
          createFolder(path);
        }
      },
    });
  };
  const deleteEntry = (e: MouseEvent) => {
    e.stopPropagation();
    shell.createModal({
      title: `Delete file ${path}?`,
      type: "confirm",
      callback: (canceled) => {
        if (!canceled) {
          deleteByPath(path);
        }
      },
    });
  };
  const update = async () => {
    files = await listDir(path);
  };

  onMount(() => {
    return appWindow.listen<FSRefreshEvent>("fs_refresh", ({ payload }) => {
      if (payload.path === path) update();
    });
  });

  $: {
    if (expanded) {
      update();
    }
  }

  if (path === "") {
    onMount(() => project.subscribe(update));
  }
</script>

{#if path !== ""}
  <div
    class={clsx(
      "text-sm rounded-md pr-2 py-0.5 hover:bg-neutral-700/50 text-white fill-white flex items-center transition [&:hover>button]:visible",
      path === "" && "font-bold",
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
      {path === "" ? "root" : path.slice(path.lastIndexOf("/") + 1)}
    </span>
    <button class="p-1 transition-colors  hover:bg-neutral-700 invisible" on:click={deleteEntry}>
      <Delete class="w-4 h-4" />
    </button>
    {#if type === "directory"}
      <button class="p-1 transition-colors hover:bg-neutral-700 invisible" on:click={handleCreateFile}>
        <FileAdd class="w-4 h-4" />
      </button>
      <button class="p-1 transition-colors hover:bg-neutral-700 invisible" on:click={handleCreateFolder}>
        <CreateNewFolder class="w-4 h-4" />
      </button>
    {/if}
  </div>
{/if}
{#if expanded}
  {#each files as file}
    <svelte:self
      type={file.type}
      path={path === "" ? `${path}${file.name}` : `${path}/${file.name}`}
    />
  {/each}
{/if}
