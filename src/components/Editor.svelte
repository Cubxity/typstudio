<script lang="ts">
  import { onMount } from "svelte";
  import type { editor as editorType } from "monaco-editor";
  import { invoke } from "@tauri-apps/api";
  import debounce from "lodash/debounce";

  import ICodeEditor = editorType.ICodeEditor;
  import IModelContentChangedEvent = editorType.IModelContentChangedEvent;
  import type { FsReadResponse } from "../lib/ipc";

  let divEl: HTMLDivElement;
  let editor: ICodeEditor;

  export let path: string;

  const handleUpdate = () => {
    invoke("fs_update_file", {
      path,
      content: editor.getModel().getValue()
    });
  };
  const handleUpdateDebounce = debounce(handleUpdate, 100, { maxWait: 300 });

  onMount(async () => {
    const EditorWorker = await import("monaco-editor/esm/vs/editor/editor.worker?worker");
    const monaco = await import("monaco-editor");

    // @ts-ignore
    self.MonacoEnvironment = {
      getWorker: function(_moduleId: any, label: string) {
        return new EditorWorker();
      }
    };

    editor = monaco.editor.create(divEl, {
      value: "",
      language: "typst",
      theme: "vs-dark",
      lineHeight: 1.8,
      automaticLayout: true
    });

    editor.onDidChangeModelContent((e: IModelContentChangedEvent) => {
      handleUpdateDebounce();
    });

    return () => {
      editor.dispose();
    };
  });

  const fetchContent = (editor: ICodeEditor, path: string) => {
    if (!editor) return;

    // TODO: Ensure file integrity
    // Make sure that the file does not get overridden by the old file due to desynced debounce
    // Make sure that the editor does not get override by pending file loads due to desynced load
    // Make sure that the old file is saved properly before loading the new file

    invoke<FsReadResponse>("fs_read_file", { path })
      .then(res => {
        editor.getModel().setValue(res.content);
      });
  };

  $: fetchContent(editor, path);
</script>

<div bind:this={divEl} class={$$props.class}></div>