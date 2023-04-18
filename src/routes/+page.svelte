<script lang="ts">
  import Editor from "../components/Editor.svelte";
  import Preview from "../components/Preview.svelte";
  import { project, shell } from "../lib/stores";
  import type { ProjectChangeEvent } from "../lib/ipc";
  import Empty from "../components/Empty.svelte";
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import StatusBar from "../components/StatusBar.svelte";
  import SidePanel from "../components/SidePanel.svelte";
  import Modals from "../components/ShellModal.svelte";

  onMount(() => {
    return appWindow.listen<ProjectChangeEvent>("project_changed", ({ payload }) => {
      shell.selectFile(undefined);
      project.set(payload.project);
    });
  });

</script>

<div class="flex flex-col max-h-screen h-screen bg-[#1e1e1e]">
  <Modals />
  <div class="flex flex-row flex-1 text-white min-h-0">
    <SidePanel />
    {#if $shell.selectedFile}
      <Editor class="min-w-0 flex-1" path={$shell.selectedFile} />
      <Preview class="min-w-0 flex-1" />
    {:else}
      <Empty text={$project ? "No file selected" : "No project open"} />
    {/if}
  </div>
  <StatusBar />
</div>