import { useDocument } from '../contexts/DocumentContext';

export default function Sidebar() {
  const { workspace, openWorkspace, currentDocument } = useDocument();

  return (
    <div
      id="sidebar"
      className="w-64 flex flex-col border-r"
      style={{
        backgroundColor: 'var(--bg-secondary)',
        borderColor: 'var(--border-color)',
      }}
    >
      <div
        className="px-4 py-4 font-semibold border-b"
        style={{ borderColor: 'var(--border-color)' }}
      >
        Files
      </div>

      <div className="flex-1 overflow-y-auto p-2">
        {!workspace ? (
          <div className="text-sm p-2" style={{ color: 'var(--text-secondary)' }}>
            <p className="mb-2">No workspace open</p>
            <button
              onClick={openWorkspace}
              className="px-3 py-1.5 text-sm border rounded cursor-pointer w-full"
              style={{
                backgroundColor: 'var(--bg-primary)',
                borderColor: 'var(--border-color)',
                color: 'var(--text-primary)',
              }}
            >
              Open Workspace
            </button>
          </div>
        ) : (
          <div className="space-y-1">
            {workspace.files?.map((file, index) => (
              <div
                key={index}
                className="px-3 py-2 rounded cursor-pointer text-sm flex items-center gap-2"
                style={{
                  backgroundColor: currentDocument?.filePath === file.path ? 'var(--accent-color)' : 'transparent',
                  color: currentDocument?.filePath === file.path ? 'white' : 'var(--text-primary)',
                }}
              >
                📄 {file.name}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}