import { SimpleGrid, HStack, Text } from "@chakra-ui/react";
import { Card } from "../../shared/components/Card";
import { Layout } from "../../shared/components/Layout";
import { usePoll } from "./usePoll.logic";
import { DateTime } from "luxon";

export default function Poll() {
  const {
    state: { poll },
  } = usePoll();

  let date: string = "";

  if (poll?.created_at) {
    date = DateTime.fromMillis(poll.created_at / 1000000).toFormat("dd LLL, yyyy");
  }

  const renderCandidates = () => {
    return poll?.candidates.map((candidate) => (
      <Card key={candidate.name.concat(candidate.party_name)} name={candidate.name} partyName={candidate.party_name} showResult={false} />
    ));
  };

  return (
    <Layout>
      <HStack>
        <Text fontSize="xl">Created On: </Text>
        <Text fontWeight="bold" fontSize="xl">
          {date}
        </Text>
      </HStack>
      <SimpleGrid columns={3} spacing={10}>
        {renderCandidates()}
      </SimpleGrid>
    </Layout>
  );
}
