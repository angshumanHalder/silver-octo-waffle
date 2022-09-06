import { atom } from "recoil";

export const PollsAtom = atom<Poll[]>({
  key: "polls",
  default: [],
});

export const PollAtom = atom<Poll | undefined>({
  key: "poll",
  default: undefined,
});
