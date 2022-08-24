import React from "react";
// import "regenerator-runtime/runtime";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Polls from "../Polls/Polls";
import CreatePoll from "../CreatePoll/CreatePoll";
import Poll from "../Poll/Poll";

export default function App() {
  // if (!window.walletConnection.isSignedIn()) {
  // }

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
