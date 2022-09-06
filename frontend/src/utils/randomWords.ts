import Jabber from "jabber";

export const generateRandomWords = (): string[] => {
  const jabber = new Jabber();
  const output: string[] = [];
  for (let i = 0; i < 8; i++) {
    output.push(jabber.createWord(4));
  }
  return output;
};
