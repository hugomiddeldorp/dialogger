import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import type { DialogueParticipant } from "../pages/Dialogue.tsx";
import "./DialogueLine.css";

enum SpeakingState {
  Idle,
  Loading,
  Playing,
}

export default function DialogueLine({
  author,
  text,
}: {
  author: DialogueParticipant;
  text: string;
}) {
  const [speakingState, setSpeakingState] = useState(SpeakingState.Idle);

  async function handleSpeak(text: string, voice: string) {
    // TODO: Implement multi-speaker
    // TODO: Future improvement: cache the response to replay without regenerating
    // TODO: Call is not exclusive, probably should be

    setSpeakingState(SpeakingState.Loading);
    const wavBytes = await invoke<number[]>("speak", {
      text: text,
      voice: voice,
    });
    const arrayBuffer = new Uint8Array(wavBytes).buffer;

    const ctx = new AudioContext();
    const audioBuffer = await ctx.decodeAudioData(arrayBuffer);

    const source = ctx.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(ctx.destination);

    source.onended = () => setSpeakingState(SpeakingState.Idle);

    setSpeakingState(SpeakingState.Playing);
    source.start();
  }

  function styleSpeakingState(state: SpeakingState) {
    if (state == SpeakingState.Loading) return "loading";
    if (state == SpeakingState.Playing) return "playing";
    return "";
  }

  return (
    <div className="dialogueLine">
      <h2>{author.name}</h2>
      <p onClick={() => handleSpeak(text, author.voice)}>
        <span className={styleSpeakingState(speakingState)}>{text}</span>
      </p>
    </div>
  );
}
