import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

require('dotenv').config();

const {RPC_URL, PRIVATE_KEY} = process.env;

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  defaultNetwork: "testnet",
  networks: {
    testnet: {
      chainId: 11155111,
      url: RPC_URL,
      accounts: [PRIVATE_KEY],
      timeout: 200000000,
      allowUnlimitedContractSize: true,
    }
  }
};

export default config;
