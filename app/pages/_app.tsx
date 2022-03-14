import React from "react";
// import '../styles/index.css'
import {WalletAdapterNetwork} from "@solana/wallet-adapter-base";
import {ConnectionProvider, WalletProvider} from "@solana/wallet-adapter-react";
import {WalletModalProvider} from "@solana/wallet-adapter-react-ui";
import {
  LedgerWalletAdapter,
  PhantomWalletAdapter,
  SlopeWalletAdapter,
  SolflareWalletAdapter,
  SolletExtensionWalletAdapter,
  SolletWalletAdapter,
  TorusWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import {clusterApiUrl} from "@solana/web3.js";
import {AppProps} from "next/app";
import {FC, useMemo} from "react";
import Image from "next/image";
import logo from "../public/logo_clear.png";
import LinkButton from "../components/LinkButton";
import {WalletMultiButton} from "@solana/wallet-adapter-react-ui";
// Use require instead of import since order matters
require("@solana/wallet-adapter-react-ui/styles.css");
require("../styles/globals.css");

// const logo = require("./logo.png");
const MyApp = ({Component, pageProps}: AppProps) => {
  // Can be set to 'devnet', 'testnet', or 'mainnet-beta'
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  // @solana/wallet-adapter-wallets includes all the adapters but supports tree shaking and lazy loading --
  // Only the wallets you configure here will be compiled into your application, and only the dependencies
  // of wallets that your users connect to will be loaded
  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SlopeWalletAdapter(),
      new SolflareWalletAdapter({network}),
      new TorusWalletAdapter(),
      new LedgerWalletAdapter(),
      new SolletWalletAdapter({network}),
      new SolletExtensionWalletAdapter({network}),
    ],
    [network]
  );

  return (
    <div className="m-2 text-white border border-white">
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <WalletModalProvider>
            <div className="flex flex-row justify-between">
              <div className="flex flex-row items-end space-x-2">
                <Image src={logo} alt="Logo" width="64" height="64" />
                <h1 className="text-4xl ">Ennector</h1>
              </div>
              <div className="flex flex-row space-x-4 items-center">
                <LinkButton name="Home" link="/" />
                <LinkButton name="Create" link="/create" />
                <LinkButton name="Invest" link="/invest" />
              </div>
              <WalletMultiButton />
            </div>

            <Component {...pageProps} />
          </WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    </div>
  );
};

export default MyApp;
