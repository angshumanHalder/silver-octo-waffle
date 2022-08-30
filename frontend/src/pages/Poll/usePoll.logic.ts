import { useDisclosure } from "@chakra-ui/hooks";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useRecoilValue } from "recoil";
import { PollsAtom } from "../../shared/recoil/polls";
import { arrayIntToHexStr, hexStrToArrayInt } from "../../utils/conversion";
import { generateKeysFromSecret, generateVoter } from "../../utils/ring-sig";

export const usePoll = () => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const polls = useRecoilValue(PollsAtom);
  const params = useParams();

  const [voterId, setVoterId] = useState<string>("");
  const [invalidText, setInvalidText] = useState<string | undefined>();
  const [voterVerified, setVoterVerified] = useState(false);

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

  const addVoterHandler = () => {
    const voter = generateVoter();
    console.log(voter);
    const secret = arrayIntToHexStr(voter.secret);
    const genV = generateKeysFromSecret(hexStrToArrayInt(secret));
    console.log("genV", genV);
  };

  const verifyVoterHandler = () => {
    if (!voterId) {
      setInvalidText("Field is required");
    } else {
      setVoterVerified(true);
      // call api
    }
  };

  return {
    state: { poll, isOpen, voterId, invalidText, voterVerified },
    actions: { addVoterHandler, verifyVoterHandler },
    reducers: { onOpen, onClose, setVoterId },
  };
};

export const useVoter = () => {};
