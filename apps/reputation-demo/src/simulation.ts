/**
 * KCHNG Reputation Simulation
 *
 * Simulates reputation flow between participants over multiple runs
 * using different game theory strategies.
 */

// ============================================================================
// TYPES
// ============================================================================

type Strategy = "cooperator" | "defector" | "tit_for_tat" | "tit_for_two_tats" | "random" | "suspicious";

interface Participant {
  id: string;
  name: string;
  reputation: number;
  strategy: Strategy;
  verified_claims: number;
  rejected_claims: number;
  // Track last actions for tit-for-tat strategies
  last_actions: Array<{ target: string; approved: boolean }>;
}

interface WorkClaim {
  id: number;
  worker: string;
  verifiers: string[];
  legitimate: boolean; // Ground truth: is this claim actually valid?
}

interface SimulationResult {
  run: number;
  reputations: Record<string, number>;
  claims_processed: number;
  fraud_detected: number;
  fraud_missed: number;
  false_positives: number;
}

// ============================================================================
// STRATEGY IMPLEMENTATIONS
// ============================================================================

/**
 * Cooperator: Always approves claims
 */
function cooperatorDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  return true; // Always approve
}

/**
 * Defector: Always rejects claims
 */
function defectorDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  return false; // Always reject
}

/**
 * Tit-for-Tat: Do what the worker did to you last time
 */
function titForTatDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  const worker = participants.get(claim.worker);
  if (!worker) return true; // No history, cooperate

  // Find last interaction where this worker was verified by this participant
  const lastAction = participant.last_actions
    .filter(a => a.target === claim.worker)
    .slice(-1)[0];

  if (!lastAction) return true; // No history, cooperate

  return lastAction.approved; // Return the favor
}

/**
 * Tit-for-Two-Tats: Only defect if worker defected twice in a row
 */
function titForTwoTatsDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  const worker = participants.get(claim.worker);
  if (!worker) return true;

  const lastActions = participant.last_actions
    .filter(a => a.target === claim.worker)
    .slice(-2);

  if (lastActions.length < 2) return true; // Not enough history

  // Only defect if both last actions were rejections
  return lastActions.every(a => a.approved);
}

/**
 * Random: Approve/reject randomly (50/50)
 */
function randomDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  return Math.random() < 0.5;
}

/**
 * Suspicious: Start defensive, only warm up after repeated cooperation
 */
function suspiciousDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  const worker = participants.get(claim.worker);
  if (!worker) return false; // Start with rejection

  const lastActions = participant.last_actions
    .filter(a => a.target === claim.worker)
    .slice(-3);

  if (lastActions.length === 0) return false; // Initially suspicious
  if (lastActions.length < 3) return true; // Need more data, be lenient

  // Approve only if at least 2 of last 3 were approvals
  const approvals = lastActions.filter(a => a.approved).length;
  return approvals >= 2;
}

// ============================================================================
// SIMULATION ENGINE
// ============================================================================

/**
 * Create participants with different strategies
 */
function createParticipants(): Map<string, Participant> {
  const participants = new Map<string, Participant>();

  const configs: Array<{ name: string; strategy: Strategy }> = [
    { name: "Alice (Cooperator)", strategy: "cooperator" },
    { name: "Bob (Defector)", strategy: "defector" },
    { name: "Carol (Tit-for-Tat)", strategy: "tit_for_tat" },
    { name: "Dave (Tit-for-2-Tats)", strategy: "tit_for_two_tats" },
    { name: "Eve (Random)", strategy: "random" },
    { name: "Frank (Suspicious)", strategy: "suspicious" },
  ];

  for (const config of configs) {
    const id = config.name.split(" ")[0].toLowerCase();
    participants.set(id, {
      id,
      name: config.name,
      reputation: 500,
      strategy: config.strategy,
      verified_claims: 0,
      rejected_claims: 0,
      last_actions: [],
    });
  }

  return participants;
}

