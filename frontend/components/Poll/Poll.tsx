import { HStack, Text } from "@chakra-ui/react";
import React from "react";
import { Layout } from "../../shared/components/Layout";

export default function Poll() {
  return (
    <Layout>
      <HStack>
        <Text fontSize="xl">Started On: </Text>
        <Text fontWeight="bold" fontSize="xl">
          24th Aug, 2022
        </Text>
      </HStack>
    </Layout>
  );
}
