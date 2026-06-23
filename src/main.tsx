import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router";
import "./index.css";

import Home from "./pages/Home.tsx";
import Dialogue from "./pages/Dialogue.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<Home />} />
        <Route path="/dialogue" element={<Dialogue />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
