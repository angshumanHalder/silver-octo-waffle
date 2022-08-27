import React, { useEffect } from "react";
import "regenerator-runtime/runtime";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Polls from "../Polls/Polls";
import { CreatePoll } from "../CreatePoll/CreatePoll";
import Poll from "../Poll/Poll";
import init, { genSignature, genVoter } from "ring-sig";

export default function App() {
  // if (!window.walletConnection.isSignedIn()) {
  // }

  useEffect(() => {
    (async () => {
      init().then(() => {
        const v1 = genVoter();
        const v2 = genVoter();
        const v3 = genVoter();
        const v4 = genVoter();

        console.log(v2.public);

        const ring = [v2.public, v3.public, v4.public];
        const sig = genSignature(v1.secret, v1.public, v1.image, ring, 1, "Hello");
        console.log(sig);
      });
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
