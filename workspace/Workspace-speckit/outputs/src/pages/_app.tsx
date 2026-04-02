import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'AI-Readiness Evaluator',
  description: 'Evaluate development requirements for AI Coding implementability',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
