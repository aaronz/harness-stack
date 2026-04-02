'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import DashboardLayout from '@/components/Layout';
import { useAuthStore } from '@/store/auth';
import { evaluationApi } from '@/lib/api';
import { useQuery } from '@tanstack/react-query';
import { FileText, ChevronRight, Search } from 'lucide-react';

export default function ReportsPage() {
  const router = useRouter();
  const { isAuthenticated } = useAuthStore();
  const [search, setSearch] = useState('');

  const { data, isLoading } = useQuery({
    queryKey: ['evaluations', 'all'],
    queryFn: () => evaluationApi.list(1, 100).then(res => res.data),
    enabled: isAuthenticated,
  });

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) return null;

  const evaluations = data?.evaluations || [];
  const filtered = evaluations.filter((e: any) =>
    e.requirementText.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <DashboardLayout>
      <div className="max-w-6xl">
        <h1 className="text-2xl font-bold mb-6">Evaluation Reports</h1>

        <div className="mb-6">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={20} />
            <input
              type="text"
              placeholder="Search evaluations..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
            />
          </div>
        </div>

        {isLoading ? (
          <div className="text-center py-12 text-gray-500">Loading...</div>
        ) : filtered.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            No evaluations found. <a href="/evaluate" className="text-primary hover:underline">Create one</a>
          </div>
        ) : (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="divide-y divide-gray-100">
              {filtered.map((evaluation: any) => (
                <div
                  key={evaluation.id}
                  className="p-4 hover:bg-gray-50 flex items-center justify-between cursor-pointer"
                  onClick={() => router.push(`/reports/${evaluation.id}`)}
                >
                  <div className="flex items-center gap-4 flex-1 min-w-0">
                    <FileText className="text-gray-400 flex-shrink-0" size={20} />
                    <div className="min-w-0">
                      <div className="font-medium truncate">{evaluation.requirementText.substring(0, 80)}...</div>
                      <div className="text-sm text-gray-500">
                        {new Date(evaluation.createdAt).toLocaleString()}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4 flex-shrink-0">
                    {evaluation.result && (
                      <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                        evaluation.result.level === 'S' ? 'bg-yellow-100 text-yellow-800' :
                        evaluation.result.level === 'A' ? 'bg-green-100 text-green-800' :
                        evaluation.result.level === 'B' ? 'bg-orange-100 text-orange-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {evaluation.result.level} ({Math.round(evaluation.result.overallScore)})
                      </span>
                    )}
                    <span className={`px-2 py-1 rounded text-xs ${
                      evaluation.status === 'COMPLETED' ? 'bg-green-50 text-green-600' :
                      evaluation.status === 'PROCESSING' ? 'bg-yellow-50 text-yellow-600' :
                      'bg-gray-50 text-gray-600'
                    }`}>
                      {evaluation.status}
                    </span>
                    <ChevronRight className="text-gray-400" size={20} />
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </DashboardLayout>
  );
}