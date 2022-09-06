import { FormControl, FormErrorMessage, FormLabel } from "@chakra-ui/form-control";
import { Box } from "@chakra-ui/layout";
import { Modal, ModalBody, ModalCloseButton, ModalContent, ModalHeader, ModalOverlay } from "@chakra-ui/modal";
import { Field, Form, Formik } from "formik";
import { Button } from "@chakra-ui/button";
import { IVoteCandidateSchema } from "../hooks/usePoll";
import { SchemaOf } from "yup";
import { Input, Spinner } from "@chakra-ui/react";

interface VoteCandidateProps {
  voteModalOpen: boolean;
  onVoteModalClose: () => void;
  VoteCandidateSchema: SchemaOf<IVoteCandidateSchema>;
  onVoteHandler: (values: { passphrase: string }) => Promise<void>;
  voting: boolean;
}

export const VoteCandidate: React.FC<VoteCandidateProps> = ({ voteModalOpen, onVoteModalClose, VoteCandidateSchema, onVoteHandler, voting }) => {
  return (
    <Modal
      isOpen={voteModalOpen}
      onClose={() => {
        onVoteModalClose();
      }}
    >
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Vote Candidate</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          <Formik initialValues={{ passphrase: "" }} validationSchema={VoteCandidateSchema} onSubmit={onVoteHandler}>
            {({ errors, touched }) => (
              <Box mb={4}>
                <Form>
                  <FormControl mb={3} isInvalid={Boolean(touched.passphrase && errors.passphrase)}>
                    <FormLabel htmlFor="secret">Enter Secret Words</FormLabel>
                    <Field id="secret" placeholder="Enter secret words to vote" name="passphrase" as={Input} />
                    <FormErrorMessage>{errors.passphrase}</FormErrorMessage>
                  </FormControl>
                  <Button w="100%" colorScheme="orange" type="submit" disabled={voting}>
                    {!voting ? "Vote" : <Spinner size="md" />}
                  </Button>
                </Form>
              </Box>
            )}
          </Formik>
        </ModalBody>
      </ModalContent>
    </Modal>
  );
};