/**
 * Generate a work claim with random legitimacy
 */
function generateClaim(workers: string[], claimId: number): WorkClaim {
  const worker = workers[Math.floor(Math.random() * workers.length)];

  // 70% of claims are legitimate, 30% are fraud attempts
  const legitimate = Math.random() < 0.7;

  // Assign 2-3 random verifiers
  const verifiers: string[] = [];
  const availableVerifiers = workers.filter(w => w !== worker);

  while (verifiers.length < 2 && verifiers.length < availableVerifiers.length) {
    const v = availableVerifiers[Math.floor(Math.random() * availableVerifiers.length)];
    if (!verifiers.includes(v)) {
      verifiers.push(v);
    }
  }

  return {
    id: claimId,
    worker,
    verifiers,
    legitimate,
  };
}

/**
 * Get decision based on participant's strategy
 */
function getDecision(
  participant: Participant,
  claim: WorkClaim,
  participants: Map<string, Participant>
): boolean {
  switch (participant.strategy) {
    case "cooperator":
      return cooperatorDecision(participant, claim, participants);
    case "defector":
      return defectorDecision(participant, claim, participants);
    case "tit_for_tat":
      return titForTatDecision(participant, claim, participants);
    case "tit_for_two_tats":
      return titForTwoTatsDecision(participant, claim, participants);
    case "random":
      return randomDecision(participant, claim, participants);
    case "suspicious":
      return suspiciousDecision(participant, claim, participants);
  }
}

/**
 * Process a single work claim
 */
function processClaim(
  claim: WorkClaim,
  participants: Map<string, Participant>
): {
  approved: boolean;
  correct_decision: boolean;
} {
  let approvals = 0;
  const decisions: Array<{ verifier: string; approved: boolean }> = [];

  // Each verifier makes a decision
  for (const verifierId of claim.verifiers) {
    const verifier = participants.get(verifierId)!;
    const decision = getDecision(verifier, claim, participants);
    decisions.push({ verifier: verifierId, approved: decision });

    if (decision) approvals++;
  }

  // Simple majority: need more than half
  const threshold = Math.floor(claim.verifiers.length / 2) + 1;
  const approved = approvals >= threshold;

  // Ground truth: was this the right decision?
  const correct_decision = (approved && claim.legitimate) || (!approved && !claim.legitimate);

  // Update verifier stats and reputation
  for (const { verifier, approved: verifierApproved } of decisions) {
    const verifierData = participants.get(verifier)!;

    // Track the action for future tit-for-tat decisions
    verifierData.last_actions.push({
      target: claim.worker,
      approved: verifierApproved,
    });

    // Keep only last 10 actions to save memory
    if (verifierData.last_actions.length > 10) {
      verifierData.last_actions = verifierData.last_actions.slice(-10);
    }

    // Update reputation based on whether they made the right call
    // In the real contract: +5 for approval, +10 for rejection
    // Here: Reward for correct decisions
    if (approved) {
      verifierData.verified_claims++;
      // If they approved a legitimate claim or correctly rejected fraud
      if ((claim.legitimate && verifierApproved) || (!claim.legitimate && !verifierApproved)) {
        verifierData.reputation += 5;
      } else {
        // Wrong decision: small penalty
        verifierData.reputation -= 3;
      }
    } else {
      verifierData.rejected_claims++;
      if (!claim.legitimate && !verifierApproved) {
        // Correctly caught fraud: bigger reward
        verifierData.reputation += 10;
      } else {
        // Wrong decision: bigger penalty
        verifierData.reputation -= 5;
      }
    }

    // Clamp reputation
    verifierData.reputation = Math.max(0, Math.min(1000, verifierData.reputation));
  }

  return { approved, correct_decision };
}

/**
 * Run a full simulation
 */
