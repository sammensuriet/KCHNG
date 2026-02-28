<svelte:head>
  <title>About KCHNG - Community Work Exchange</title>
  <meta name="description" content="Learn about KCHNG, a community work exchange system for eco-villages, cooperatives, and community spaces." />
</svelte:head>

<div class="page-header">
  <h1>About KCHNG</h1>
  <p class="subtitle">Community currency for collaborative communities</p>
</div>

<section class="section">
  <h2>What is KCHNG?</h2>
  <p>
    KCHNG is a <strong>community work exchange system</strong> — a way for communities to track
    and reward the work that keeps them running. Instead of money that comes from outside,
    KCHNG is created by the community itself, through actual work done by its members.
  </p>
  <p>
    The core idea is simple: <strong>30 minutes of verified work = 1,000 KCHNG = 1 shared meal</strong>.
    This creates a stable, understandable value that's tied to real effort, not market speculation.
  </p>
</section>

<section class="section">
  <h2>Who Is It For?</h2>
  <p>KCHNG is designed for communities that:</p>
  <ul>
    <li><strong>Eco-villages</strong> — Track contributions to shared infrastructure, gardening, maintenance</li>
    <li><strong>Cooperatives</strong> — Reward member participation beyond formal employment</li>
    <li><strong>Community spaces</strong> — Shared venues, event spaces, gathering places</li>
    <li><strong>Housing collectives</strong> — Manage shared chores and responsibilities</li>
    <li><strong>Any group</strong> with shared work and a need to track contributions fairly</li>
  </ul>
  <div class="highlight">
    <p>
      <strong>Key requirement:</strong> The community should be small enough that members
      can know and trust each other. KCHNG works through peer validation, not algorithms.
    </p>
  </div>
</section>

<section class="section">
  <h2>How Does It Work?</h2>
  <div class="steps">
    <div class="step">
      <div class="step-num">1</div>
      <div class="step-content">
        <h3>Someone Does Work</h3>
        <p>Cooking communal meals, fixing things, organizing events, teaching workshops, cleaning, gardening — anything the community values.</p>
      </div>
    </div>
    <div class="step">
      <div class="step-num">2</div>
      <div class="step-content">
        <h3>Community Validates</h3>
        <p>Other members confirm the work happened. This isn't bureaucratic — it's just people vouching for each other. Usually 2 verifiers are enough.</p>
      </div>
    </div>
    <div class="step">
      <div class="step-num">3</div>
      <div class="step-content">
        <h3>Tokens Are Created</h3>
        <p>New KCHNG is minted directly to the worker's wallet. This isn't transferred from somewhere else — it's newly created value, backed by real work.</p>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <h2>Why Time-Based Value?</h2>
  <p>
    Unlike regular money, KCHNG's value is tied to <strong>actual human effort</strong>, not
    what the market decides. One hour of work is worth the same whether you're cooking,
    cleaning, or teaching.
  </p>
  <p>
    This creates fairness. The person scrubbing floors isn't valued less than the person
    running a workshop. All work that keeps the community going is equally valuable.
  </p>
  <div class="work-types">
    <h4>Work Types and Multipliers</h4>
    <p>While all work is valued, some work carries more responsibility or urgency:</p>
    <div class="multiplier-grid">
      <div class="multiplier">
        <span class="mult-value">1.0×</span>
        <span class="mult-label">Standard Work</span>
        <span class="mult-examples">Cooking, cleaning, routine tasks</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">1.3×</span>
        <span class="mult-label">Skilled Work</span>
        <span class="mult-examples">Technical repairs, specialized services</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">1.5×</span>
        <span class="mult-label">Teaching</span>
        <span class="mult-examples">Workshops, training, mentorship</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">2.0×</span>
        <span class="mult-label">Emergency</span>
        <span class="mult-examples">Urgent repairs, crisis response</span>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <h2>Why Do Tokens Decrease When Unused?</h2>
  <p>
    This is the key difference between KCHNG and regular money. KCHNG is designed to
    <strong>circulate</strong>, not accumulate.
  </p>
  <p>
    When tokens sit idle in someone's wallet for too long, they gradually decrease in value.
    This isn't a punishment — it's how the system stays healthy:
  </p>
  <ul>
    <li><strong>Encourages spending</strong> — Use your KCHNG for meals, services, or trade</li>
    <li><strong>Prevents hoarding</strong> — Wealth can't just sit and grow automatically</li>
    <li><strong>Keeps value moving</strong> — Active currency means more community interaction</li>
  </ul>
  <p>
    <small>Practical example: If you don't use your balance for a month, it might decrease by about 1%. Stay active and it stays full.</small>
  </p>
</section>

<section class="section">
  <h2>Historical Context</h2>
  <p>
    The idea of "active currency" isn't new. It's been tested and proven in real communities:
  </p>

  <div class="history-table">
    <table>
      <thead>
        <tr>
          <th>System</th>
          <th>When</th>
          <th>What Happened</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>Wörgl, Austria</strong></td>
          <td>1932-1933</td>
          <td>A small town issued "work certificates" that lost value if held. Unemployment dropped dramatically, local trade flourished. The Austrian central bank eventually banned it.</td>
        </tr>
        <tr>
          <td><strong>Chiemgauer, Germany</strong></td>
          <td>2003-present</td>
          <td>A regional currency with a similar "use it or lose it" design. Still running after 20+ years, with money circulating about 3x faster than regular currency.</td>
        </tr>
        <tr>
          <td><strong>KCHNG</strong></td>
          <td>2026-</td>
          <td>Bringing this model to communities everywhere, using blockchain technology for transparency and trust.</td>
        </tr>
      </tbody>
    </table>
  </div>

  <blockquote>
    <p>
      "The Wörgl experiment demonstrated that active currency achieved a 12× velocity
      multiplier compared to conventional money — money changed hands 12 times more often,
      stimulating local trade and reducing unemployment."
    </p>
    <cite>— Historical data from the Wörgl Freigeld experiment (1932-1933)</cite>
  </blockquote>
