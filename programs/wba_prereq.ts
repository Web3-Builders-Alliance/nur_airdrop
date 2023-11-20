export type WbaPrereq = {
  version: string;
  name: string;
  instructions: Instruction[];
  accounts: Account[];
  errors: Error[];
};

type Instruction = {
  name: string;
  accounts: InstructionAccount[];
  args: Argument[];
};

type InstructionAccount = {
  name: string;
  isMut: boolean;
  isSigner: boolean;
};

type Argument = {
  name: string;
  type: string;
};

type Account = {
  name: string;
  type: {
    kind: string;
    fields: Field[];
  };
};

type Field = {
  name: string;
  type: string;
};

type Error = {
  code: number;
  name: string;
  msg: string;
};

export const IDL: WbaPrereq = {
  version: "0.1.0",
  name: "wba_prereq",
  instructions: [
    {
      name: "complete",
      accounts: [
        { name: "signer", isMut: true, isSigner: true },
        { name: "prereq", isMut: true, isSigner: false },
        { name: "systemProgram", isMut: false, isSigner: false },
      ],
      args: [{ name: "github", type: "bytes" }],
    },
    {
      name: "update",
      accounts: [
        { name: "signer", isMut: true, isSigner: true },
        { name: "prereq", isMut: true, isSigner: false },
        { name: "systemProgram", isMut: false, isSigner: false },
      ],
      args: [{ name: "github", type: "bytes" }],
    },
  ],
  accounts: [
    {
      name: "PrereqAccount",
      type: {
        kind: "struct",
        fields: [
          { name: "github", type: "bytes" },
          { name: "key", type: "publicKey" },
        ],
      },
    },
  ],
  errors: [{ code: 6000, name: "InvalidGithubAccount", msg: "Invalid Github account" }],
};
