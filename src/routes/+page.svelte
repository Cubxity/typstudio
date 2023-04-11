<script lang="ts">
  import Editor from "../components/Editor.svelte";
  import Preview from "../components/Preview.svelte";
  import Explorer from "../components/ExplorerTree.svelte";
  import { project, shell } from "../lib/stores";
  import type { ProjectChangeEvent } from "../lib/ipc";
  import Empty from "../components/Empty.svelte";
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";

  onMount(() => {
    return appWindow.listen<ProjectChangeEvent>("project_changed", ({ payload }) => {
      shell.selectFile(undefined);
      project.set(payload.project);
    });
  });

</script>

<div class="flex flex-row h-screen bg-[#1e1e1e] text-white">
  <Explorer class="w-80" />
  {#if $shell.selectedFile}
    <Editor class="min-w-0 flex-1" path={$shell.selectedFile} />
    <Preview class="flex-1" />
  {:else}
    <Empty text={$project ? "No file selected" : "No project open"} />
  {/if}
</div>
