<script>
  import Icon from "./Icon.svelte";
  import { artUrl } from "./api.js";
  import { glyphsFor, installGlyph } from "./glyphs.js";
  import strings from "./strings.json";

  let { game, focused, onhover, onactivate } = $props();
  let failedSources = $state(0);

  const sources = $derived(
    [game.art.capsule.local && artUrl({ local: game.art.capsule.local }), game.art.capsule.remote].filter(Boolean),
  );
  const source = $derived(sources[failedSources] ?? null);
  const status = $derived(installGlyph(game));
</script>

<button
  class="tile"
  class:focused
  class:uninstalled={game.copies.length === 0}
  title={game.name}
  onpointerenter={onhover}
  onclick={onactivate}
>
  {#if source}
    <img src={source} alt="" loading="lazy" draggable="false" onerror={() => failedSources++} />
  {:else}
    <span class="fallback">{game.name}</span>
  {/if}
  <span class="status" class:on={status === "installed"} title={strings.glyphs[status]}>
    <Icon name={status} size={14} />
  </span>
  <span class="caption">
    <span class="name">{game.name}</span>
    <span class="glyphs">
      {#each glyphsFor(game) as glyph}
        <span title={strings.glyphs[glyph]}><Icon name={glyph} size={15} /></span>
      {/each}
    </span>
  </span>
</button>

<style>
  .tile {
    position: relative;
    display: block;
    width: 100%;
    aspect-ratio: 2 / 3;
    padding: 0;
    overflow: hidden;
    border-radius: var(--radius);
    background: var(--surface);
    content-visibility: auto;
    contain-intrinsic-size: auto 252px;
    transition: transform 120ms ease, box-shadow 120ms ease;
    outline: none;
  }
  .tile.uninstalled img {
    filter: saturate(0.55) brightness(0.8);
  }
  .tile.focused {
    transform: scale(1.04);
    box-shadow: 0 0 0 var(--focus-ring) var(--accent), 0 12px 28px rgba(0, 0, 0, 0.5);
    z-index: 2;
  }
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
  .fallback {
    display: grid;
    place-items: center;
    height: 100%;
    padding: 12px;
    text-align: center;
    color: var(--muted);
    font-size: 14px;
  }
  .status {
    position: absolute;
    top: 8px;
    right: 8px;
    display: grid;
    place-items: center;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: rgba(14, 16, 21, 0.72);
    color: var(--muted);
  }
  .status.on {
    color: var(--accent);
  }
  .caption {
    position: absolute;
    inset: auto 0 0 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 28px 10px 10px;
    background: linear-gradient(transparent, rgba(8, 9, 12, 0.92));
    opacity: 0;
    transition: opacity 120ms ease;
    text-align: left;
  }
  .tile.focused .caption,
  .tile:hover .caption {
    opacity: 1;
  }
  .name {
    font-size: 13px;
    font-weight: 600;
    line-height: 1.25;
  }
  .glyphs {
    display: flex;
    gap: 8px;
    color: var(--muted);
  }
</style>
