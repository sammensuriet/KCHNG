#!/usr/bin/env python3
"""
KCHNG Game Theory Simulation
Testing Tit-for-Tat vs Tit-for-2-Tats strategies in mutual credit system

Contract: CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX
Network: Stellar Testnet
"""

import json
import random
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional, Tuple
from pathlib import Path

# Try to import Stellar SDK
try:
    from stellar_sdk import Server, Keypair, TransactionBuilder, Network
    from stellar_sdk.exceptions import PrepareTransactionException
    STELLAR_AVAILABLE = True
except ImportError:
    STELLAR_AVAILABLE = False
    print("⚠️  Stellar SDK not available. Install with: pip install stellar-sdk")


class Strategy(Enum):
    ALWAYS_COOPERATE = "AC"
    ALWAYS_DEFECT = "AD"
    TIT_FOR_TAT = "TFT"
    TIT_FOR_2_TATS = "TF2T"
    RANDOM = "RAND"
    GRIM_TRIGGER = "GRIM"


class WorkType(Enum):
    BASIC_CARE = 0
    SKILLED_CARE = 1
    TRAINING = 2
    EMERGENCY_CARE = 3


class ClaimStatus(Enum):
    PENDING = 0
    APPROVED = 1
    REJECTED = 2
    EXPIRED = 3


@dataclass
class VerifierState:
    """State of a verifier in the simulation"""
    name: str
    strategy: Strategy
    reputation_score: int = 500  # Start at 500 (0-1000 scale)
    verified_claims: int = 0
    rejected_claims: int = 0
    fraud_reports: int = 0
    last_moves: Dict[str, List[str]] = field(default_factory=dict)  # Track interactions with others
    has_defected: Dict[str, bool] = field(default_factory=dict)  # For GRIM strategy

    def decide(self, other_verifier: 'VerifierState', claim_quality: str) -> str:
        """
        Decide whether to approve or reject a work claim.
        claim_quality: "honest" or "fraudulent"
        """
        # If work is fraudulent, good verifiers should reject
        if claim_quality == "fraudulent":
            if self.strategy == Strategy.ALWAYS_COOPERATE:
                return "approve"  # Naive
            elif self.strategy == Strategy.RANDOM:
                return random.choice(["approve", "reject"])
            else:
                return "reject"  # All other strategies reject fraud

        # For honest work, use strategy
        if self.strategy == Strategy.ALWAYS_COOPERATE:
            return "approve"

        elif self.strategy == Strategy.ALWAYS_DEFECT:
            return "reject"

        elif self.strategy == Strategy.RANDOM:
            return random.choice(["approve", "reject"])

        elif self.strategy == Strategy.TIT_FOR_TAT:
            # Mirror last move
            last_moves = self.last_moves.get(other_verifier.name, ["cooperate"])
            last_move = last_moves[-1] if last_moves else "cooperate"
            return "approve" if last_move == "approve" else "reject"

        elif self.strategy == Strategy.TIT_FOR_2_TATS:
            # Defect only if opponent defected TWICE in a row
            last_moves = self.last_moves.get(other_verifier.name, ["cooperate", "cooperate"])
            recent = last_moves[-2:] if len(last_moves) >= 2 else last_moves
            consecutive_defections = sum(1 for m in recent if m == "reject")
            return "reject" if consecutive_defections >= 2 else "approve"

        elif self.strategy == Strategy.GRIM_TRIGGER:
            # Once defected against, always defect
            if self.has_defected.get(other_verifier.name, False):
                return "reject"
            return "approve"

        return "approve"  # Default


@dataclass
class WorkerState:
    """State of a worker in the simulation"""
    name: str
    balance: int = 100
    contribution_hours: float = 0.0
    claims_submitted: int = 0
    claims_approved: int = 0
    claims_rejected: int = 0
    fraud_rate: float = 0.1  # 10% of work is fraudulent


@dataclass
class WorkClaim:
    """A work claim in the system"""
    claim_id: int
    worker: str
    work_type: WorkType
    minutes_worked: int
    is_honest: bool
    status: ClaimStatus = ClaimStatus.PENDING
    verifiers_assigned: List[str] = field(default_factory=list)
    approvals: int = 0
    rejections: int = 0


