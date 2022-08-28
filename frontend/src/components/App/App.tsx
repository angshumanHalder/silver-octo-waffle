import React, { useEffect } from "react";
import "regenerator-runtime/runtime";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Polls from "../Polls/Polls";
import { CreatePoll } from "../CreatePoll/CreatePoll";
import Poll from "../Poll/Poll";
import { generateBallot, generateCandidate, generateSignature, generateVoter } from "../../utils/ring-sig";

export default function App() {
  useEffect(() => {
    const v1 = generateVoter();
    const v2 = generateVoter();
    const v3 = generateVoter();
    const v4 = generateVoter();

    const ring = [v2.public, v3.public, v4.public];

    const c = generateCandidate();
    const shared = generateCandidate();

    const ballot = generateBallot(shared.public, c.public);
    const signature = generateSignature(v1.secret, v1.public, v1.image, ring, 1, ballot);

    console.log("shared ", shared);
    console.log("signature", signature);
    console.log("ballot", ballot);

    console.log("v1", v1);
    console.log("v2", v2);
    console.log("v3", v3);
    console.log("v4", v4);

    console.log("candidate", c);
  });
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
