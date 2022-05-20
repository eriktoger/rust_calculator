import { FormEvent, StrictMode, useState } from "react";

function App() {
  const [question, setQuestion] = useState("");
  const [answer, setAnswer] = useState("");
  const onSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    fetch("http://localhost:8000", {
      method: "POST",
      body: question,
    }).then((res) => res.json().then((data) => setAnswer(data)));
  };

  return (
    <StrictMode>
      <div> Enter math questions:</div>
      <form onSubmit={onSubmit}>
        <input
          value={question}
          onChange={(event) => setQuestion(event.target.value)}
        />
        <input type="submit" value="Submit" />
      </form>
      <div>Answer: {answer} </div>
    </StrictMode>
  );
}

export default <App />;
