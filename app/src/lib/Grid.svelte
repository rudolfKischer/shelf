<script>
  import GameTile from "./GameTile.svelte";

  let { games, focusIndex, onfocus, onactivate } = $props();
  let grid = $state();

  const MOVES = { left: -1, right: 1 };

  function columnCount() {
    if (!grid) return 1;
    return getComputedStyle(grid).gridTemplateColumns.split(" ").length;
  }

  export function move(action) {
    const columns = columnCount();
    const delta = MOVES[action] ?? (action === "up" ? -columns : action === "down" ? columns : 0);
    const next = Math.min(Math.max(focusIndex + delta, 0), games.length - 1);
    onfocus(next);
  }

  $effect(() => {
    grid?.children[focusIndex]?.scrollIntoView({ block: "nearest" });
  });
</script>

<div class="scroll">
  <div class="grid" bind:this={grid}>
    {#each games as game, index (game.app_id)}
      <GameTile
        {game}
        focused={index === focusIndex}
        onhover={() => onfocus(index, false)}
        onactivate={() => onactivate(game)}
      />
    {/each}
  </div>
</div>

<style>
  .scroll {
    height: 100%;
    overflow-y: auto;
    padding: 18px 28px 40px;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--tile-min), 1fr));
    gap: var(--gap);
  }
</style>
