'use client';

import RadarChartView from '@/components/RadarChart';
import RecommendationList from '@/components/RecommendationList';

interface EvaluationResultProps {
  result: any;
}

export default function EvaluationResult({ result }: EvaluationResultProps) {
  const score = result?.overallScore ?? result?.score ?? 0;
  const summary = result?.summary ?? '已完成评估';
  const dimensions = result?.dimensions ?? result?.scores ?? {};
  const recommendations =
    result?.recommendations ?? result?.suggestions ?? ['暂无建议'];

  return (
    <section className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div className="lg:col-span-2 bg-white rounded-xl shadow-sm border p-4 sm:p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">评估结果</h2>
          <span className="text-sm px-3 py-1 rounded-full bg-blue-100 text-blue-700">
            总分：{score}
          </span>
        </div>
        <p className="text-gray-700 mb-6">{summary}</p>
        <RadarChartView data={dimensions} />
      </div>

      <div className="bg-white rounded-xl shadow-sm border p-4 sm:p-6">
        <h3 className="text-lg font-semibold mb-4">优化建议</h3>
        <RecommendationList items={recommendations} />
      </div>
    </section>
  );
}
