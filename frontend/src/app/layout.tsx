import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "InfiniteBase - Local Visual OS",
  description: "Infinite canvas for your digital brain. Local-first, privacy-focused.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}

