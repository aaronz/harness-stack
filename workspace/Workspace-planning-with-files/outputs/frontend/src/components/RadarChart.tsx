'use client';

import {
  Radar,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  ResponsiveContainer
} from 'recharts';

interface RadarChartViewProps {
  data: Record<string, number>;
}

export default function RadarChartView({ data }: RadarChartViewProps) {
  const chartData = Object.entries(data).map(([name, value]) => ({
    subject: name,
    value: Number(value) || 0,
    fullMark: 100
  }));

  if (!chartData.length) {
    return <div className="text-gray-500 text-sm">暂无维度数据可展示</div>;
  }

  return (
    <div className="w-full h-80">
      <ResponsiveContainer width="100%" height="100%">
        <RadarChart cx="50%" cy="50%" outerRadius="70%" data={chartData}>
          <PolarGrid />
          <PolarAngleAxis dataKey="subject" />
          <PolarRadiusAxis angle={30} domain={[0, 100]} />
          <Radar
            name="评分"
            dataKey="value"
            stroke="#2563eb"
            fill="#2563eb"
            fillOpacity={0.35}
          />
        </RadarChart>
      </ResponsiveContainer>
    </div>
  );
}
