<script lang="ts">
  import type { TypstRenderResponse } from "../lib/ipc";
  import { render } from "../lib/ipc";

  export let page: number;
  export let hash: string;
  export let width: number;
  export let height: number;
  export let scale: number;

  let canvas: HTMLCanvasElement;

  const update = async (hash: string, updateScale: number) => {
    // return
    const res: TypstRenderResponse = await render(page, updateScale);

    const img = new Image(width, height);
    img.src = "data:image/png;base64," + res.image;
    img.onload = () => {
      if (scale === updateScale) {
        const ctx = canvas.getContext("2d");
        canvas.width = width;
        canvas.height = height;

        ctx.drawImage(img, 0, 0);
      }
    };
  };

  $: update(hash, scale);
</script>

<div
  class="bg-white shadow-md mx-auto"
  style="height: {height}px; min-height: {height}px; width: {width}px; min-width: {width}px; box-sizing: border-box;">
  <canvas class="bg-white w-full h-full" bind:this={canvas}></canvas>
</div>