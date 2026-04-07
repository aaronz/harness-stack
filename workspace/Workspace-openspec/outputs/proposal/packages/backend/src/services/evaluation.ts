import { prisma } from '../index.js';
import { callLLM } from '../services/llm.js';

interface DimensionScore {
  score: number;
  checks: { name: string; passed: boolean; reason: string }[];
}

const DIMENSION_WEIGHTS = {
  contextSufficiency: 0.25,
  atomicity: 0.25,
  boundaryDefiniteness: 0.20,
  verifiability: 0.15,
  technicalConstraints: 0.15
};

const COMPLEXITY_PENALTIES = {
  SIMPLE: 1.0,
  MEDIUM: 0.9,
  COMPLEX: 0.7
};

const CHECK_PROMPTS = {
  contextSufficiency: `
Evaluate the requirement for context sufficiency (25% weight).
Check for:
1. Business goal statement - Is the business problem clearly described (not just technical solution)?
2. Terminology glossary - Are domain-specific terms defined?
3. User journey mapping - Is the feature's position in the user flow clear?
4. Data entity relationships - Are core data objects and their relationships defined?

Provide scores (0-100) for each check and overall dimension score.
  `,
  atomicity: `
Evaluate the requirement for logical atomicity (25% weight).
Check for:
1. Single responsibility - Does the requirement do only one thing?
2. Context window fit - Is the description under 3000 tokens?
3. Explicit dependencies - Are prerequisites and external dependencies listed?
4. Deployability - Can this be developed, tested, and deployed independently?

Provide scores (0-100) for each check and overall dimension score.
  `,
  boundaryDefiniteness: `
Evaluate the requirement for boundary definiteness (20% weight).
Check for:
1. Input domain definition - Are valid values, invalid values, null handling defined?
2. State transition completeness - Are all states and transitions defined (not just happy path)?
3. Concurrency handling - Is race condition handling specified?
4. Performance boundaries - Are time/space complexity requirements defined?

Provide scores (0-100) for each check and overall dimension score.
  `,
  verifiability: `
Evaluate the requirement for verifiability (15% weight).
Check for:
1. Examples provided - Are input/output examples given (including edge cases)?
2. Quantified acceptance - Are there numeric metrics (not vague "works well")?
3. Automated testability - Can acceptance criteria become unit test assertions?
4. Behavior contracts - Are preconditions, postconditions, invariants defined?

Provide scores (0-100) for each check and overall dimension score.
  `,
  technicalConstraints: `
Evaluate the requirement for technical constraint clarity (15% weight).
Check for:
1. Architecture compliance - Is the required architecture pattern specified?
2. Tech stack lock - Are language version, framework version, library whitelist/blacklist defined?
3. Interface contracts - Is API protocol (REST/GraphQL/gRPC), data format defined?
4. Security constraints - Is auth method, permission model, sensitive data handling defined?

Provide scores (0-100) for each check and overall dimension score.
  `
};

async function evaluateDimension(
  requirementText: string,
  dimension: keyof typeof CHECK_PROMPTS
): Promise<DimensionScore> {
  const prompt = `${CHECK_PROMPTS[dimension]}\n\nRequirement to evaluate:\n${requirementText}\n\nRespond in JSON format:
{
  "checks": [{"name": "check name", "passed": true/false, "reason": "explanation"}],
  "score": 0-100,
  "riskHighlights": [{"text": "problematic text", "reason": "why problematic"}]
}`;

  try {
    const response = await callLLM(prompt);
    return JSON.parse(response);
  } catch (error) {
    return { score: 50, checks: [{ name: 'error', passed: false, reason: 'LLM call failed' }] };
  }
}

function calculateLevel(score: number): 'S' | 'A' | 'B' | 'C' {
  if (score >= 90) return 'S';
  if (score >= 75) return 'A';
  if (score >= 60) return 'B';
  return 'C';
}

