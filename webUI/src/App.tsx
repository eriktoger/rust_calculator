import { FormEvent, StrictMode, useState } from "react";

const getAnswer = async (question: string) => {
  try {
    const response = await fetch("http://localhost:8000", {
      method: "POST",
      body: question,
    });
    if (response.ok) {
      return response.text();
    }
    return "Unknown Error";
  } catch (error) {
    console.log("error", error);
    return "Unknown Error";
  }
};

function App() {
  const [question, setQuestion] = useState("");
  const [answer, setAnswer] = useState("");
  const onSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    getAnswer(question).then((data) => setAnswer(data));
  };
  const message = answer.includes("Error") ? answer : `Answer: ${answer}`;
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
      <div>{message} </div>
    </StrictMode>
  );
}

export default <App />;
