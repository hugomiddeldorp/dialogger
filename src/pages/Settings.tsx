import { useNavigate } from "react-router";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

import AppLayout from "../layouts/AppLayout.tsx";

import "./Settings.css";

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
      toast.success("API Key set successfully.");
      navigate("/");
    } catch (err) {
      toast.error("API Key could not be set.", { description: err });
    }
  }

  return (
    <AppLayout>
      <div className="home">
        <form onSubmit={handleSave} method="" accept-charset="utf-8">
          <input
            type="text"
            name="api_key"
            id="api_key"
            placeholder="API Key"
            required
            value={apiKey}
            onChange={onInputChange}
          />
          <button type="submit">
            <i className="icon icon-key"></i>
          </button>
        </form>
      </div>
    </AppLayout>
  );
}
