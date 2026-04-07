import './globals.css';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'AI Coding 可落地性评估系统',
  description: '需求可落地性评估前端'
};

export default function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN">
      <body className="bg-gray-50 text-gray-900">{children}</body>
    </html>
  );
}
