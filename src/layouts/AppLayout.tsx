import "./AppLayout.css";

import TitleBar from "../components/TitleBar.tsx";

function AppLayout({ children }) {
  return (
    <div className="appLayout">
      <TitleBar />
      {children}
    </div>
  );
}

export default AppLayout;
