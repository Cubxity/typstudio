<script lang="ts">
  import { onMount } from "svelte";
  import type { editor as editorType } from "monaco-editor";
  import debounce from "lodash/debounce";

  import ICodeEditor = editorType.ICodeEditor;
  import IModelContentChangedEvent = editorType.IModelContentChangedEvent;
  import { initMonaco } from "../lib/editor/monaco";
  import { readFileText, writeFileText } from "../lib/ipc";

  let divEl: HTMLDivElement;
  let editor: ICodeEditor;

  export let path: string;

  const handleUpdate = () => {
    const content = editor.getModel()?.getValue();
    if (content) {
      writeFileText(path, content);
    }
  };
  const handleUpdateDebounce = debounce(handleUpdate, 100, { maxWait: 300 });

  onMount(async () => {
    const EditorWorker = await import("monaco-editor/esm/vs/editor/editor.worker?worker");
    const monaco = await import("monaco-editor");
    await initMonaco;

    // @ts-ignore
    self.MonacoEnvironment = {
      getWorker: function(_moduleId: any, label: string) {
        return new EditorWorker.default();
      }
    };

    editor = monaco.editor.create(divEl, {
      value: "",
      language: "typst",
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

  const fetchContent = async (editor: ICodeEditor, path: string) => {
    if (!editor) return;

    // TODO: Ensure file integrity
    // Make sure that the file does not get overridden by the old file due to desynced debounce
    // Make sure that the editor does not get override by pending file loads due to desynced load
    // Make sure that the old file is saved properly before loading the new file
    const content = await readFileText(path);
    editor.getModel()?.setValue(content);
  };

  $: fetchContent(editor, path);
</script>

<div bind:this={divEl} class={$$props.class}></div>