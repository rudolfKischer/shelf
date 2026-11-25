<script>
  let { shelves, activeId, onselect } = $props();
  let strip = $state();

  $effect(() => {
    strip?.querySelector(".active")?.scrollIntoView({ inline: "center", block: "nearest" });
  });
</script>

<nav class="tabs" bind:this={strip}>
  {#each shelves as shelf (shelf.id)}
    <button class="tab" class:active={shelf.id === activeId} onclick={() => onselect(shelf.id)}>
      {shelf.label}
      <span class="count">{shelf.games.length}</span>
    </button>
  {/each}
</nav>

<style>
  .tabs {
    display: flex;
    gap: 4px;
    overflow-x: auto;
    padding: 2px;
    flex: 1;
    min-width: 0;
  }
  .tab {
    flex: none;
    padding: 8px 14px;
    border-radius: 999px;
    color: var(--muted);
    font-size: 14px;
    white-space: nowrap;
    transition: background 120ms ease, color 120ms ease;
  }
  .tab:hover {
    color: var(--text);
  }
  .tab.active {
    background: var(--surface-raised);
    color: var(--text);
  }
  .count {
    margin-left: 6px;
    font-size: 12px;
    opacity: 0.6;
  }
</style>
