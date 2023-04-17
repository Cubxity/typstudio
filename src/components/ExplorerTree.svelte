<script lang="ts">

  import clsx from "clsx";
  import ExplorerNode from "./ExplorerNode.svelte";
</script>


<div class={clsx("border-r border-neutral-700 p-3 select-none", $$props.class)}>
  <ExplorerNode type="directory" path="" />
</div>
