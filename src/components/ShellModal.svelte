<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import type { Modal } from "../lib/stores";
  import { shell } from "../lib/stores";
  import CloseIcon from "./icons/CloseIcon.svelte";

  let modal: Modal | undefined;
  $: modal = $shell.modals[0];
  $: text = writable(modal?.type === "input" ? modal.initialText : "");

  const handleClose = (cancel: boolean = true) => {
    if (cancel && modal?.type === "input") {
      modal.callback(null);
    } else if (cancel && modal?.type === "confirm") {
      modal.callback(true);
    }
    shell.popModal();
  };

  const handleInputKeyUp = (event: KeyboardEvent) => {
    switch (event.key) {
      case "Enter":
        modal?.callback((event.target as HTMLInputElement).value);
        handleClose(false);
        break;
      case "Escape":
        handleClose();
        break;
    }
  };
  const handleConfirm = (event: MouseEvent) => {
    if (modal?.type === "confirm") {
      modal.callback(false);
    }
    handleClose(false);
  };
</script>

{#if modal}
  <!-- TODO: Move to dialog component? -->
  <div class="z-50 absolute inset-0 bg-black/30 flex items-center justify-center">
    <div class="bg-neutral-800 text-white rounded-md border border-neutral-600 w-80 p-4 shadow-lg">
      <div class="flex flex-row items-center mb-2">
        <span class="font-semibold block flex-1">
          {modal.title}
        </span>
        <button
          class="rounded-md border border-neutral-700 p-1 transition-colors hover:bg-neutral-700"
          on:click={handleClose}
        >
          <CloseIcon class="w-4 h-4" />
        </button>
      </div>
      {#if modal.type === "input"}
        <input
          class="w-full rounded-md bg-neutral-600 px-2 py-1 mt-2 text-sm"
          on:keyup={handleInputKeyUp}
          bind:value={$text}
          autofocus
        />
      {:else if modal.type === "confirm"}
        <div class="flex justify-between mt-3">
          <button on:click={handleClose} class="rounded-md border-2 border-neutral-700 px-2 py-1"
            >Cancel</button
          >
          <button on:click={handleConfirm} class="rounded-md px-5 py-1 bg-red-500">Ok</button>
        </div>
      {/if}
    </div>
  </div>
{/if}
