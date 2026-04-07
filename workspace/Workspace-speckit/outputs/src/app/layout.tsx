export const metadata = { title: 'AI-Readiness Evaluator', description: 'Evaluate requirements for AI Coding' };

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}