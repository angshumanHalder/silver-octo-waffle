import { SimpleGrid, HStack, Text, Flex, Box, Button } from "@chakra-ui/react";
import { Card } from "../../shared/components/Card";
import { Layout } from "../../shared/components/Layout";
import { usePoll } from "./usePoll.logic";
import { DateTime } from "luxon";
import VerifyVoter from "./components/VerifyVoter";

export const Poll = () => {
  const {
    state: { poll, isOpen, voterId, invalidText, voterVerified },
    actions: { addVoterHandler, verifyVoterHandler },
    reducers: { onOpen, onClose, setVoterId },
  } = usePoll();

  let date: string = "";

  if (poll?.created_at) {
    date = DateTime.fromMillis(poll.created_at / 1000000).toFormat("dd LLL, yyyy");
  }

  const renderCandidates = () => {
    return poll?.candidates.map((candidate) => (
      <Card key={candidate.name.concat(candidate.party_name.toString())} name={candidate.name} partyName={candidate.party_name} showResult={false} />
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
        <Box>
          <Button colorScheme="telegram" onClick={onOpen}>
            Add Voter
          </Button>
          <Button colorScheme="red" ml={2}>
            End Poll
          </Button>
        </Box>
      </Flex>
      <SimpleGrid columns={3} spacing={10}>
        {renderCandidates()}
      </SimpleGrid>
      <VerifyVoter
        isOpen={isOpen}
        onClose={onClose}
        invalidText={invalidText}
        verifyVoterHandler={verifyVoterHandler}
        setVoterId={setVoterId}
        voterVerified={voterVerified}
        voterId={voterId}
      />
    </Layout>
  );
};
