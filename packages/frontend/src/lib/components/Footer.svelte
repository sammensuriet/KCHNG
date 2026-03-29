<script lang="ts">
  import { t } from "$lib/i18n";
  const currentYear = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="footer-content">
    <div class="footer-brand">
      <a href="/" class="footer-logo">KCHNG</a>
      <p class="footer-tagline">{t('footer.tagline')}</p>
    </div>

    <nav class="footer-nav">
      <div class="footer-section">
        <h4>{t('footer.learn')}</h4>
        <a href="/about">{t('nav.about')}</a>
      </div>

      <div class="footer-section">
        <h4>{t('footer.platform')}</h4>
        <a href="/dashboard">{t('nav.dashboard')}</a>
        <a href="/work">{t('nav.work')}</a>
        <a href="/communities">{t('nav.communities')}</a>
        <a href="/governance">{t('nav.governance')}</a>
      </div>

      <div class="footer-section">
        <h4>{t('footer.connect')}</h4>
        <a href="/communicate">{t('nav.chat')}</a>
      </div>
    </nav>
  </div>

  <div class="footer-bottom">
    <p>&copy; {currentYear} KCHNG. {t('footer.builtFor')}</p>
  </div>
</footer>

<style>
  .footer {
    background: #f9fafb;
    border-top: 1px solid #e5e7eb;
    margin-top: auto;
    padding: 2rem 1rem;
    box-sizing: border-box;
  }

  .footer-content {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: flex-start;
    gap: 2rem;
    max-width: 1200px;
    margin: 0 auto;
    box-sizing: border-box;
  }

  .footer-brand {
    flex-shrink: 0;
    max-width: 280px;
  }

  .footer-logo {
    font-size: 1.25rem;
    font-weight: 700;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-decoration: none;
  }

  .footer-tagline {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: #6b7280;
    line-height: 1.5;
  }

  .footer-nav {
    display: flex;
    flex-direction: row;
    gap: 3rem;
  }

  .footer-section h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.75rem;
    font-weight: 600;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .footer-section a {
    display: block;
    padding: 0.25rem 0;
    color: #374151;
    text-decoration: none;
    font-size: 0.875rem;
    transition: color 0.2s;
  }

  .footer-section a:hover {
    color: #667eea;
  }

  .footer-bottom {
    max-width: 1200px;
    margin: 1.5rem auto 0;
    padding-top: 1rem;
    border-top: 1px solid #e5e7eb;
    text-align: center;
  }

  .footer-bottom p {
    margin: 0;
    font-size: 0.875rem;
    color: #9ca3af;
  }

  @media (max-width: 768px) {
    .footer-content {
      flex-direction: column;
      gap: 1.5rem;
    }

    .footer-brand {
      max-width: 100%;
      text-align: center;
    }

    .footer-nav {
      justify-content: center;
      flex-wrap: wrap;
      gap: 2rem;
    }

    .footer-section {
      text-align: center;
      min-width: 100px;
    }
  }

  @media (max-width: 640px) {
    /* Simplify footer on mobile - navigation is in bottom nav bar */
    .footer {
      padding: 1rem;
    }

    .footer-nav {
      display: none;
    }

    .footer-content {
      justify-content: center;
    }

    .footer-brand {
      text-align: center;
      max-width: 100%;
    }

    .footer-bottom {
      margin-top: 0;
      padding-top: 0.5rem;
      border-top: none;
    }
  }

  @media (max-width: 480px) {
    .footer-nav {
      flex-direction: column;
      align-items: center;
    }
  }
</style>
