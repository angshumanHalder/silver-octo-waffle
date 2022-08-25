import React from "react";
import { Field, FieldArray, Form, Formik, useFormik } from "formik";
import { Box, Button, FormControl, FormLabel, HStack, Input, VStack } from "@chakra-ui/react";
import { stringify } from "querystring";
import { Layout } from "../../shared/components/Layout";
import { AddIcon, MinusIcon } from "@chakra-ui/icons";

export const CreatePoll = () => {
  const renderField = (idx: number, insert: (idx: number, value: any) => void, remove: (idx: number) => void, length: number) => {
    return (
      <HStack key={idx} alignItems="flex-end" mt={4}>
        <FormControl>
          <FormLabel htmlFor="partyName">Party Name</FormLabel>
          <Field name={`candidates.${idx}.partyName`} as={Input} type="text" id="partyName" />
        </FormControl>
        <FormControl>
          <FormLabel htmlFor="name">Candidate Name</FormLabel>
          <Field name={`candidates.${idx}.name`} as={Input} id="name" type="text" />
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
      <Formik initialValues={{ candidates: [{ partyName: "", name: "" }] }} onSubmit={(values) => console.log(JSON.stringify(values))}>
        {({ errors, touched, values }) => (
          <Form>
            <Box mb={5}>
              <FieldArray
                name="candidates"
                render={(arrayHelpers) => (
                  <Box width="100%">
                    {values.candidates &&
                      values.candidates.length > 0 &&
                      values.candidates.map((_, idx) => renderField(idx, arrayHelpers.insert, arrayHelpers.remove, values.candidates.length))}
                  </Box>
                )}
              />
            </Box>
            <Button width="100%" colorScheme="telegram" type="submit">
              Create Poll
            </Button>
          </Form>
        )}
      </Formik>
    </Layout>
  );
};
