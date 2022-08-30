import { Button } from "@chakra-ui/button";
import { FormControl, FormErrorMessage, FormLabel } from "@chakra-ui/form-control";
import { Input } from "@chakra-ui/input";
import { Box, Text } from "@chakra-ui/layout";
import { Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay } from "@chakra-ui/modal";

interface VerifyVoterProps {
  isOpen: boolean;
  onClose: () => void;
  invalidText: String | undefined;
  verifyVoterHandler: () => void;
  setVoterId: React.Dispatch<React.SetStateAction<string>>;
  voterVerified: boolean;
  voterId: string;
}

const VerifyVoter: React.FC<VerifyVoterProps> = ({ isOpen, onClose, invalidText, verifyVoterHandler, setVoterId, voterVerified, voterId }) => {
  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Add Voter</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          <Box mb={4}>
            <FormControl id="voter-id" isRequired mb={3} isInvalid={!!invalidText}>
              <FormLabel>Enter Voter Id</FormLabel>
              <Input placeholder="Enter voter id to verify" value={voterId} onChange={(e) => setVoterId((e.target as HTMLInputElement).value)} />
              <FormErrorMessage>{invalidText}</FormErrorMessage>
            </FormControl>
            <Button w="100%" colorScheme="orange" onClick={verifyVoterHandler}>
              Verify
            </Button>
          </Box>
          {voterVerified && (
            <Box>
              <Input disabled value="key" />
            </Box>
          )}
        </ModalBody>

        <ModalFooter>
          <Button colorScheme="telegram" mr={3} onClick={onClose}>
            Add
          </Button>
          <Button colorScheme="red">Cancel</Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export default VerifyVoter;
