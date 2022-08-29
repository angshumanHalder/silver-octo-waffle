import React from "react";
import { Heading, Avatar, Box, Center, Text, Stack, Button, Link, Badge, useColorModeValue, HStack } from "@chakra-ui/react";

interface CardProps {
  name: String;
  partyName: String;
  showResult: boolean;
  votes?: number;
}

export const Card: React.FC<CardProps> = ({ name, partyName, showResult }) => {
  return (
    <Center py={6}>
      <Box maxW={"320px"} w={"full"} bg={useColorModeValue("white", "gray.700")} boxShadow={"2xl"} rounded={"lg"} p={6} textAlign={"center"}>
        <Avatar size={"xl"} name={name} mb={4} pos={"relative"} />
        <Heading fontSize={"2xl"} fontFamily={"body"}>
          {name}
        </Heading>
        <Text fontWeight={600} color={"gray.400"} mb={4}>
          {partyName}
        </Text>
        {showResult ? (
          <HStack justifyContent="center">
            <Text fontSize={"lg"}>Votes: </Text>
            <Text fontSize={"lg"} fontWeight="bold">
              600
            </Text>
          </HStack>
        ) : (
          <Button flex={1} fontSize={"sm"} colorScheme="telegram" boxShadow={"0px 1px 25px -5px rgb(66 153 225 / 48%), 0 10px 10px -5px rgb(66 153 225 / 43%)"}>
            Vote
          </Button>
        )}
      </Box>
    </Center>
  );
};
