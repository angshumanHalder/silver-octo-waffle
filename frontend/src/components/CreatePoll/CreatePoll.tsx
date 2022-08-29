import { Field, FieldArray, Form, Formik, FormikErrors, FormikTouched, getIn, useFormik } from "formik";
import { Box, Button, FormControl, FormErrorMessage, FormLabel, HStack, Input, VStack } from "@chakra-ui/react";
import { Layout } from "../../shared/components/Layout";
import { AddIcon, MinusIcon } from "@chakra-ui/icons";
import { useCreatePoll } from "./useCreatePoll";

export const CreatePoll = () => {
  const {
    state: { CreatePollSchema },
    actions: { handleSubmit },
  } = useCreatePoll();

  const renderField = (
    idx: number,
    insert: (idx: number, value: any) => void,
    remove: (idx: number) => void,
    length: number,
    errors: FormikErrors<{
      candidates: {
        partyName: string;
        name: string;
      }[];
    }>,
    touched: FormikTouched<{
      candidates: { partyName: string; name: string }[];
    }>
  ) => {
    const partyError = getIn(errors, `candidates.${idx}.partyName`);
    const partyTouch = getIn(touched, `candidates.${idx}.partyName`);
    const partyErrorMessage = partyTouch && partyError ? partyError : null;

    const candidateError = getIn(errors, `candidates.${idx}.name`);
    const candidateTouch = getIn(touched, `candidates.${idx}.name`);
    const candidateErrorMessage = candidateTouch && candidateError ? candidateError : null;

    return (
      <HStack key={idx} alignItems="flex-end" mt={4}>
        <FormControl isInvalid={partyErrorMessage}>
          <FormLabel htmlFor="partyName">Party Name</FormLabel>
          <Field name={`candidates.${idx}.partyName`} as={Input} type="text" id="partyName" />
          <FormErrorMessage>{partyErrorMessage}</FormErrorMessage>
        </FormControl>
        <FormControl isInvalid={candidateErrorMessage}>
          <FormLabel htmlFor="name">Candidate Name</FormLabel>
          <Field name={`candidates.${idx}.name`} as={Input} id="name" type="text" />
          <FormErrorMessage>{candidateErrorMessage}</FormErrorMessage>
        </FormControl>
        {idx == length - 1 ? (
          <Button size="md" colorScheme="telegram" onClick={() => insert(idx + 1, { partyName: "", name: "" })}>
            <AddIcon />
          </Button>
        ) : (
          <Button size="md" colorScheme="telegram" onClick={() => remove(idx)}>
            <MinusIcon />
          </Button>
        )}
      </HStack>
    );
  };

  return (
    <Layout>
      <Formik initialValues={{ candidates: [{ partyName: "", name: "" }] }} validationSchema={CreatePollSchema} onSubmit={handleSubmit}>
        {({ errors, touched, values }) => {
          return (
            <Form>
              <Box mb={5}>
                <FieldArray
                  name="candidates"
                  render={(arrayHelpers) => (
                    <Box width="100%">
                      {values.candidates &&
                        values.candidates.length > 0 &&
                        values.candidates.map((_, idx) =>
                          renderField(idx, arrayHelpers.insert, arrayHelpers.remove, values.candidates.length, errors, touched)
                        )}
                    </Box>
                  )}
                />
              </Box>
              <Button width="100%" colorScheme="telegram" type="submit">
                Create Poll
              </Button>
            </Form>
          );
        }}
      </Formik>
    </Layout>
  );
};
