<script lang="ts">
  import { onMount } from "svelte";
  import type { editor as editorType } from "monaco-editor";
  import debounce from "lodash/debounce";

  import { initMonaco } from "../lib/editor/monaco";
  import type { TypstCompileEvent } from "../lib/ipc";
  import { compile, readFileText, writeFileText } from "../lib/ipc";
  import { appWindow } from "@tauri-apps/api/window";
  import ICodeEditor = editorType.ICodeEditor;
  import IModelContentChangedEvent = editorType.IModelContentChangedEvent;
  import IModelChangedEvent = editorType.IModelChangedEvent;
  import IMarkerData = editorType.IMarkerData;
  import { paste } from "$lib/ipc/clipboard";
  import { throttle } from "$lib/fn";

  let divEl: HTMLDivElement;
  let editor: ICodeEditor;
  const monacoImport = import("monaco-editor");

  export let path: string;

  const handleCompile = async () => {
    const model = editor.getModel();
    if (model) {
      // Removing the preceding slash
      await compile(model.uri.path, model.getValue());
    }
  };
  const handleSave = () => {
    const model = editor.getModel();
    if (model) {
      // Removing the preceding slash
      writeFileText(model.uri.path, model.getValue());
    }
  };

  const handleCompileThrottle = throttle(handleCompile);
  const handleSaveDebounce = debounce(handleSave, 1000, { maxWait: 5000 });

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
      readOnly: true,
      folding: true,
      quickSuggestions: false,
      wordWrap: "on",
      unicodeHighlight: { ambiguousCharacters: false }
    });

    editor.onDidChangeModel((e: IModelChangedEvent) => {
      handleCompileThrottle();
    });
    editor.onDidChangeModelContent((e: IModelContentChangedEvent) => {
      // Compile will update the source file directly in the memory without
      // writing to the file system first, this will reduce the preview delay.
      handleCompileThrottle();
      handleSaveDebounce();
    });

    return () => {
      editor.dispose();
    };
  });

  onMount(async () => {
    const monaco = await monacoImport;

    // Returns an unlisten function
    return appWindow.listen<TypstCompileEvent>("typst_compile", ({ event, payload }) => {
      const { errors } = payload;
      const model = editor.getModel();
      if (model) {
        const markers: IMarkerData[] = errors?.map(({ range, message }) => {
          const start = model.getPositionAt(range.start);
          const end = model.getPositionAt(range.end);
          return {
            startLineNumber: start.lineNumber,
            startColumn: start.column,
            endLineNumber: end.lineNumber,
            endColumn: end.column,
            message,
            severity: monaco.MarkerSeverity.Error
          };
        }) ?? [];

        monaco.editor.setModelMarkers(model, "owner", markers);
      }
    });
  });

  const fetchContent = async (editor: ICodeEditor, path: string) => {
    if (!editor) return;

    // Prevent further updates and immediately flush pending updates
    editor.updateOptions({ readOnly: true });
    handleSaveDebounce.flush();

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

  const handlePaste = async (event: ClipboardEvent) => {
    const text = event.clipboardData?.getData("text");
    if (text === "") {
      // Could be an image or something else
      // TODO: Check if this workaround is required on Windows
      event.preventDefault();
      const res = await paste();

      const range = editor.getSelection();
      const model = editor.getModel();
      if (range && model) {
        model.pushEditOperations([], [
          {
            range: range,
            text: `\n#figure(\n  image("${res.path}"),\n  caption: []\n)\n`
          }
        ], () => null);
      }
    }
  };

  $: fetchContent(editor, path);
</script>

<div bind:this={divEl} on:paste={handlePaste} class={$$props.class}></div>