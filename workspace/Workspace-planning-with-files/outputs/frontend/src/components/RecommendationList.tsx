interface RecommendationListProps {
  items: string[];
}

export default function RecommendationList({ items }: RecommendationListProps) {
  if (!items.length) {
    return <div className="text-gray-500 text-sm">暂无建议</div>;
  }

  return (
    <ul className="space-y-3 list-disc pl-5 text-sm text-gray-700">
      {items.map((item, index) => (
        <li key={`${item}-${index}`}>{item}</li>
      ))}
    </ul>
  );
}
