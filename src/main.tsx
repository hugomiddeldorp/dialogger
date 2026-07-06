import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import { Toaster } from "sonner";
import "./index.css";

import Home from "./pages/Home.tsx";
import Dialogue from "./pages/Dialogue.tsx";
import Settings from "./pages/Settings.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Toaster position="top-right" richColors closeButton />
    <BrowserRouter>
      <Routes>
        <Route index element={<Home />} />
        <Route path="/dialogue/:conversationId" element={<Dialogue />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
