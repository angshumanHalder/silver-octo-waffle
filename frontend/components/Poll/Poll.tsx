import { SpinnerIcon } from "@chakra-ui/icons";
import { SimpleGrid, HStack, Text } from "@chakra-ui/react";
import React from "react";
import { Card } from "../../shared/components/Card";
import { Layout } from "../../shared/components/Layout";

export default function Poll() {
  return (
    <Layout>
      <HStack>
        <Text fontSize="xl">Created On: </Text>
        <Text fontWeight="bold" fontSize="xl">
          24th Aug, 2022
        </Text>
      </HStack>
      <SimpleGrid columns={3} spacing={10}>
        <Card name="Tuna" partyName="Tuna Association" showResult={false} />
      </SimpleGrid>
    </Layout>
  );
}
