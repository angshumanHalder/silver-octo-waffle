import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import * as Yup from "yup";
import { addVoter } from "../../../contract/silver-octo-waffle";
import { generateRandomWords } from "../../../utils/randomWords";
import { generateKeysFromSecret } from "../../../utils/ring-sig";

export const useVerifyVoter = () => {
  const VerifyVoterSchema = Yup.object().shape({
    id: Yup.string().required("Id is required"),
  });

  const [passphrase, setPassphrase] = useState<string[]>([]);
  const [isVerified, setIsVerified] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    return () => {
      setPassphrase([]);
    };
  }, []);

  const params = useParams();

  const addVoterHandler = async () => {
    setLoading(true);
    const encodedPassphrase = new TextEncoder().encode(passphrase.join(""));

    const voter = generateKeysFromSecret(Array.from(encodedPassphrase));
    await addVoter({ poll_id: parseInt(params.id!), voter: voter.public });
    setLoading(false);
  };

  const verifyVoterHandler = (values: { id: string }) => {
    // once verification is complete generate random secret passphrase
    setIsVerified(true);
    const passphrase = generateRandomWords();
    setPassphrase(passphrase);
  };

  return {
    state: { VerifyVoterSchema, passphrase, isVerified, loading },
    actions: { verifyVoterHandler, addVoterHandler },
    reducers: { setPassphrase },
  };
};
