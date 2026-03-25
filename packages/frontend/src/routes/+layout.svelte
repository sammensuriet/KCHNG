<script lang="ts">
  import "../app.css";
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import MobileNav from "$lib/components/MobileNav.svelte";
  import { initFromServer, currentLanguage, type Language, type Messages } from "$lib/i18n";

  // Receive server-loaded data
  let { data }: { data: { initialLanguage: Language; initialMessages: Messages } } = $props();

  // Initialize i18n immediately with server data (for SSR hydration)
  initFromServer(data.initialLanguage, data.initialMessages);

  // Subscribe to language changes
  $effect(() => {
    const unsubscribe = currentLanguage.subscribe((lang) => {
      // Update document lang attribute when language changes
      if (typeof document !== 'undefined') {
        document.documentElement.lang = lang;
      }
    });
    return unsubscribe;
  });
</script>

<div class="page-wrapper">
  <Header />
  <main class="container">
    <slot />
  </main>
  <Footer />
  <MobileNav />
</div>

<style>
  .page-wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    overflow-x: hidden;
  }
</style>
