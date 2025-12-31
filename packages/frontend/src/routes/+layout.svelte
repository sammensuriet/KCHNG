<script lang="ts">
  import "../app.css";
  import Header from "$lib/components/Header.svelte";
</script>

<svelte:head>
  <title>KCHNG - Community Currency</title>
  <meta
    name="description"
    content="A Stellar blockchain community currency with demurrage"
  />
</svelte:head>

<Header />

<main class="container">
  <slot />
</main>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }
</style>
