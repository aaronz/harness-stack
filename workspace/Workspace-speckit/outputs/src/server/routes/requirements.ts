import { Router, Request, Response } from 'express';
import { requirementRepository } from '../../entities/requirement';
import { scoreRepository } from '../../entities/score';
import { scoringEngine } from '../../services/scoring-engine';
import { scoreClassifier } from '../../services/score-classifier';
import { riskAnalyzer } from '../../services/risk-analyzer';
import { riskHighlightRepository } from '../../entities/risk-highlight';
import { optimizationSuggestionRepository } from '../../entities/optimization-suggestion';
import { requireFields } from '../../middleware/validation';

const router = Router();

const MAX_REQUIREMENT_LENGTH = 50000;

router.post('/', requireFields(['text']), (req: Request, res: Response) => {
  try {
    const { text, submittedBy } = req.body;

    if (typeof text !== 'string' || text.length === 0) {
      res.status(400).json({
        error: {
          message: 'Requirement text is required',
          code: 'MISSING_TEXT',
        },
      });
      return;
    }

    if (text.length > MAX_REQUIREMENT_LENGTH) {
      res.status(400).json({
        error: {
          message: `Requirement text exceeds maximum length of ${MAX_REQUIREMENT_LENGTH} characters`,
          code: 'TEXT_TOO_LONG',
        },
      });
      return;
    }

    const requirement = requirementRepository.create({ text, submittedBy });

    const dimensionScores = scoringEngine.calculateScores(text);
    const complexityPenalty = scoringEngine.detectComplexity(text);
    const weightedScore = scoringEngine.applyWeights(dimensionScores, complexityPenalty);
    const level = scoreClassifier.classify(weightedScore);

    const score = scoreRepository.create({
      requirementId: requirement.id,
      totalScore: weightedScore,
      level,
      contextScore: dimensionScores.context,
      atomicityScore: dimensionScores.atomicity,
      boundaryScore: dimensionScores.boundary,
      verifiabilityScore: dimensionScores.verifiability,
      technicalScore: dimensionScores.technical,
      complexityPenalty,
    });

    const riskAnalysis = riskAnalyzer.analyze(text, score.id, dimensionScores);

    res.status(201).json({
      requirement: {
        id: requirement.id,
        text: requirement.text,
        submittedAt: requirement.submittedAt,
      },
      score: {
        totalScore: score.totalScore,
        level: score.level,
        dimensionScores: {
          context: score.contextScore,
          atomicity: score.atomicityScore,
          boundary: score.boundaryScore,
          verifiability: score.verifiabilityScore,
          technical: score.technicalScore,
        },
        complexityPenalty: score.complexityPenalty,
      },
      riskAnalysis: {
        highlights: riskAnalysis.highlights,
        suggestions: riskAnalysis.suggestions,
      },
    });
  } catch (error) {
    console.error('Error creating requirement:', error);
    res.status(500).json({
      error: {
        message: 'Failed to process requirement',
        code: 'PROCESSING_ERROR',
      },
    });
  }
});

router.get('/:id', async (req: Request, res: Response) => {
  try {
    const { id } = req.params;

    const requirement = requirementRepository.findById(id);
    if (!requirement) {
      res.status(404).json({
        error: {
          message: 'Requirement not found',
          code: 'NOT_FOUND',
        },
      });
      return;
    }

    const score = scoreRepository.findByRequirementId(id);
    const highlights = score ? riskHighlightRepository.findByScoreId(score.id) : [];
    const suggestions = score ? optimizationSuggestionRepository.findByScoreId(score.id) : [];

    res.json({
      requirement,
      score: score
        ? {
            totalScore: score.totalScore,
            level: score.level,
            dimensionScores: {
              context: score.contextScore,
              atomicity: score.atomicityScore,
              boundary: score.boundaryScore,
              verifiability: score.verifiabilityScore,
              technical: score.technicalScore,
            },
            complexityPenalty: score.complexityPenalty,
          }
        : null,
      riskAnalysis: {
        highlights: highlights.map((h) => ({
          dimension: h.dimension,
          textSpan: h.textSpan,
          reason: h.reason,
        })),
        suggestions: suggestions.map((s) => ({
          category: s.category,
          description: s.description,
          priority: s.priority,
        })),
      },
    });
  } catch (error) {
    console.error('Error retrieving requirement:', error);
    res.status(500).json({
      error: {
        message: 'Failed to retrieve requirement',
        code: 'RETRIEVAL_ERROR',
      },
    });
  }
});

router.get('/', (req: Request, res: Response) => {
  try {
    const limit = Math.min(parseInt(req.query.limit as string) || 100, 100);
    const offset = parseInt(req.query.offset as string) || 0;

    const requirements = requirementRepository.findAll(limit, offset);

    res.json({
      requirements: requirements.map((r) => ({
        id: r.id,
        text: r.text.substring(0, 200) + (r.text.length > 200 ? '...' : ''),
        submittedAt: r.submittedAt,
      })),
      limit,
      offset,
    });
  } catch (error) {
    console.error('Error listing requirements:', error);
    res.status(500).json({
      error: {
        message: 'Failed to list requirements',
        code: 'LIST_ERROR',
      },
    });
  }
});

export default router;
