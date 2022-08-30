import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useRecoilState } from "recoil";
import { getPolls } from "../../contract/silver-octo-waffle";
import { PollsAtom } from "../../shared/recoil/polls";

export const usePolls = () => {
  const navigate = useNavigate();

  const [polls, setPolls] = useRecoilState(PollsAtom);

  const onPollClickHandler = (poll_id: number) => {
    navigate(`/poll/${poll_id}`);
  };

  useEffect(() => {
    getAllPolls();
  }, []);

  const getAllPolls = async () => {
    const polls = await getPolls();
    setPolls(polls);
  };

  return {
    state: { polls },
    actions: { onPollClickHandler },
  };
};
