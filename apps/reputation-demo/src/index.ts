/**
 * KCHNG Reputation System - Third-Party Demo App
 *
 * This demonstrates how third-party applications can use the KCHNG contract's
 * reputation system to implement custom logic without modifying the core contract.
 *
 * The contract tracks reputation scores (0-1000, starting at 500):
 * - +5 points for each claim approved
 * - +10 points for each claim rejected (higher incentive for fraud detection)
 *
 * Third-party apps can read this data and implement their own features:
 * - Reputation leaderboards
 * - Verifier recommendation systems
 * - Reputation-weighted UI/UX
 * - External incentive programs
 */

// ============================================================================
// TYPES (mirroring the contract structures)
// ============================================================================

interface VerifierData {
  trust_id: string | null;
  stake: bigint;
  reputation_score: number;        // 0-1000
  verified_claims: number;
  rejected_claims: number;
  fraud_reports: number;
}

interface VerifierWithAddress {
  address: string;
  data: VerifierData;
}

// ============================================================================
// REPUTATION ANALYSIS FUNCTIONS (Third-Party Logic)
// ============================================================================

/**
 * Sort verifiers by reputation score (highest first)
 */
function sortByReputation(verifiers: VerifierWithAddress[]): VerifierWithAddress[] {
  return [...verifiers].sort((a, b) => b.data.reputation_score - a.data.reputation_score);
}

/**
 * Calculate reputation tier for display purposes
 */
function getReputationTier(score: number): string {
  if (score >= 900) return "🏆 Legendary";
  if (score >= 750) return "⭐ Trusted";
  if (score >= 600) return "✓ Established";
  if (score >= 500) return "◯ Neutral";
  if (score >= 400) return "△ New";
  return "▽ Unproven";
}

/**
 * Calculate a trust score based on multiple factors
 * This is custom third-party logic, not enforced by contract
 */
function calculateTrustScore(verifier: VerifierData): number {
  // Base: reputation score (0-1000)
  let score = verifier.reputation_score;

  // Bonus: High verification activity (indicates experience)
  const totalClaims = verifier.verified_claims + verifier.rejected_claims;
  if (totalClaims > 50) score += 50;
  else if (totalClaims > 20) score += 25;
  else if (totalClaims > 10) score += 10;

  // Bonus: Balanced approval/rejection ratio (indicates thoughtful review)
  if (totalClaims > 5) {
    const ratio = verifier.rejected_claims / totalClaims;
    // Optimal rejection rate is 5-15% (catching fraud but not being overly harsh)
    if (ratio >= 0.05 && ratio <= 0.15) score += 30;
  }

  // Penalty: Zero rejections with many approvals (possible rubber-stamping)
  if (verifier.verified_claims > 20 && verifier.rejected_claims === 0) {
    score -= 50;
  }

  return Math.max(0, Math.min(1000, score));
}

/**
 * Generate a verifier recommendation for a work claim
 * Third-party apps can implement custom selection algorithms
 */
function recommendVerifiers(
  allVerifiers: VerifierWithAddress[],
  count: number = 3
): Array<VerifierWithAddress & { reason: string }> {
  const sorted = sortByReputation(allVerifiers);
  const recommendations: Array<VerifierWithAddress & { reason: string }> = [];

  for (const verifier of sorted.slice(0, count)) {
    const tier = getReputationTier(verifier.data.reputation_score);
    const trustScore = calculateTrustScore(verifier.data);

    let reason = `${tier} (Reputation: ${verifier.data.reputation_score}, Trust Score: ${trustScore})`;

    // Add custom reasons based on verifier history
    if (verifier.data.rejected_claims > verifier.data.verified_claims * 0.1) {
      reason += " - Active fraud detector";
    }
    if (verifier.data.verified_claims > 50) {
      reason += " - Highly experienced";
    }

    recommendations.push({ ...verifier, reason });
  }

  return recommendations;
}

// ============================================================================
// DEMO DATA (Simulating contract queries)
// ============================================================================

/**
 * Example verifier addresses (testnet format)
 */
const EXAMPLE_VERIFIERS: VerifierWithAddress[] = [
  {
    address: "GCW4XHQLIK3VHXUGNXJ4NFLW5JDWZZ3UT2XJCYTZSO6AQJZUUDB7RVMS",
    data: {
      trust_id: "CDLY7NZNVDKJMHBSVDIQZCMKPKEJL7LFHLQNXYZIA2N5UTFGFFRYQCU7",
      stake: 100000n,
      reputation_score: 740,
      verified_claims: 55,
      rejected_claims: 6,
      fraud_reports: 0,
    },
  },
  {
    address: "GAM6N54Y5SBFUDV2YRLZB45CBPMAEJO5KACRSAR35F37GYDGGUGNNDK2",
    data: {
      trust_id: "CDLY7NZNVDKJMHBSVDIQZCMKPKEJL7LFHLQNXYZIA2N5UTFGFFRYQCU7",
      stake: 100000n,
      reputation_score: 660,
      verified_claims: 40,
      rejected_claims: 4,
      fraud_reports: 0,
    },
  },
  {
    address: "GBQ2KJ7AEY3M2AGK7RCPICGYGM7NB7JUZL5XKSHGNFPPXB4V7PNHZXG2",
    data: {
      trust_id: "CDLY7NZNVDKJMHBSVDIQZCMKPKEJL7LFHLQNXYZIA2N5UTFGFFRYQCU7",
      stake: 100000n,
      reputation_score: 820,
      verified_claims: 78,
      rejected_claims: 9,
      fraud_reports: 1,
    },
  },
  {
    address: "GD7Y7NGJLX7LJ7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7D7",
    data: {
      trust_id: null,
      stake: 100000n,
      reputation_score: 500,
      verified_claims: 5,
      rejected_claims: 0,
      fraud_reports: 0,
    },
  },
];

