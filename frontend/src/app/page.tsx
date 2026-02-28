"use client";

import { useEffect, useState } from "react";
import { Tldraw, Editor } from "@tldraw/tldraw";
import "@tldraw/tldraw/tldraw.css";
import { Toaster } from "react-hot-toast";
import { useFileUpload } from "@/hooks/useFileUpload";

// InfiniteBase Canvas Component
function InfiniteCanvas() {
  const [isReady, setIsReady] = useState(false);
  const [editor, setEditor] = useState<Editor | null>(null);
  const { handleDrop } = useFileUpload({ editor });

  useEffect(() => {
    setIsReady(true);
  }, []);

  if (!isReady) {
    return (
      <div className="flex items-center justify-center h-screen bg-black text-white">
        <div className="loading-spinner" />
      </div>
    );
  }

  return (
    <div 
      className="canvas-container"
      onDrop={handleDrop}
      onDragOver={(e) => e.preventDefault()}
    >
      <Tldraw 
        inferDarkMode
        persistenceKey="infinitebase-main"
        autoFocus
        onMount={(editor) => setEditor(editor)}
      />
      <Toaster 
        position="bottom-right"
        toastOptions={{
          style: {
            background: "#1a1a1a",
            color: "#fff",
            border: "1px solid #333",
          },
        }}
      />
    </div>
  );
}

export default function Home() {
  return (
    <main>
      <InfiniteCanvas />
    </main>
  );
}
