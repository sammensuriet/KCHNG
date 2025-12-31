<script lang="ts">
  import "../app.css";
</script>

<svelte:head>
  <title>KCHNG - Community Currency</title>
  <meta
    name="description"
    content="A Stellar blockchain community currency with demurrage"
  />
</svelte:head>

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
