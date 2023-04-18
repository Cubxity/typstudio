<script lang="ts">
  import type { Modal } from "../lib/stores";
  import { shell } from "../lib/stores";
  import CloseIcon from "./icons/CloseIcon.svelte";

  let modal: Modal | undefined;
  $: modal = $shell.modals[0];

  const handleClose = (cancel: boolean = true) => {
    if (cancel && modal?.type === "input") {
      modal.callback(null);
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
          autofocus
        >
      {/if}
    </div>
  </div>
{/if}
