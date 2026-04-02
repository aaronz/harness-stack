import './globals.css'
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'AI-Ready Evaluator',
  description: 'Assess development requirements for AI Coding implementability',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <div className="min-h-screen bg-gray-50">
          <header className="bg-white shadow-sm">
            <div className="max-w-5xl mx-auto px-4 py-4 flex justify-between items-center">
              <h1 className="text-xl font-semibold text-gray-900">
                AI-Ready Evaluator
              </h1>
              <nav className="flex gap-4">
                <a href="/" className="text-gray-600 hover:text-gray-900">
                  Home
                </a>
                <a href="/settings" className="text-gray-600 hover:text-gray-900">
                  Settings
                </a>
              </nav>
            </div>
          </header>
          <main className="max-w-5xl mx-auto px-4 py-8">
            {children}
          </main>
        </div>
      </body>
    </html>
  )
}