</section>

<section class="section">
  <h2>The Technology</h2>
  <p>
    KCHNG runs on <strong>Stellar</strong>, a public blockchain. This means:
  </p>
  <ul>
    <li><strong>Transparent</strong> — All transactions are public and verifiable</li>
    <li><strong>No central authority</strong> — No single entity controls the currency</li>
    <li><strong>Low fees</strong> — Transaction costs are minimal</li>
    <li><strong>Accessible</strong> — Works anywhere with internet access</li>
  </ul>
  <p>
    You don't need to understand blockchain to use KCHNG. Just connect a wallet and start
    participating in your community's economy.
  </p>
</section>

<section class="section cta-section">
  <h2>Questions?</h2>
  <p>
    <a href="/communicate" class="cta-button">Chat with us →</a>
  </p>
  <p class="cta-note">
    Or check out the <a href="/faq">Frequently Asked Questions</a>
  </p>
</section>

<style>
  .page-header {
    text-align: center;
    padding: var(--space-xl) 0;
    border-bottom: 1px solid var(--color-border);
    margin-bottom: var(--space-xl);
  }

  .page-header h1 {
    font-size: var(--font-size-3xl);
    font-weight: 800;
    margin: 0;
    background: var(--color-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: var(--font-size-lg);
    color: var(--color-text-muted);
    margin-top: var(--space-sm);
  }

  .section {
    margin: var(--space-xl) 0;
    max-width: 700px;
  }

  .section h2 {
    color: var(--color-text);
    font-size: var(--font-size-xl);
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-sm);
    border-bottom: 2px solid var(--color-primary);
    display: inline-block;
  }

  .section p {
    line-height: 1.7;
    color: var(--color-text);
  }

  .section ul {
    line-height: 1.8;
    color: var(--color-text);
  }

  .section li {
    margin-bottom: var(--space-sm);
  }

  .highlight {
    background: var(--color-bg-subtle);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    margin-top: var(--space-md);
    border-left: 4px solid var(--color-primary);
  }

  .highlight p {
    margin: 0;
  }

  /* Steps */
  .steps {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .step {
    display: flex;
    gap: var(--space-lg);
    align-items: flex-start;
  }

  .step-num {
    width: 48px;
    height: 48px;
    background: var(--color-gradient);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-xl);
    font-weight: 700;
    flex-shrink: 0;
  }

  .step-content h3 {
    margin: 0 0 var(--space-xs) 0;
    font-size: var(--font-size-lg);
    color: var(--color-text);
  }

  .step-content p {
    margin: 0;
    color: var(--color-text-muted);
  }

  /* Work Types */
  .work-types {
    margin-top: var(--space-lg);
    padding: var(--space-lg);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
  }

  .work-types h4 {
    margin: 0 0 var(--space-sm) 0;
    font-size: var(--font-size-base);
    color: var(--color-text);
  }

  .work-types p {
    margin: 0 0 var(--space-md) 0;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .multiplier-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-md);
  }

  .multiplier {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-sm);
    background: white;
    border-radius: var(--radius-sm);
  }

  .mult-value {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--color-primary);
  }

  .mult-label {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text);
    margin-top: var(--space-xs);
  }

  .mult-examples {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 2px;
  }

  /* History Table */
  .history-table {
    overflow-x: auto;
    margin: var(--space-lg) 0;
  }

  .history-table table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }

  .history-table th,
  .history-table td {
    padding: var(--space-sm) var(--space-md);
    text-align: left;
    border-bottom: 1px solid var(--color-border);
  }

  .history-table th {
    font-weight: 600;
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .history-table td {
    color: var(--color-text);
    vertical-align: top;
  }

  blockquote {
    margin: var(--space-lg) 0;
    padding: var(--space-md);
    background: white;
    border-left: 4px solid var(--color-primary);
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  }

  blockquote p {
    margin: 0 0 var(--space-sm) 0;
    font-style: italic;
    color: var(--color-text);
    line-height: 1.6;
  }

  blockquote cite {
    display: block;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: normal;
  }

  /* CTA */
  .cta-section {
    text-align: center;
    padding: var(--space-xl);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-lg);
  }

  .cta-button {
    display: inline-block;
    padding: var(--space-md) var(--space-xl);
    background: var(--color-gradient);
    color: white;
    text-decoration: none;
    border-radius: var(--radius-md);
    font-weight: 600;
    transition: opacity 0.2s;
  }

  .cta-button:hover {
    opacity: 0.9;
  }

  .cta-note {
    margin-top: var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .cta-note a {
    color: var(--color-primary);
  }

  @media (max-width: 768px) {
    .step {
      flex-direction: column;
      align-items: center;
      text-align: center;
    }

    .multiplier-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
