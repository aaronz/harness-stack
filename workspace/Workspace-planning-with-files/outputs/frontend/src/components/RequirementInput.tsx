interface RequirementInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  loading: boolean;
}

export default function RequirementInput({
  value,
  onChange,
  onSubmit,
  loading
}: RequirementInputProps) {
  return (
    <div className="mb-8 bg-white rounded-xl shadow-sm border p-4 sm:p-6">
      <label className="block text-lg font-medium mb-2">输入开发需求描述</label>
      <textarea
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="请输入需要评估的开发需求...
        
例如：实现用户注册功能。输入：用户名（6-20位字母数字），密码（8-32位必须包含大小写字母和数字）。输出：用户对象或错误码..."
        className="w-full h-48 p-4 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
      <div className="flex justify-between items-center mt-2">
        <span className="text-sm text-gray-500">{value.length} 字符</span>
        <button
          onClick={onSubmit}
          disabled={!value.trim() || loading}
          className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? '评估中...' : '开始评估'}
        </button>
      </div>
    </div>
  );
}