export async function evaluateRequirement(evaluationId: string): Promise<void> {
  const evaluation = await prisma.evaluation.findUnique({ where: { id: evaluationId } });
  if (!evaluation) return;

  await prisma.evaluation.update({
    where: { id: evaluationId },
    data: { status: 'PROCESSING' }
  });

  try {
    const [contextScore, atomicityScore, boundaryScore, verifiabilityScore, techScore] = await Promise.all([
      evaluateDimension(evaluation.requirementText, 'contextSufficiency'),
      evaluateDimension(evaluation.requirementText, 'atomicity'),
      evaluateDimension(evaluation.requirementText, 'boundaryDefiniteness'),
      evaluateDimension(evaluation.requirementText, 'verifiability'),
      evaluateDimension(evaluation.requirementText, 'technicalConstraints')
    ]);

    const weightedScore =
      contextScore.score * DIMENSION_WEIGHTS.contextSufficiency +
      atomicityScore.score * DIMENSION_WEIGHTS.atomicity +
      boundaryScore.score * DIMENSION_WEIGHTS.boundaryDefiniteness +
      verifiabilityScore.score * DIMENSION_WEIGHTS.verifiability +
      techScore.score * DIMENSION_WEIGHTS.technicalConstraints;

    const complexity = evaluation.complexity as keyof typeof COMPLEXITY_PENALTIES;
    const finalScore = weightedScore * (COMPLEXITY_PENALTIES[complexity] || 0.9);
    const level = calculateLevel(finalScore);

    const riskHeatmap = [
      ...(contextScore.checks.filter(c => !c.passed).map(c => ({ ...c, type: 'context' }))),
      ...(atomicityScore.checks.filter(c => !c.passed).map(c => ({ ...c, type: 'atomicity' }))),
      ...(boundaryScore.checks.filter(c => !c.passed).map(c => ({ ...c, type: 'boundary' }))),
      ...(verifiabilityScore.checks.filter(c => !c.passed).map(c => ({ ...c, type: 'verifiability' }))),
      ...(techScore.checks.filter(c => !c.passed).map(c => ({ ...c, type: 'technical' })))
    ];

    const contextGaps = [
      contextScore.checks.find(c => c.name.toLowerCase().includes('business') && !c.passed)
        ? 'Missing business goal statement'
        : null,
      contextScore.checks.find(c => c.name.toLowerCase().includes('terminology') && !c.passed)
        ? 'Missing domain terminology definitions'
        : null,
      atomicityScore.checks.find(c => c.name.toLowerCase().includes('dependency') && !c.passed)
        ? 'Unclear dependencies'
        : null
    ].filter(Boolean) as string[];

    const recommendations = [
      ...(contextScore.score < 70 ? [{
        type: 'context',
        priority: 'high',
        title: 'Enhance business context',
        description: 'Add business goal statement, define domain terms, map user journey'
      }] : []),
      ...(atomicityScore.score < 70 ? [{
        type: 'atomicity',
        priority: 'high',
        title: 'Improve requirement granularity',
        description: 'Split into smaller, independent requirements if too large'
      }] : []),
      ...(boundaryScore.score < 70 ? [{
        type: 'boundary',
        priority: 'high',
        title: 'Define edge cases',
        description: 'Specify null handling, error conditions, state transitions'
      }] : []),
      ...(verifiabilityScore.score < 70 ? [{
        type: 'verifiability',
        priority: 'medium',
        title: 'Add verifiable acceptance criteria',
        description: 'Provide examples, quantify metrics, define testable assertions'
      }] : []),
      ...(techScore.score < 70 ? [{
        type: 'technical',
        priority: 'medium',
        title: 'Specify technical constraints',
        description: 'Define tech stack, architecture pattern, security requirements'
      }] : [])
    ];

    await prisma.evaluationResult.create({
      data: {
        evaluationId,
        overallScore: finalScore,
        level,
        contextSufficiency: contextScore.score,
        atomicity: atomicityScore.score,
        boundaryDefiniteness: boundaryScore.score,
        verifiability: verifiabilityScore.score,
        technicalConstraints: techScore.score,
        riskHeatmap: JSON.stringify(riskHeatmap),
        contextGaps: JSON.stringify(contextGaps),
        recommendations: JSON.stringify(recommendations)
      }
    });

    await prisma.evaluation.update({
      where: { id: evaluationId },
      data: { status: 'COMPLETED' }
    });
  } catch (error) {
    await prisma.evaluation.update({
      where: { id: evaluationId },
      data: { status: 'FAILED' }
    });
    console.error('Evaluation failed:', error);
  }
}