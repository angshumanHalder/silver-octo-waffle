import * as Yup from "yup";
import { generateCandidate } from "../../utils/ring-sig";
import { createPoll } from "../../contract/silver-octo-waffle";
import { useNavigate } from "react-router-dom";

export const useCreatePoll = () => {
  const navigate = useNavigate();

  const CreatePollSchema = Yup.object().shape({
    candidates: Yup.array().of(
      Yup.object().shape({
        partyName: Yup.string().required("Party Name is required"),
        name: Yup.string().required("Candidate Name is required"),
      })
    ),
  });

  const handleSubmit = async (values: { candidates: { partyName: string; name: string }[] }) => {
    const shared = generateCandidate();
    const candidateList = values.candidates.map((candidate) => {
      const c = {
        name: candidate.name,
        party_name: candidate.partyName,
        public_key: generateCandidate().public,
      };
      return c;
    });
    try {
      await createPoll({ candidates: candidateList, shared_sk: shared.secret, shared_pk: shared.public });
      navigate("/", { replace: true });
    } catch (err) {
      console.log(err);
    }
  };

  return {
    state: { CreatePollSchema },
    actions: { handleSubmit },
  };
};
