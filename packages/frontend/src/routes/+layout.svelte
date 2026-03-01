<script lang="ts">
  import "../app.css";
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
</script>

<svelte:head>
  <title>KCHNG - Community Work Exchange</title>
  <meta
    name="description"
    content="Community currency for collaborative communities. Track and reward the work that keeps your community running."
  />
</svelte:head>

<div class="page-wrapper">
  <Header />

  <main class="container">
    <slot />
  </main>

  <Footer />
</div>

<style>
  .page-wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    overflow-x: hidden;
  }

  .container {
    flex: 1;
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
    width: 100%;
    box-sizing: border-box;
  }

  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }
  }
</style>
