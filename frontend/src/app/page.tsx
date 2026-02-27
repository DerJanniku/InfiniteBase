"use client";

import { useEffect, useState } from "react";
import { Tldraw } from "tldraw";
import "tldraw/tldraw.css";

// InfiniteBase Canvas Component
function InfiniteCanvas() {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    setIsReady(true);
  }, []);

  if (!isReady) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          backgroundColor: "#000000",
          color: "#ffffff",
        }}
      >
        <div className="loading-spinner" />
      </div>
    );
  }

  return (
    <div 
      style={{ 
        position: "fixed", 
        inset: 0,
        backgroundColor: "#000000",
      }}
    >
      <Tldraw hideUi={false} />
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

