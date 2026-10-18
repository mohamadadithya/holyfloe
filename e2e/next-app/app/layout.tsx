import type { ReactNode } from 'react';

export const metadata = { title: 'holyfloe next-app e2e' };

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
