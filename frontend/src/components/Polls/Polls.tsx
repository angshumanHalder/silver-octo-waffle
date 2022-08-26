import { Button, Table, TableContainer, Tbody, Td, Th, Thead, Tr } from "@chakra-ui/react";
import React from "react";
import { useNavigate } from "react-router-dom";
import { Layout } from "../../shared/components/Layout";

export default function Polls() {
  const navigate = useNavigate();

  const onPollClickHandler = (poll_id: number) => {
    navigate(`/poll/${poll_id}`);
  };

  const renderRow = () => {
    return (
      <Tr>
        <Td>0</Td>
        <Td>test</Td>
        <Td>
          <Button variant="solid" colorScheme="telegram" onClick={() => onPollClickHandler(0)}>
            Go to Poll
          </Button>
        </Td>
      </Tr>
    );
  };

  return (
    <Layout>
      <TableContainer>
        <Table variant="striped">
          <Thead>
            <Tr>
              <Th>Poll Id</Th>
              <Th>Poll Status</Th>
              <Th>Action</Th>
            </Tr>
          </Thead>
          <Tbody>{renderRow()}</Tbody>
        </Table>
      </TableContainer>
    </Layout>
  );
}
