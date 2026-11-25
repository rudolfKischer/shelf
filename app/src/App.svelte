<script>
  import { onMount } from "svelte";
  import Grid from "./lib/Grid.svelte";
  import ShelfTabs from "./lib/ShelfTabs.svelte";
  import { catalog } from "./lib/catalog.svelte.js";
  import { createInput } from "./lib/input.js";
  import settings from "./lib/settings.json";
  import strings from "./lib/strings.json";

  let grid = $state();

  const NAVIGATION_ACTIONS = new Set(["up", "down", "left", "right"]);
  const SHELF_STEPS = { prevShelf: -1, nextShelf: 1 };

  function handleAction(action) {
    if (NAVIGATION_ACTIONS.has(action)) return grid?.move(action);
    if (action in SHELF_STEPS) return catalog.stepShelf(SHELF_STEPS[action]);
    if (action === "sort") return catalog.cycleSort();
  }

  function focusTile(index) {
    catalog.focusIndex = index;
  }

  onMount(() => {
    document.documentElement.style.setProperty("--tile-min", `${settings.tileMinWidth}px`);
    document.documentElement.style.setProperty("--gap", `${settings.gridGap}px`);
    catalog.load();
    return createInput(handleAction);
  });
</script>

<div class="shell">
  <header>
    <ShelfTabs shelves={catalog.shelves} activeId={catalog.activeShelf?.id} onselect={(id) => catalog.selectShelf(id)} />
    <button class="sort" onclick={() => catalog.cycleSort()}>
      {strings.sortLabel}: {strings.sorts[catalog.sortId]}
    </button>
  </header>
  <main>
    {#if catalog.status === "loading"}
      <p class="note">{strings.loading}</p>
    {:else if catalog.status === "failed"}
      <p class="note">{catalog.error}</p>
    {:else if catalog.visibleGames.length === 0}
      <p class="note">{strings.empty}</p>
    {:else}
      <Grid
        bind:this={grid}
        games={catalog.visibleGames}
        focusIndex={catalog.focusIndex}
        onfocus={focusTile}
        onactivate={() => {}}
      />
    {/if}
  </main>
</div>

<style>
  .shell {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  header {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 24px 6px;
  }
  main {
    flex: 1;
    min-height: 0;
  }
  .sort {
    flex: none;
    padding: 8px 14px;
    border-radius: 999px;
    background: var(--surface);
    color: var(--muted);
    font-size: 13px;
  }
  .sort:hover {
    color: var(--text);
  }
  .note {
    margin: 48px;
    color: var(--muted);
  }
</style>
