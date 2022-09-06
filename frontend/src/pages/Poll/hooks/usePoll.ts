import { useDisclosure } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useRecoilState } from "recoil";
import * as Yup from "yup";
import { endPoll, getPoll, getRing, startPoll, tally, vote } from "../../../contract/silver-octo-waffle";
import { PollAtom } from "../../../shared/recoil/polls";
import { generateRandomNumber } from "../../../utils/randomNumber";
import { generateBallot, generateKeysFromSecret, generateSignature } from "../../../utils/ring-sig";

export interface IVoteCandidateSchema {
  passphrase: string;
}

export const usePoll = () => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { isOpen: voteModalOpen, onOpen: onVoteModalOpen, onClose: onVoteModalClose } = useDisclosure();

  const VoteCandidateSchema: Yup.SchemaOf<IVoteCandidateSchema> = Yup.object().shape({
    passphrase: Yup.string().length(39, "Please check your secret words").required(),
  });

  const params = useParams();

  const [poll, setPoll] = useRecoilState(PollAtom);

  const [currentCandidate, setCurrentCandidate] = useState<Array<number> | null>(null);
  const [voting, setVoting] = useState(false);
  const [tallying, setTallying] = useState(false);
  const [startingOrEndingPoll, setStartingOrEndingPoll] = useState(false);

  useEffect(() => {
    (async () => {
      const poll = await getPoll({ poll_id: parseInt(params.id!) });
      console.log(poll);
      setPoll(poll);
    })();
    return () => {
      setPoll(undefined);
    };
  }, []);

  const onVoteHandler = async (values: { passphrase: string }) => {
    setVoting(true);
    let randomNums: Set<number> = new Set();
    while (randomNums.size < 3) {
      randomNums.add(generateRandomNumber(0, poll?.voters!));
    }

    let rngArr: number[] = Array.from(randomNums);
    const passphrase = values.passphrase.split(" ");
    const encodedPassphrase = new TextEncoder().encode(passphrase.join(""));
    const voter = generateKeysFromSecret(Array.from(encodedPassphrase));

    const ballot = generateBallot(poll?.shared_pk!, currentCandidate!);
    const ring = await getRing({ poll_id: poll?.poll_id!, rng1: rngArr[0], rng2: rngArr[1], rng3: rngArr[2] });

    const signature = generateSignature(voter.secret, voter.public, voter.image, ring, generateRandomNumber(0, 4), ballot);

    await vote({ poll_id: poll?.poll_id!, ballot, signature });
    setVoting(false);
  };

  const startPollHandler = async () => {
    setStartingOrEndingPoll(true);
    await startPoll({ poll_id: poll?.poll_id! });
    const updatedPoll = await getPoll({ poll_id: poll?.poll_id! });
    setPoll(updatedPoll);
    setStartingOrEndingPoll(false);
  };

  const endPollHandler = async () => {
    setStartingOrEndingPoll(true);
    await endPoll({ poll_id: poll?.poll_id! });
    const updatedPoll = await getPoll({ poll_id: poll?.poll_id! });
    setPoll(updatedPoll);
    setStartingOrEndingPoll(false);
  };

  const onTallyHandler = async () => {
    setTallying(true);
    await tally({ poll_id: poll?.poll_id! });
    const updatedPoll = await getPoll({ poll_id: poll?.poll_id! });
    setPoll(updatedPoll);
    setTallying(false);
  };

  return {
    state: { poll, isOpen, voteModalOpen, VoteCandidateSchema, voting, tallying, startingOrEndingPoll },
    actions: { onVoteHandler, startPollHandler, endPollHandler, onOpen, onClose, onVoteModalOpen, onVoteModalClose, onTallyHandler },
    reducers: { setCurrentCandidate },
  };
};
