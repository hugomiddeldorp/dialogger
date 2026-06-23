import { useNavigate } from "react-router";
import { useState } from "react";

import "./Home.css";
import AppLayout from "../layouts/AppLayout.tsx";

export default function Home() {
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");

  function onInputChange(e) {
    setPrompt(e.target.value);
  }

  return (
    <AppLayout>
      <div className="home">
        <form
          onSubmit={() => navigate("/dialogue")}
          method=""
          accept-charset="utf-8"
        >
          <input
            type="text"
            name="prompt"
            id="prompt"
            placeholder="Generate dialogue"
            value={prompt}
            onChange={onInputChange}
          />
          <button type="submit">&gt;</button>
        </form>
      </div>
    </AppLayout>
  );
}
