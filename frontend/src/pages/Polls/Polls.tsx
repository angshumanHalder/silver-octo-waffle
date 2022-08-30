import { Button, Table, TableContainer, Tbody, Td, Th, Thead, Tr } from "@chakra-ui/react";
import { Layout } from "../../shared/components/Layout";
import { usePolls } from "./usePolls.logic";

export default function Polls() {
  const {
    state: { polls },
    actions: { onPollClickHandler },
  } = usePolls();
  console.log(polls);

  const renderRows = () => {
    return polls.map((poll) => (
      <Tr key={poll.poll_id}>
        <Td>{poll.poll_id}</Td>
        <Td>{poll.poll_status}</Td>
        <Td>
          <Button variant="solid" colorScheme="telegram" onClick={() => onPollClickHandler(poll.poll_id)}>
            Go to Poll
          </Button>
        </Td>
      </Tr>
    ));
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
          <Tbody>{renderRows()}</Tbody>
        </Table>
      </TableContainer>
    </Layout>
  );
}
