import { atom } from "recoil";

export const PollsAtom = atom<Poll[]>({
  key: "polls",
  default: [],
});
