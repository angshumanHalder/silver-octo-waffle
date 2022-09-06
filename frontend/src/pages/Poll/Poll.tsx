import { SimpleGrid, HStack, Text, Flex, Box, Button, Spinner } from "@chakra-ui/react";
import { Candidate } from "../../shared/components/Candidate";
import { Layout } from "../../shared/components/Layout";
import { usePoll } from "./hooks/usePoll";
import { DateTime } from "luxon";
import VerifyVoter from "./components/VerifyVoter";
import { VoteCandidate } from "./components/VoteCandidate";

enum PollStatus {
  REGISTERED = "REGISTERED",
  STARTED = "STARTED",
  ENDED = "ENDED",
}

export const Poll = () => {
  const {
    state: { poll, isOpen, voteModalOpen, VoteCandidateSchema, voting, tallying, startingOrEndingPoll },
    actions: { onVoteHandler, startPollHandler, endPollHandler, onOpen, onClose, onVoteModalClose, onVoteModalOpen, onTallyHandler },
    reducers: { setCurrentCandidate },
  } = usePoll();

  let date: string = "";

  if (poll?.created_at) {
    date = DateTime.fromMillis(poll.created_at / 1000000).toFormat("dd LLL, yyyy");
  }

  const renderCandidates = () => {
    return poll?.candidates.map((candidate) => (
      <Candidate
        key={candidate.name.concat(candidate.party_name.toString())}
        name={candidate.name}
        partyName={candidate.party_name}
        showResult={poll.poll_status === PollStatus.ENDED}
        onClickHandler={() => {
          onVoteModalOpen();
          setCurrentCandidate(candidate.public_key);
        }}
        disabled={poll.poll_status !== PollStatus.STARTED}
        result={poll.results[candidate.party_name]}
      />
    ));
  };

  return (
    <Layout>
      <Flex direction="row" justifyContent="space-between">
        <HStack>
          <Text fontSize="xl">Created On: </Text>
          <Text fontWeight="bold" fontSize="xl">
            {date}
          </Text>
        </HStack>
        {poll?.poll_status !== PollStatus.ENDED ? (
          <Box>
            <Button colorScheme="telegram" onClick={onOpen}>
              Add Voter
            </Button>
            <Button
              colorScheme="red"
              ml={2}
              onClick={() => {
                poll?.poll_status === PollStatus.REGISTERED ? startPollHandler() : endPollHandler();
              }}
              disabled={startingOrEndingPoll}
            >
              {!startingOrEndingPoll ? poll?.poll_status === PollStatus.REGISTERED ? "Start Poll" : "End Poll" : <Spinner size="md" />}
            </Button>
          </Box>
        ) : (
          <Box>
            <Button colorScheme="green" onClick={onTallyHandler} disabled={tallying}>
              {!tallying ? "Tally" : <Spinner />}
            </Button>
          </Box>
        )}
      </Flex>
      <SimpleGrid columns={3} spacing={10}>
        {renderCandidates()}
      </SimpleGrid>
      <VerifyVoter isOpen={isOpen} onClose={onClose} />
      <VoteCandidate
        onVoteModalClose={() => {
          onVoteModalClose();
          setCurrentCandidate(null);
        }}
        voteModalOpen={voteModalOpen}
        VoteCandidateSchema={VoteCandidateSchema}
        onVoteHandler={onVoteHandler}
        voting={voting}
      />
    </Layout>
  );
};
