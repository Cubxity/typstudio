<script lang="ts">
  import type { TypstRenderResponse } from "../lib/ipc";
  import { render } from "../lib/ipc";
  import { onMount } from "svelte";

  export let page: number;
  export let hash: string;
  export let width: number;
  export let height: number;
  export let scale: number;

  let canvas: HTMLCanvasElement;
  let canRender = false;
  let isIntersecting = false;
  let nonce = 1;
  let lastNonce = 0;

  onMount(() => {
    const observer = new IntersectionObserver((entries) => {
      // Don't update canRender when the preview page is no longer visible
      isIntersecting = entries[0].isIntersecting;
      if (isIntersecting) canRender = true;
    });
    observer.observe(canvas);
    return () => observer.disconnect();
  });

  const invalidateCanRender = (_hash: string, _scale: number) => {
    canRender = isIntersecting;
  };

  const update = async (updateHash: string, updateScale: number) => {
    // return
    // adjust dimensions to account for device pixel ratio
    const densityWidth = Math.floor(width * window.devicePixelRatio);
    const densityHeight = Math.floor(height * window.devicePixelRatio);
    const densityScale = Math.floor(updateScale * window.devicePixelRatio);

    const res: TypstRenderResponse = await render(page, densityScale, nonce++);

    const img = new Image(res.width, res.height);
    img.src = "data:image/png;base64," + res.image;
    img.onload = () => {
      // Prevent out-of-order rendering
      if (res.nonce > lastNonce) {
        lastNonce = res.nonce;
        const ctx = canvas.getContext("2d");

        canvas.width = densityWidth;
        canvas.height = densityHeight;

        ctx.drawImage(img, 0, 0, densityWidth, densityHeight);
      }
    };
  };

  $: invalidateCanRender(hash, scale);
  $: if (canRender) update(hash, scale);
</script>

<div
  class="bg-white shadow-md mx-auto"
  style="height: {height}px; min-height: {height}px; width: {width}px; min-width: {width}px; box-sizing: border-box;">
  <canvas class="bg-white w-full h-full" bind:this={canvas}></canvas>
</div>