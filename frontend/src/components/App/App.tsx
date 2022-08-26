import React, { useEffect } from "react";
import "regenerator-runtime/runtime";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Polls from "../Polls/Polls";
import { CreatePoll } from "../CreatePoll/CreatePoll";
import Poll from "../Poll/Poll";
import init, { greet } from "ring-sig";

export default function App() {
  // if (!window.walletConnection.isSignedIn()) {
  // }

  useEffect(() => {
    (async () => {
      await init();
      greet();
    })();
  });

  // console.log(greet());

  return (
    <Router>
      <Routes>
        <Route path="/poll/create" element={<CreatePoll />} />
        <Route path="/poll/:id" element={<Poll />} />
        <Route path="/" element={<Polls />} />
      </Routes>
    </Router>
  );
}
