import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useRecoilValue } from "recoil";
import { PollsAtom } from "../../shared/recoil/polls";

export const usePoll = () => {
  const polls = useRecoilValue(PollsAtom);
  const params = useParams();

  const [poll, setPoll] = useState<Poll | undefined>(undefined);

  useEffect(() => {
    if (polls.length > 0) {
      const poll = polls.find((poll) => poll.poll_id === parseInt(params.id!));
      setPoll(poll);
    }

    return () => {
      setPoll(undefined);
    };
  }, [polls]);

  return {
    state: { poll },
  };
};
