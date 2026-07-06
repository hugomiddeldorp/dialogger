import { useState } from "react";
import "./AppLayout.css";

import Menu from "../components/Menu.tsx";

export default function AppLayout({ children }) {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="appLayout">
      <div className="titleBar">
        <i onClick={() => setIsOpen(true)} className="icon icon-menu"></i>
        <h1>French</h1>
      </div>
      <Menu isOpen={isOpen} onClose={() => setIsOpen(false)} />
      {children}
    </div>
  );
}
