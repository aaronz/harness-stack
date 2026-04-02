'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import DashboardLayout from '@/components/Layout';
import { useAuthStore } from '@/store/auth';
import { evaluationApi } from '@/lib/api';
import { useQuery } from '@tanstack/react-query';
import { FileText, Clock, AlertCircle } from 'lucide-react';

export default function DashboardPage() {
  const router = useRouter();
  const { isAuthenticated } = useAuthStore();

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  const { data } = useQuery({
    queryKey: ['evaluations'],
    queryFn: () => evaluationApi.list(1, 5).then(res => res.data),
    enabled: isAuthenticated,
  });

  if (!isAuthenticated) return null;

  const evaluations = data?.evaluations || [];
  const stats = {
    total: data?.total || 0,
    completed: evaluations.filter((e: any) => e.status === 'COMPLETED').length,
    pending: evaluations.filter((e: any) => e.status === 'PENDING' || e.status === 'PROCESSING').length,
  };

  return (
    <DashboardLayout>
      <div className="max-w-6xl">
        <h1 className="text-2xl font-bold mb-6">Dashboard</h1>

        <div className="grid grid-cols-3 gap-6 mb-8">
          <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
            <div className="text-3xl font-bold text-primary">{stats.total}</div>
            <div className="text-gray-500 text-sm mt-1">Total Evaluations</div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
            <div className="text-3xl font-bold text-success">{stats.completed}</div>
            <div className="text-gray-500 text-sm mt-1">Completed</div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
            <div className="text-3xl font-bold text-warning">{stats.pending}</div>
            <div className="text-gray-500 text-sm mt-1">In Progress</div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border border-gray-200">
          <div className="p-4 border-b border-gray-200 flex justify-between items-center">
            <h2 className="font-semibold">Recent Evaluations</h2>
            <a href="/reports" className="text-primary text-sm hover:underline">View all</a>
          </div>

          {evaluations.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              No evaluations yet. <a href="/evaluate" className="text-primary hover:underline">Start evaluating</a>
            </div>
          ) : (
            <div className="divide-y divide-gray-100">
              {evaluations.map((evaluation: any) => (
                <div key={evaluation.id} className="p-4 hover:bg-gray-50 flex items-center justify-between">
                  <div className="flex items-center gap-4">
                    {evaluation.status === 'COMPLETED' ? (
                      <FileText className="text-success" size={20} />
                    ) : evaluation.status === 'PROCESSING' ? (
                      <Clock className="text-warning" size={20} />
                    ) : (
                      <AlertCircle className="text-gray-400" size={20} />
                    )}
                    <div>
                      <div className="font-medium line-clamp-1 max-w-md">
                        {evaluation.requirementText.substring(0, 100)}...
                      </div>
                      <div className="text-sm text-gray-500">
                        {new Date(evaluation.createdAt).toLocaleString()}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    {evaluation.result && (
                      <span className={`px-3 py-1 rounded-full text-xs font-medium ${
                        evaluation.result.level === 'S' ? 'bg-yellow-100 text-yellow-800' :
                        evaluation.result.level === 'A' ? 'bg-green-100 text-green-800' :
                        evaluation.result.level === 'B' ? 'bg-orange-100 text-orange-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {evaluation.result.level}-Level ({Math.round(evaluation.result.overallScore)})
                      </span>
                    )}
                    <span className={`px-2 py-1 rounded text-xs ${
                      evaluation.status === 'COMPLETED' ? 'bg-green-50 text-green-600' :
                      evaluation.status === 'PROCESSING' ? 'bg-yellow-50 text-yellow-600' :
                      'bg-gray-50 text-gray-600'
                    }`}>
                      {evaluation.status}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </DashboardLayout>
  );
}