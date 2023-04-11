<script lang="ts">

  import clsx from "clsx";
  import PreviewPage from "./PreviewPage.svelte";
  import { onMount } from "svelte";
  import type { TypstCompileEvent } from "../lib/ipc";
  import { appWindow } from "@tauri-apps/api/window";

  let container: HTMLDivElement;
  let previousEvent: MouseEvent | undefined;

  let isMouseDown = false;

  let pages = 0;
  let hash = null;

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

  onMount(async () => {


    // Returns an unlisten function
    return appWindow.listen<TypstCompileEvent>("typst_compile", ({ event, payload }) => {
      pages = payload.pages;
      hash = payload.hash;
    });
  });
</script>

<div
  bind:this={container}
  on:mousemove={handleMove}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
  class={clsx("flex flex-col items-center overflow-auto bg-neutral-700 p-4 gap-4", $$props.class)}
>
  {#each Array(pages) as _, i}
    <PreviewPage page={i} hash={hash} />
  {/each}
</div>
