<script lang="ts">

  import clsx from "clsx";
  import PreviewPage from "./PreviewPage.svelte";
  import { onMount } from "svelte";
  import type { TypstCompileEvent } from "../lib/ipc";
  import { appWindow } from "@tauri-apps/api/window";

  const scales = [0.5, 1.0, 1.25, 1.5, 2, 3, 4];

  let container: HTMLDivElement;
  let previousEvent: MouseEvent | undefined;

  let isMouseDown = false;
  let scaleIndex = 1; // scales[1] = 100%
  let scale: number;
  $: scale = scales[scaleIndex];

  let pages = 0;
  let hash = null;
  let width: number;
  let height: number;

  const handleMouseDown = (event: MouseEvent) => {
    event.preventDefault();
    isMouseDown = true;
  };

  const handleMouseUp = (event: MouseEvent) => {
    event.preventDefault();
    isMouseDown = false;
  };

  const handleMove = (event: MouseEvent) => {
    event.preventDefault();
    if (previousEvent && isMouseDown) {
      const deltaX = previousEvent.screenX - event.screenX;
      const deltaY = previousEvent.screenY - event.screenY;

      container.scrollBy({ left: deltaX, top: deltaY });
    }
    previousEvent = event;
  };

  const handleWheel = (event: WheelEvent) => {
    if (event.ctrlKey) {
      event.preventDefault();

      if (event.deltaY < 0) { // zoom in
        scale = scales[scaleIndex = Math.min(scaleIndex + 1, scales.length - 1)];
      } else if (event.deltaY > 0) { // zoom out
        scale = scales[scaleIndex = Math.max(scaleIndex - 1, 0)];
      }
    }
  };

  onMount(() => {
    // Returns an unlisten function
    return appWindow.listen<TypstCompileEvent>("typst_compile", ({ event, payload }) => {
      const { document } = payload;
      if (document) {
        pages = document.pages;
        hash = document.hash;
        width = document.width;
        height = document.height;
      }
    });
  });
</script>

<div
  bind:this={container}
  on:mousemove={handleMove}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
  on:wheel={handleWheel}
  class={clsx("flex flex-col overflow-auto bg-neutral-700 p-4 gap-4", $$props.class)}
>
  {#each Array(pages) as _, i}
    <PreviewPage
      page={i}
      hash={hash}
      width={Math.floor(width * scale)}
      height={Math.floor(height * scale)}
      scale={scale}
    />
  {/each}
</div>
