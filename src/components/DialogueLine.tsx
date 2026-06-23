import "./DialogueLine.css";

export default function DialogueLine({
  author,
  text,
}: {
  author: string;
  text: string;
}) {
  return (
    <div className="dialogueLine">
      <h2>{author}</h2>
      <p>{text}</p>
    </div>
  );
}
