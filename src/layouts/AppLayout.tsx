import { useState } from "react";
import "./AppLayout.css";

import Menu from "../components/Menu.tsx";

export default function AppLayout({ children }) {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="appLayout">
      <div className="titleBar">
        <button onClick={() => setIsOpen(true)}>Menu</button>
        <h1>French</h1>
      </div>
      <Menu isOpen={isOpen} onClose={() => setIsOpen(false)} />
      {children}
    </div>
  );
}