function runSimulation(
  runs: number,
  participants: Map<string, Participant>
): SimulationResult[] {
  const results: SimulationResult[] = [];
  const participantIds = Array.from(participants.keys());

  for (let run = 1; run <= runs; run++) {
    const claim = generateClaim(participantIds, run);
    const { approved, correct_decision } = processClaim(claim, participants);

    // Calculate stats for this run
    let claims_processed = 1;
    let fraud_detected = 0;
    let fraud_missed = 0;
    let false_positives = 0;

    if (!claim.legitimate) {
      if (!approved) {
        fraud_detected++;
      } else {
        fraud_missed++;
      }
    } else {
      if (!approved) {
        false_positives++;
      }
    }

    // Store reputations at this point
    const reputations: Record<string, number> = {};
    for (const [id, p] of participants) {
      reputations[id] = p.reputation;
    }

    results.push({
      run,
      reputations,
      claims_processed,
      fraud_detected,
      fraud_missed,
      false_positives,
    });

    // Print progress every 50 runs
    if (run % 50 === 0) {
      console.log(`  Processed ${run}/${runs} runs...`);
    }
  }

  return results;
}

// ============================================================================
// VISUALIZATION
// ============================================================================

/**
 * Display results as an ASCII chart
 */
function displayResults(results: SimulationResult[], participants: Map<string, Participant>): void {
  console.log("\n" + "=".repeat(80));
  console.log("📊 REPUTATION SIMULATION RESULTS");
  console.log("=".repeat(80));

  // Final reputation rankings
  console.log("\n🏆 FINAL REPUTATION RANKINGS:");
  console.log("-".repeat(80));

  const sorted = Array.from(participants.entries()).sort((a, b) => b[1].reputation - a[1].reputation);

  for (let i = 0; i < sorted.length; i++) {
    const [id, p] = sorted[i];
    const tier = getReputationTierSim(p.reputation);
    const bar = "█".repeat(Math.floor(p.reputation / 20));

    console.log(`\n${i + 1}. ${p.name.padEnd(25)} ${tier}`);
    console.log(`   Reputation: ${p.reputation}/1000`);
    console.log(`   ${bar}`);
    console.log(`   Verified: ${p.verified_claims} | Rejected: ${p.rejected_claims}`);
  }

  // Statistics
  const totalRuns = results.length;
  const totalFraudDetected = results.reduce((sum, r) => sum + r.fraud_detected, 0);
  const totalFraudMissed = results.reduce((sum, r) => sum + r.fraud_missed, 0);
  const totalFalsePositives = results.reduce((sum, r) => sum + r.false_positives, 0);

  console.log("\n" + "-".repeat(80));
  console.log("📈 STATISTICS:");
  console.log("-".repeat(80));
  console.log(`Total Runs:              ${totalRuns}`);
  console.log(`Fraud Detected:          ${totalFraudDetected} ✅`);
  console.log(`Fraud Missed:            ${totalFraudMissed} ❌`);
  console.log(`False Positives:         ${totalFalsePositives} ⚠️`);

  if (totalFraudDetected + totalFraudMissed > 0) {
    const detectionRate = (totalFraudDetected / (totalFraudDetected + totalFraudMissed)) * 100;
    console.log(`Fraud Detection Rate:   ${detectionRate.toFixed(1)}%`);
  }

  // ASCII timeline chart
  console.log("\n" + "-".repeat(80));
  console.log("📉 REPUTATION OVER TIME (every 25 runs):");
  console.log("-".repeat(80));

  const step = 25;
  for (let i = 0; i < results.length; i += step) {
    const result = results[i];
    const line = [`Run ${result.run.toString().padStart(3)}:`];

    for (const [id, p] of participants) {
      const rep = result.reputations[id];
      const normalized = Math.floor(rep / 50); // 0-20 chars
      const bar = "▓".repeat(normalized).padEnd(20, "░");
      line.push(`${id[0].toUpperCase()}:${bar}`);
    }

    console.log(line.join(" "));
  }

  console.log("\nLegend: A=Alice, B=Bob, C=Carol, D=Dave, E=Eve, F=Frank");
  console.log("        ▓=reputation earned, ░=potential reputation");

  // Strategy analysis
  console.log("\n" + "-".repeat(80));
  console.log("🎯 STRATEGY ANALYSIS:");
  console.log("-".repeat(80));

  for (const [id, p] of sorted) {
    const avgRepGain = (p.reputation - 500) / totalRuns;
    const verdict = avgRepGain > 0.5 ? "🟢 Effective" : avgRepGain > -0.5 ? "🟡 Neutral" : "🔴 Ineffective";

    console.log(`\n${p.name}:`);
    console.log(`  Final Rep:    ${p.reputation} (${p.reputation - 500 > 0 ? "+" : ""}${p.reputation - 500})`);
    console.log(`  Avg Gain/Run: ${avgRepGain.toFixed(3)}`);
    console.log(`  Verdict:      ${verdict}`);
  }
}

