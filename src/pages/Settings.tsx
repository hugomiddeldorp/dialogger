import { useNavigate } from "react-router";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import AppLayout from "../layouts/AppLayout.tsx";

export default function Settings() {
  const navigate = useNavigate();
  const [apiKey, setApiKey] = useState("");

  async function onInputChange(e) {
    setApiKey(e.target.value);
  }

  async function handleSave(e) {
    e.preventDefault();
    try {
      // TODO: add popover for success
      await invoke<string>("save_api_key", {
        keyString: apiKey,
      });
      navigate("/");
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <AppLayout>
      <div className="settings">
        <form onSubmit={handleSave} method="" accept-charset="utf-8">
          <input
            type="text"
            name="api_key"
            id="api_key"
            placeholder="API Key"
            value={apiKey}
            onChange={onInputChange}
          />
          <button type="submit">&gt;</button>
        </form>
      </div>
    </AppLayout>
  );
}
