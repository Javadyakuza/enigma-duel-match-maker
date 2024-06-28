const { SigningArchwayClient } = require("@archwayhq/arch3.js");
const { GasPrice } = require("@cosmjs/stargate");
const { Buffer } = require("buffer");
const process = require("process");
const addresses = require("./constants");
// const mockQuestions = require("./mock_questions.json");
const axios = require("axios");
const qs = require("qs");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");

global.Buffer = Buffer;
global.process = process;
require("dotenv").config();

async function create_match(contestant1, contestant2, prizePool) {
  console.log(contestant1, contestant2, prizePool);
  const network = {
    endpoint: "https://rpc.constantine.archway.io",
    prefix: "archway",
  };

  const walletMnemonic = process.env.ADMIN_MENMONIC;
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(walletMnemonic, {
    prefix: network.prefix,
  });
  const admin = await SigningArchwayClient.connectWithSigner(
    network.endpoint,
    wallet
  );

  const gasPrice = GasPrice.fromString("0.02aconst");
  if (!gasPrice) {
    throw new Error("Failed to initialize gas price");
  }

  const account = await wallet.getAccounts();
  console.log(account);

  let create_game_room = {
    create_game_room: {
      game_room_init_params: {
        contestant1: `${contestant1}`,
        contestant2: `${contestant2}`,
        prize_pool: `${prizePool}`,
        status: {
          started: {},
        },
      },
    },
  };
  admin
    .execute(
      account[0].address,
      addresses.enigma_duel,
      create_game_room,
      "auto",
      "decreasing the balance",
      [{ denom: "aconst", amount: "100000000000000000" }]
    )
    .then((res) => {
      console.log(res);
    })
    .catch((err) => {
      console.log(err);
    });
}

const args = process.argv.slice(2);
const contestant1 = args[0];
const contestant2 = args[1];
const prizePool = args[2];

create_match(contestant1, contestant2, prizePool)
  .then((result) => {
    if (result) {
      console.log(result);
    }
  })
  .catch((err) => {
    console.log(err);
  });