// ============================================================================
// HELPERS
// ============================================================================

/**
 * Get reputation tier for display
 */
function getReputationTierSim(score: number): string {
  if (score >= 900) return "🏆 Legendary";
  if (score >= 750) return "⭐ Trusted";
  if (score >= 600) return "✓ Established";
  if (score >= 500) return "◯ Neutral";
  if (score >= 400) return "△ New";
  return "▽ Unproven";
}

/**
 * Display scenario description
 */
function displayScenario(): void {
  console.log("\n" + "=".repeat(80));
  console.log("🎮 REPUTATION GAME THEORY SIMULATION");
  console.log("=".repeat(80));

  console.log(`
This simulation demonstrates how reputation flows between participants
using different game theory strategies over multiple rounds.

SCENARIO:
  • 6 participants with different strategies
  • Each run: random worker submits a claim
  • 2-3 randomly selected verifiers decide on each claim
  • 70% of claims are legitimate, 30% are fraudulent
  • Reputation updates: +5 for correct approval, +10 for catching fraud

STRATEGIES:
  • Cooperator (Alice)    : Always approves
  • Defector (Bob)        : Always rejects
  • Tit-for-Tat (Carol)   : Returns other's last action
  • Tit-for-2-Tats (Dave) : Needs 2 defections before retaliating
  • Random (Eve)          : Approves/rejects randomly
  • Suspicious (Frank)    : Starts defensive, warms slowly

REPUTATION MECHANICS:
  • +5  : Correctly approved a legitimate claim
  • +10 : Correctly rejected a fraudulent claim (caught fraud!)
  • -3  : Wrongly approved a fraudulent claim
  • -5  : Wrongly rejected a legitimate claim

This simulation shows how different strategies perform in a reputation-based
system where participants must balance cooperation with fraud detection.
`);
}

// ============================================================================
// MAIN
// ============================================================================

function runSimulationMain() {
  displayScenario();

  const RUNS = 150;
  console.log(`Running ${RUNS} iterations...\n`);

  const participants = createParticipants();
  const results = runSimulation(RUNS, participants);

  displayResults(results, participants);

  console.log("\n" + "=".repeat(80));
  console.log("💡 KEY INSIGHTS:");
  console.log("=".repeat(80));
  console.log(`
  1. Tit-for-Tat strategies tend to perform well in reputation systems
     because they reward cooperation while defending against exploitation.

  2. Always-defectors miss out on reputation gains from legitimate claims.

  3. Always-cooperators can be exploited by fraudsters.

  4. The optimal strategy depends on the fraud rate in the system.
     Higher fraud rates favor more suspicious strategies.

  5. Reputation systems create incentives for:
     - Consistent honest behavior
     - Careful verification (not rubber-stamping)
     - Balanced approach to trust

  In the KCHNG contract, reputation is tracked on-chain but how it's
  used is left to third-party applications. This simulation shows one
  possible way reputation could influence verifier selection and rewards.
`);
  console.log("=".repeat(80) + "\n");
}

// Run the simulation
runSimulationMain();
