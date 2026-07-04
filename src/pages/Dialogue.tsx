import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Link, useParams } from "react-router";

import "./Dialogue.css";

import AppLayout from "../layouts/AppLayout.tsx";
import DialogueLine from "../components/DialogueLine.tsx";

interface DialogueInterface {
  title: string;
  people: DialogueParticipant[];
  dialogue: string[];
}

export interface DialogueParticipant {
  name: string;
  voice: string;
}

function Dialogue() {
  const params = useParams();
  const conversationId = params.conversationId;

  const [dialogue, setDialogue] = useState<DialogueInterface | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    invoke<DialogueInterface>("get_dialogue", {
      conversationId: conversationId,
    })
      .then(setDialogue)
      .catch((e) => setError(String(e)));
  }, [conversationId]);

  // TODO: I'm not sure if this is the best way to do it
  if (!dialogue)
    return (
      <AppLayout>
        <div className="dialogue">Loading...</div>
      </AppLayout>
    );

  return (
    <AppLayout>
      <div className="dialogue">
        {dialogue.dialogue.map((text, idx) => (
          <DialogueLine author={dialogue.people[idx % 2]} text={text} />
        ))}
        <Link to="/"> Back</Link>
      </div>
    </AppLayout>
  );
}

export default Dialogue;