// ============================================================================
// DEMO DISPLAYS
// ============================================================================

/**
 * Display a reputation leaderboard
 */
function displayLeaderboard(verifiers: VerifierWithAddress[]): void {
  console.log("\n" + "=".repeat(60));
  console.log("📊 REPUTATION LEADERBOARD");
  console.log("=".repeat(60));

  const sorted = sortByReputation(verifiers);

  for (let i = 0; i < sorted.length; i++) {
    const { address, data } = sorted[i];
    const tier = getReputationTier(data.reputation_score);
    const trustScore = calculateTrustScore(data);

    console.log(`\n#${i + 1} ${address.slice(0, 12)}...${address.slice(-4)}`);
    console.log(`   Reputation: ${data.reputation_score}/1000`);
    console.log(`   Tier: ${tier}`);
    console.log(`   Trust Score: ${trustScore}/1000`);
    console.log(`   Verified: ${data.verified_claims} | Rejected: ${data.rejected_claims}`);
  }
}

/**
 * Display verifier recommendations for a work claim
 */
function displayRecommendations(recommendations: Array<VerifierWithAddress & { reason: string }>): void {
  console.log("\n" + "=".repeat(60));
  console.log("🎯 RECOMMENDED VERIFIERS FOR YOUR WORK CLAIM");
  console.log("=".repeat(60));

  for (let i = 0; i < recommendations.length; i++) {
    const rec = recommendations[i];
    console.log(`\n${i + 1}. ${rec.address.slice(0, 12)}...${rec.address.slice(-4)}`);
    console.log(`   ${rec.reason}`);
  }

  console.log("\n💡 Tip: This ranking uses custom third-party logic!");
  console.log("   The contract only stores the raw reputation score.");
  console.log("   Apps can implement any selection algorithm they want.");
}

/**
 * Display custom reputation tiers
 */
function displayTierAnalysis(verifiers: VerifierWithAddress[]): void {
  console.log("\n" + "=".repeat(60));
  console.log("📈 REPUTATION TIER ANALYSIS");
  console.log("=".repeat(60));

  const tiers: Record<string, number> = {};

  for (const verifier of verifiers) {
    const tier = getReputationTier(verifier.data.reputation_score);
    tiers[tier] = (tiers[tier] || 0) + 1;
  }

  console.log("");
  for (const [tier, count] of Object.entries(tiers)) {
    const bar = "█".repeat(count);
    console.log(`${tier.padEnd(20)} ${bar} (${count})`);
  }
}

/**
 * Display contract integration example
 */
function displayContractIntegration(): void {
  console.log("\n" + "=".repeat(60));
  console.log("🔗 CONTRACT INTEGRATION EXAMPLE");
  console.log("=".repeat(60));

  console.log(`
To query reputation from the KCHNG contract:

Contract Method: get_verifier(verifier: Address) -> VerifierData

Returns:
{
  trust_id: Address | null,
  stake: Amount,
  reputation_score: number,    // 0-1000 ← USE THIS FOR YOUR LOGIC
  verified_claims: number,
  rejected_claims: number,
  fraud_reports: number,
}

Example (TypeScript):
import { Contract } from "@stellar/stellar-sdk";

const contract = new Contract({
  contractId: "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX",
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
});

const result = await contract.get_verifier("GD...XYZ");
console.log(result.reputation_score); // 740
`);
}

// ============================================================================
// MAIN DEMO
// ============================================================================

/**
 * Main demo function
 */
function main() {
  console.log("\n" + "=".repeat(60));
  console.log("🌟 KCHNG REPUTATION SYSTEM - THIRD-PARTY DEMO");
  console.log("=".repeat(60));
  console.log("\nThis demo shows how external apps can use the KCHNG contract's");
  console.log("reputation system to build custom features on top.\n");

  // Display various third-party use cases
  displayLeaderboard(EXAMPLE_VERIFIERS);
  displayTierAnalysis(EXAMPLE_VERIFIERS);

  const recommendations = recommendVerifiers(EXAMPLE_VERIFIERS, 3);
  displayRecommendations(recommendations);

  displayContractIntegration();

  console.log("=".repeat(60));
  console.log("🔧 EXTENSIBILITY EXAMPLES");
  console.log("=".repeat(60));
  console.log("\nThird-party apps could use reputation data for:");
  console.log("");
  console.log("  • Verifier Bounties    - Pay higher rates to top-tier verifiers");
  console.log("  • Priority Queue       - Process claims from high-reputation workers first");
  console.log("  • Badge System         - Award badges for reputation milestones");
  console.log("  • Discount Program     - Offer discounts based on reputation");
  console.log("  • Insurance            - Lower fees for high-reputation participants");
  console.log("  • Governance Weight    - Extra voting power for trusted members");
  console.log("");

  console.log("=".repeat(60));
  console.log("✨ The contract tracks reputation, but doesn't enforce how it's used!");
  console.log("=".repeat(60) + "\n");
}

// Run the demo
main();
