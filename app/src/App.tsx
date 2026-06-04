import { useEffect, useState } from "react";
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  FREIGHTER_ID,
} from "@creit.tech/stellar-wallets-kit";
import { discoverOnboarder, type OnboarderConfig } from "@theaha/trustline-onboarder";
import { ASSET, BRANDING, DISCOVER_DOMAIN, NETWORK } from "./config.js";
import { Activate } from "./components/Activate.js";

const kit = new StellarWalletsKit({
  network: NETWORK.passphrase as unknown as WalletNetwork,
  selectedWalletId: FREIGHTER_ID,
  modules: allowAllModules(),
});

export function App() {
  const [address, setAddress] = useState<string>("");
  const [config, setConfig] = useState<OnboarderConfig>(ASSET);

  // Optionally discover the onboarder config from the issuer's stellar.toml.
  useEffect(() => {
    if (!DISCOVER_DOMAIN) return;
    discoverOnboarder(DISCOVER_DOMAIN).then((c) => c && setConfig(c)).catch(() => {});
  }, []);

  // Read-only preview: ?address=G... shows activation status without a wallet.
  useEffect(() => {
    const a = new URLSearchParams(window.location.search).get("address");
    if (a && a.length === 56) setAddress(a);
  }, []);

  async function connect() {
    await kit.openModal({
      onWalletSelected: async (option) => {
        kit.setWallet(option.id);
        const { address } = await kit.getAddress();
        setAddress(address);
      },
    });
  }

  return (
    <main style={{ maxWidth: 560, margin: "8vh auto", fontFamily: "system-ui, sans-serif" }}>
      <header style={{ marginBottom: 24 }}>
        <h1 style={{ margin: 0 }}>{BRANDING.title}</h1>
        <p style={{ color: "#667" }}>{BRANDING.subtitle}</p>
      </header>

      {!address ? (
        <button onClick={() => void connect()} style={btn(BRANDING.accent)}>
          Connect wallet
        </button>
      ) : (
        <Activate
          address={address}
          config={config}
          accent={BRANDING.accent}
          signTransaction={(xdr, opts) => kit.signTransaction(xdr, opts)}
        />
      )}

      <footer style={{ marginTop: 40, fontSize: 13, color: "#99a" }}>
        Powered by Trustline Onboarder · one signature via CAP-73 ·{" "}
        <a href="https://github.com/theahaco/trustline-onboarder">source</a>
      </footer>
    </main>
  );
}

function btn(accent: string): React.CSSProperties {
  return {
    background: accent,
    color: "#fff",
    border: "none",
    borderRadius: 10,
    padding: "12px 20px",
    fontSize: 16,
    cursor: "pointer",
  };
}
