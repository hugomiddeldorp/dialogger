import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Link } from "react-router";
import "./Menu.css";

export default function Menu({
  isOpen,
  onClose,
}: {
  isOpen: boolean;
  onClose: () => void;
}) {
  const [conversations, setConversations] = useState([]);
  const [error, setError] = useState("");

  useEffect(() => {
    invoke("get_conversations")
      .then(setConversations)
      .catch((e) => setError(String(e)));
  }, []);

  return (
    <aside className={`menu ${isOpen ? "open" : ""}`}>
      <div className="head">
        <i onClick={onClose} className="icon icon-close"></i>
        <Link to="/">
          <i className="icon icon-home"></i>
        </Link>
      </div>
      <h1>Previous conversations</h1>
      <ul>
        {conversations.map((entry) => (
          <li>
            <Link to={`/dialogue/${entry.uuid}`} reloadDocument>
              {entry.title}
            </Link>
          </li>
        ))}
      </ul>
      <Link to="/settings">
        <div className="settings-button">
          <i className="icon icon-settings"></i>Settings
        </div>
      </Link>
    </aside>
  );
}
