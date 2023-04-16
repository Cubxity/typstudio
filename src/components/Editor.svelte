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
  const monacoImport = import("monaco-editor");

  export let path: string;

  const handleUpdate = () => {
    const model = editor.getModel();
    if (model) {
      // Removing the preceding slash
      const path = model.uri.path.substring(1);
      writeFileText(path, model.getValue());
    }
  };
  const handleUpdateDebounce = debounce(handleUpdate, 100, { maxWait: 300 });

  onMount(async () => {
    const EditorWorker = await import("monaco-editor/esm/vs/editor/editor.worker?worker");
    await initMonaco;

    // @ts-ignore
    self.MonacoEnvironment = {
      getWorker: function(_moduleId: any, label: string) {
        return new EditorWorker.default();
      }
    };

    editor = (await monacoImport).editor.create(divEl, {
      lineHeight: 1.8,
      automaticLayout: true,
      readOnly: true
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

    // Prevent further updates and immediately flush pending updates
    editor.updateOptions({ readOnly: true });
    handleUpdateDebounce.flush();

    editor.getModel()?.dispose();

    try {
      const content = await readFileText(path);
      const monaco = await monacoImport;
      const uri = monaco.Uri.file(path);

      let model = monaco.editor.getModel(uri);
      if (model) {
        // Update existing model. This should only be possible in development mode
        // after hot reload.
        model.setValue(content);
      } else {
        model = monaco.editor.createModel(content, undefined, uri);
      }

      editor.setModel(model);
    } finally {
      editor.updateOptions({ readOnly: false });
    }
  };

  $: fetchContent(editor, path);
</script>

<div bind:this={divEl} class={$$props.class}></div>