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

async function finish_match(game_room_key, winner) {
  console.log(addresses);
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
  let result;
  if (winner == "") {
    result = { draw: {} };
  } else {
    result = { win: { addr: `${winner}` } };
  }
  let finish_game_room = {
    finish_game_room: {
      game_room_finish_params: {
        game_room_id: `${game_room_key}`,
        result: result,
      },
    },
  };
  admin
    .execute(
      account[0].address,
      addresses.enigma_duel,
      finish_game_room,
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
const game_room_key = args[0];
const winner = args[1];

finish_match(game_room_key, winner).then((result) => {
  if (result) {
    console.log(result);
  }
});
