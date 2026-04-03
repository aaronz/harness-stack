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
const MAX_LEN = 50000;

router.post('/', requireFields(['text']), (req: Request, res: Response) => {
  try {
    const { text, submittedBy } = req.body;
    if (!text || typeof text !== 'string') { res.status(400).json({ error: { message: 'Requirement text required', code: 'MISSING_TEXT' } }); return; }
    if (text.length > MAX_LEN) { res.status(400).json({ error: { message: `Text exceeds ${MAX_LEN} chars`, code: 'TEXT_TOO_LONG' } }); return; }

    const requirement = requirementRepository.create({ text, submittedBy });
    const dims = scoringEngine.calculateScores(text);
    const penalty = scoringEngine.detectComplexity(text);
    const total = scoringEngine.applyWeights(dims, penalty);
    const level = scoreClassifier.classify(total);
    const score = scoreRepository.create({ requirementId: requirement.id, totalScore: total, level, contextScore: dims.context, atomicityScore: dims.atomicity, boundaryScore: dims.boundary, verifiabilityScore: dims.verifiability, technicalScore: dims.technical, complexityPenalty: penalty });
    const analysis = riskAnalyzer.analyze(text, score.id, dims);

    res.status(201).json({ requirement: { id: requirement.id, text: requirement.text, submittedAt: requirement.submittedAt }, score: { totalScore: score.totalScore, level: score.level, dimensionScores: { context: score.contextScore, atomicity: score.atomicityScore, boundary: score.boundaryScore, verifiability: score.verifiabilityScore, technical: score.technicalScore }, complexityPenalty: score.complexityPenalty }, riskAnalysis: { highlights: analysis.highlights, suggestions: analysis.suggestions } });
  } catch (err) { console.error(err); res.status(500).json({ error: { message: 'Processing failed', code: 'PROCESSING_ERROR' } }); }
});

router.get('/:id', (req: Request, res: Response) => {
  try {
    const req2 = requirementRepository.findById(req.params.id);
    if (!req2) { res.status(404).json({ error: { message: 'Not found', code: 'NOT_FOUND' } }); return; }
    const score = scoreRepository.findByRequirementId(req2.id);
    const highlights = score ? riskHighlightRepository.findByScoreId(score.id) : [];
    const suggestions = score ? optimizationSuggestionRepository.findByScoreId(score.id) : [];
    res.json({ requirement: req2, score: score ? { totalScore: score.totalScore, level: score.level, dimensionScores: { context: score.contextScore, atomicity: score.atomicityScore, boundary: score.boundaryScore, verifiability: score.verifiabilityScore, technical: score.technicalScore }, complexityPenalty: score.complexityPenalty } : null, riskAnalysis: { highlights: highlights.map((h) => ({ dimension: h.dimension, textSpan: h.textSpan, reason: h.reason })), suggestions: suggestions.map((s) => ({ category: s.category, description: s.description, priority: s.priority })) } });
  } catch (err) { res.status(500).json({ error: { message: 'Retrieval failed', code: 'RETRIEVAL_ERROR' } }); }
});

router.get('/', (req: Request, res: Response) => {
  const limit = Math.min(parseInt(req.query.limit as string) || 100, 100);
  const offset = parseInt(req.query.offset as string) || 0;
  const list = requirementRepository.findAll(limit, offset);
  res.json({ requirements: list.map((r) => ({ id: r.id, text: r.text.substring(0, 200) + (r.text.length > 200 ? '...' : ''), submittedAt: r.submittedAt })), limit, offset });
});

export default router;