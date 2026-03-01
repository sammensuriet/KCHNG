<script lang="ts">
  const currentYear = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="footer-content">
    <div class="footer-brand">
      <a href="/" class="footer-logo">KCHNG</a>
      <p class="footer-tagline">Community currency for collaborative communities</p>
    </div>

    <nav class="footer-nav">
      <div class="footer-section">
        <h4>Learn</h4>
        <a href="/about">About</a>
        <a href="/faq">FAQ</a>
      </div>

      <div class="footer-section">
        <h4>Platform</h4>
        <a href="/dashboard">Dashboard</a>
        <a href="/work">Work</a>
        <a href="/trusts">Communities</a>
        <a href="/governance">Governance</a>
      </div>

      <div class="footer-section">
        <h4>Connect</h4>
        <a href="/communicate">Chat</a>
      </div>
    </nav>
  </div>

  <div class="footer-bottom">
    <p>&copy; {currentYear} KCHNG. Built for communities.</p>
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

  @media (max-width: 480px) {
    .footer-nav {
      flex-direction: column;
      align-items: center;
    }
  }
</style>