@dataclass
class SimulationMetrics:
    """Track metrics throughout simulation"""
    round: int = 0
    total_claims: int = 0
    approved_claims: int = 0
    rejected_claims: int = 0
    fraud_detected: int = 0
    fraud_approved: int = 0
    honest_rejected: int = 0

    def to_dict(self):
        return {
            "round": self.round,
            "total_claims": self.total_claims,
            "approved_claims": self.approved_claims,
            "rejected_claims": self.rejected_claims,
            "fraud_detected": self.fraud_detected,
            "fraud_approved": self.fraud_approved,
            "honest_rejected": self.honest_rejected,
        }


class SimulationConfig:
    """Configuration for the simulation"""
    def __init__(self):
        self.contract_id = "CDAKPFYVD6LYCKMTQAHLOYLLYO2PVE6YJIH3SS2W4R5GEJJ75UUZCDPX"
        self.network_url = "https://soroban-testnet.stellar.org"
        self.horizon_url = "https://horizon-testnet.stellar.org"
        self.rounds = 100
        self.claims_per_round = 2
        self.verifiers_per_claim = 2
        self.checkpoint_interval = 100
        self.metrics_interval = 10


class GameTheorySimulation:
    """Main simulation class"""

    def __init__(self, config: SimulationConfig):
        self.config = config
        self.verifiers: List[VerifierState] = []
        self.workers: List[WorkerState] = []
        self.claims: List[WorkClaim] = []
        self.metrics_history: List[SimulationMetrics] = []
        self.checkpoint_dir = Path("/tmp/kchng-simulation/checkpoints")
        self.checkpoint_dir.mkdir(parents=True, exist_ok=True)

        # Server connection (if Stellar SDK available)
        self.server = None
        if STELLAR_AVAILABLE:
            self.server = Server(self.config.horizon_url)

    def setup_verifiers(self):
        """Create verifiers with different strategies"""
        strategies = [
            (Strategy.ALWAYS_COOPERATE, 2),
            (Strategy.ALWAYS_DEFECT, 2),
            (Strategy.TIT_FOR_TAT, 4),
            (Strategy.TIT_FOR_2_TATS, 4),
            (Strategy.RANDOM, 2),
            (Strategy.GRIM_TRIGGER, 2),
        ]

        verifier_id = 0
        for strategy, count in strategies:
            for i in range(count):
                verifier = VerifierState(
                    name=f"{strategy.value}_{i+1}",
                    strategy=strategy,
                    reputation_score=500,
                )
                self.verifiers.append(verifier)
                verifier_id += 1

        print(f"✓ Created {len(self.verifiers)} verifiers")
        for strategy, _ in strategies:
            count = sum(1 for v in self.verifiers if v.strategy == strategy)
            print(f"  - {strategy.value}: {count} verifiers")

    def setup_workers(self):
        """Create workers"""
        for i in range(8):
            worker = WorkerState(
                name=f"worker_{i+1}",
                balance=100,
                fraud_rate=0.1,  # 10% fraudulent work
            )
            self.workers.append(worker)

        print(f"✓ Created {len(self.workers)} workers")

    def update_reputation(self, verifier: VerifierState, claim: WorkClaim, decision: str):
        """Update verifier reputation based on their decision"""
        # Base reputation change
        if claim.is_honest:
            if decision == "approve":
                verifier.reputation_score += 5
                verifier.verified_claims += 1
            else:  # rejected honest work
                verifier.reputation_score -= 10
                verifier.rejected_claims += 1
        else:  # fraudulent work
            if decision == "reject":
                verifier.reputation_score += 10  # Caught fraud
                verifier.verified_claims += 1
            else:  # approved fraud
                verifier.reputation_score -= 50  # Big penalty
                verifier.fraud_reports += 1

        # Clamp reputation
        verifier.reputation_score = max(0, min(1000, verifier.reputation_score))

    def process_claim(self, claim: WorkClaim) -> ClaimStatus:
        """Process a work claim through verifier review"""
        # Assign random verifiers
        assigned = random.sample(self.verifiers, self.config.verifiers_per_claim)
        claim.verifiers_assigned = [v.name for v in assigned]

        approvals = 0
        rejections = 0

        for verifier in assigned:
            # Get decision from each verifier
            # For cross-verification, each verifier considers the other
            other = assigned[1] if verifier == assigned[0] else assigned[0]
            decision = verifier.decide(other, "honest" if claim.is_honest else "fraudulent")

            # Update verifier's memory of other's behavior
            if other.name not in verifier.last_moves:
                verifier.last_moves[other.name] = []
            verifier.last_moves[other.name].append(decision)

            # For GRIM strategy, track if anyone defected
            if decision == "reject":
                verifier.has_defected[other.name] = True

            if decision == "approve":
                approvals += 1
            else:
                rejections += 1

            # Update reputation
            self.update_reputation(verifier, claim, decision)

        claim.approvals = approvals
        claim.rejections = rejections

        # Claim approved if majority approve
        if approvals > rejections:
            claim.status = ClaimStatus.APPROVED
            # Worker earns hours
            worker = next(w for w in self.workers if w.name == claim.worker)
            worker.contribution_hours += claim.minutes_worked / 60
            worker.claims_approved += 1
        else:
            claim.status = ClaimStatus.REJECTED
            worker = next(w for w in self.workers if w.name == claim.worker)
            worker.claims_rejected += 1

        return claim.status

    def run_round(self, round_num: int) -> SimulationMetrics:
        """Run one round of simulation"""
        metrics = SimulationMetrics(round=round_num)

        for _ in range(self.config.claims_per_round):
            # Select random worker
            worker = random.choice(self.workers)

            # Determine if work is honest or fraudulent
            is_honest = random.random() > worker.fraud_rate

            # Create claim
            claim = WorkClaim(
                claim_id=len(self.claims),
                worker=worker.name,
                work_type=random.choice(list(WorkType)),
                minutes_worked=random.choice([30, 60, 120, 180]),
                is_honest=is_honest,
            )

            self.claims.append(claim)
            worker.claims_submitted += 1
            metrics.total_claims += 1

            # Process claim
            status = self.process_claim(claim)

            if status == ClaimStatus.APPROVED:
                metrics.approved_claims += 1
                if not is_honest:
                    metrics.fraud_approved += 1
            else:
                metrics.rejected_claims += 1
                if not is_honest:
                    metrics.fraud_detected += 1
                else:
                    metrics.honest_rejected += 1

        return metrics

    def save_checkpoint(self, round_num: int):
        """Save simulation state"""
        if round_num % self.config.checkpoint_interval != 0:
            return

        state = {
            "round": round_num,
            "verifiers": [
                {
                    "name": v.name,
                    "strategy": v.strategy.value,
                    "reputation_score": v.reputation_score,
                    "verified_claims": v.verified_claims,
                    "rejected_claims": v.rejected_claims,
                    "fraud_reports": v.fraud_reports,
                }
                for v in self.verifiers
            ],
            "workers": [
                {
                    "name": w.name,
                    "balance": w.balance,
                    "contribution_hours": w.contribution_hours,
                    "claims_submitted": w.claims_submitted,
                    "claims_approved": w.claims_approved,
                    "claims_rejected": w.claims_rejected,
                }
                for w in self.workers
            ],
            "metrics": [m.to_dict() for m in self.metrics_history],
        }

        filepath = self.checkpoint_dir / f"checkpoint_{round_num}.json"
        with open(filepath, 'w') as f:
            json.dump(state, f, indent=2)

        print(f"✓ Checkpoint saved at round {round_num}")

    def print_progress(self, round_num: int, start_time: float):
        """Print simulation progress"""
        if round_num % self.config.metrics_interval == 0 or round_num == 1:
            elapsed = time.time() - start_time
            avg_time = elapsed / round_num
            remaining = (self.config.rounds - round_num) * avg_time

            print(f"Round {round_num}/{self.config.rounds} | "
                  f"Elapsed: {elapsed:.1f}s | ETA: {remaining:.1f}s")

            # Show current rankings
            sorted_verifiers = sorted(self.verifiers, key=lambda v: v.reputation_score, reverse=True)
            print("  Top 5 Verifiers:")
            for v in sorted_verifiers[:5]:
                print(f"    {v.name:15} | Reputation: {v.reputation_score:3} | "
                      f"Strategy: {v.strategy.value}")

    def run_simulation(self) -> Dict:
        """Run the full simulation"""
        print("\n" + "="*60)
        print("KCHNG Game Theory Simulation")
        print("="*60)
        print(f"Contract: {self.config.contract_id}")
        print(f"Rounds: {self.config.rounds}")
        print(f"Claims per round: {self.config.claims_per_round}")
        print(f"Verifiers: {len(self.verifiers)}")
        print(f"Workers: {len(self.workers)}")
        print("="*60 + "\n")

        start_time = time.time()

        for round_num in range(1, self.config.rounds + 1):
            metrics = self.run_round(round_num)
            self.metrics_history.append(metrics)

            self.print_progress(round_num, start_time)
            self.save_checkpoint(round_num)

        elapsed = time.time() - start_time
        print(f"\n✓ Simulation completed in {elapsed:.1f}s")

        return self.generate_final_report()

    def generate_final_report(self) -> Dict:
        """Generate final simulation report"""
        # Calculate statistics by strategy
        strategy_stats = {}
        for strategy in Strategy:
            verifiers = [v for v in self.verifiers if v.strategy == strategy]
            if verifiers:
                avg_reputation = sum(v.reputation_score for v in verifiers) / len(verifiers)
                avg_verified = sum(v.verified_claims for v in verifiers) / len(verifiers)
                avg_rejected = sum(v.rejected_claims for v in verifiers) / len(verifiers)

                strategy_stats[strategy.value] = {
                    "count": len(verifiers),
                    "avg_reputation": avg_reputation,
                    "avg_verified_claims": avg_verified,
                    "avg_rejected_claims": avg_rejected,
                    "final_reputations": [v.reputation_score for v in verifiers],
                }

        # Worker statistics
        total_hours = sum(w.contribution_hours for w in self.workers)
        approval_rate = sum(w.claims_approved for w in self.workers) / sum(w.claims_submitted for w in self.workers)

        report = {
            "config": {
                "rounds": self.config.rounds,
                "claims_per_round": self.config.claims_per_round,
                "verifiers": len(self.verifiers),
                "workers": len(self.workers),
            },
            "strategy_stats": strategy_stats,
            "worker_stats": {
                "total_contribution_hours": total_hours,
                "avg_hours_per_worker": total_hours / len(self.workers),
                "approval_rate": approval_rate,
            },
            "final_rankings": {
                v.name: {
                    "strategy": v.strategy.value,
                    "reputation": v.reputation_score,
                    "verified": v.verified_claims,
                    "rejected": v.rejected_claims,
                }
                for v in sorted(self.verifiers, key=lambda v: v.reputation_score, reverse=True)
            },
        }

        # Save report
        report_path = self.checkpoint_dir / f"report_{self.config.rounds}_rounds.json"
        with open(report_path, 'w') as f:
            json.dump(report, f, indent=2)

        print(f"\n✓ Report saved to {report_path}")

        return report

    def print_final_report(self, report: Dict):
        """Print formatted final report"""
        print("\n" + "="*60)
        print("FINAL SIMULATION REPORT")
        print("="*60 + "\n")

        print("📊 Strategy Performance (Average Reputation)")
        print("-" * 60)
        strategies = sorted(
            report["strategy_stats"].items(),
            key=lambda x: x[1]["avg_reputation"],
            reverse=True
        )

        medals = ["🥇", "🥈", "🥉"]
        for i, (strategy, stats) in enumerate(strategies):
            medal = medals[i] if i < 3 else "  "
            print(f"{medal} {strategy:12} | Reputation: {stats['avg_reputation']:6.1f} | "
                  f"Verified: {stats['avg_verified_claims']:5.0f} | "
                  f"Rejected: {stats['avg_rejected_claims']:5.0f}")

        print("\n🏆 Top 5 Verifiers")
        print("-" * 60)
        for i, (name, data) in enumerate(list(report["final_rankings"].items())[:5]):
            print(f"{i+1}. {name:15} | {data['strategy']:8} | "
                  f"Reputation: {data['reputation']:3} | "
                  f"Verified: {data['verified']} | Rejected: {data['rejected']}")

        print("\n📈 Worker Statistics")
        print("-" * 60)
        ws = report["worker_stats"]
        print(f"Total Contribution Hours: {ws['total_contribution_hours']:.1f}")
        print(f"Average Hours per Worker: {ws['avg_hours_per_worker']:.1f}")
        print(f"Approval Rate: {ws['approval_rate']:.1%}")

        print("\n" + "="*60)


def main():
    """Main entry point"""
    config = SimulationConfig()

    # Allow command line override
    import sys
    if len(sys.argv) > 1:
        config.rounds = int(sys.argv[1])
        print(f"Running {config.rounds} rounds as requested")

    sim = GameTheorySimulation(config)

    # Setup
    sim.setup_verifiers()
    sim.setup_workers()

    # Run
    report = sim.run_simulation()

    # Print results
    sim.print_final_report(report)


if __name__ == "__main__":
    main()
