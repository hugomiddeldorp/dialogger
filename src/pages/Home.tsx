import { useNavigate } from "react-router";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./Home.css";
import AppLayout from "../layouts/AppLayout.tsx";

export default function Home() {
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState("");

  async function onInputChange(e) {
    setPrompt(e.target.value);
  }

  async function handleGenerate() {
    setIsGenerating(true);
    setError("");

    try {
      const conversationId = await invoke("generate_dialogue", { prompt });
      navigate(`/dialogue/${conversationId}`);
    } catch (err) {
      setError(String(err));
      console.log(error);
      setIsGenerating(false);
    }
  }

  return (
    <AppLayout>
      {isGenerating ? (
        <div className="home">Generating...</div>
      ) : (
        <div className="home">
          <form
            onSubmit={() => handleGenerate()}
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
      )}
    </AppLayout>
  );
}
