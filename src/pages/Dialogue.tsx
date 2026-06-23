import { Link } from "react-router";

import "./Dialogue.css";

import AppLayout from "../layouts/AppLayout.tsx";
import DialogueLine from "../components/DialogueLine.tsx";

function Dialogue() {
  const dialogue = [
    "Hello",
    "Hi, stranger",
    "É oui, ßiençur monşiàr !",
    "Yes!",
  ];
  const authors = ["Don", "Pedro"];
  return (
    <AppLayout>
      <div className="dialogue">
        {dialogue.map((text, idx) => (
          <DialogueLine author={authors[idx % 2]} text={text} />
        ))}
        <Link to="/"> Back</Link>
      </div>
    </AppLayout>
  );
}

export default Dialogue;
