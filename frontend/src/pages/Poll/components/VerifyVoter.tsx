import { Button } from "@chakra-ui/button";
import { FormControl, FormErrorMessage, FormLabel } from "@chakra-ui/form-control";
import { Input } from "@chakra-ui/input";
import { Box, Text } from "@chakra-ui/layout";
import { Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay } from "@chakra-ui/modal";
import { Spinner } from "@chakra-ui/react";
import { Field, Form, Formik } from "formik";
import { useVerifyVoter } from "../hooks/useVerifyVoter";

interface VerifyVoterProps {
  isOpen: boolean;
  onClose: () => void;
}

const VerifyVoter: React.FC<VerifyVoterProps> = ({ isOpen, onClose }) => {
  const {
    state: { VerifyVoterSchema, passphrase, isVerified, loading },
    actions: { verifyVoterHandler, addVoterHandler },
    reducers: { setPassphrase },
  } = useVerifyVoter();

  return (
    <Modal
      isOpen={isOpen}
      onClose={() => {
        onClose();
        setPassphrase([]);
      }}
    >
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Add Voter</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          <Formik initialValues={{ id: "" }} validationSchema={VerifyVoterSchema} onSubmit={verifyVoterHandler}>
            {({ errors, touched }) => (
              <Box mb={4}>
                <Form>
                  <FormControl mb={3} isInvalid={Boolean(touched.id && errors.id)}>
                    <FormLabel htmlFor="voter-id">Enter Voter Id</FormLabel>
                    <Field id="voter-id" placeholder="Enter voter id to verify" name="id" as={Input} />
                    <FormErrorMessage>{errors.id}</FormErrorMessage>
                  </FormControl>
                  <Button w="100%" colorScheme="orange" type="submit" disabled={loading}>
                    {!loading ? "Verify" : <Spinner size="md" />}
                  </Button>
                </Form>
              </Box>
            )}
          </Formik>
          {passphrase.length > 0 && (
            <>
              <Box mb={3}>
                <Input value={passphrase.join(" ")} readOnly />
              </Box>
              <Text color="red.500">Copy the secret words. If you loose it you will not be able to vote.</Text>
            </>
          )}
        </ModalBody>

        <ModalFooter>
          <Button
            colorScheme="telegram"
            mr={3}
            onClick={() => {
              addVoterHandler();
              setPassphrase([]);
              onClose();
            }}
            disabled={!isVerified}
          >
            Add
          </Button>
          <Button colorScheme="red">Cancel</Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export default VerifyVoter;
