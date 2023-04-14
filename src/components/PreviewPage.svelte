<script lang="ts">
  import type { TypstRenderResponse } from "../lib/ipc";
  import { invoke } from "@tauri-apps/api";

  export let page: number;
  export let hash: string;

  let img: HTMLImageElement;

  const render = async (hash: string) => {
    console.log(`update: ${hash}`);

    // return
    const res: TypstRenderResponse = await invoke("typst_render", { page });
    const dataUrl = "data:image/png;base64," + res.image;
    const blob = await fetch(dataUrl).then(res => res.blob());
    img.src = URL.createObjectURL(blob);
  };

  $: render(hash);
</script>

<div class="w-[595px] h-[842px] min-w-[595px] min-h-[842px] bg-white shadow-md">
  <img class="bg-white" alt="" bind:this={img} />
</div>