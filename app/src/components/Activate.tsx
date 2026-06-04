import { useEffect, useState } from "react";
import {
  getActivationStatus,
  type ActivationStatus,
  type OnboarderConfig,
} from "@theaha/trustline-onboarder";
import { useActivation } from "@theaha/trustline-onboarder/react";
import { NETWORK } from "../config.js";

export function Activate({
  address,
  config,
  accent,
  signTransaction,
}: {
  address: string;
  config: OnboarderConfig;
  accent: string;
  signTransaction: (xdr: string, opts: { networkPassphrase: string }) => Promise<{ signedTxXdr: string }>;
}) {
  const [status, setStatus] = useState<ActivationStatus | null>(null);

  const { state, error, activate } = useActivation({
    rpcUrl: NETWORK.rpcUrl,
    networkPassphrase: NETWORK.passphrase,
    config,
    signTransaction,
  });

  const refresh = () =>
    getActivationStatus({
      horizonUrl: NETWORK.horizonUrl,
      account: address,
      assetCode: config.assetCode,
      assetIssuer: config.assetIssuer,
    }).then(setStatus);

  useEffect(() => {
    void refresh();
  }, [address, config, state]);

  const done = status?.hasTrustline && status?.isAuthorized;
  const busy = state === "building" || state === "signing" || state === "submitting";

  return (
    <section style={card}>
      <p style={{ marginTop: 0 }}>
        Account <code>{address.slice(0, 6)}…{address.slice(-6)}</code>
      </p>

      {done ? (
        <p>✓ Your {config.assetCode} trustline is authorized. You're ready to receive {config.assetCode}.</p>
      ) : (
        <>
          <button
            disabled={busy}
            onClick={() => void activate(address)}
            style={{
              background: accent,
              color: "#fff",
              border: "none",
              borderRadius: 10,
              padding: "12px 20px",
              fontSize: 16,
              cursor: busy ? "default" : "pointer",
            }}
          >
            {busy ? "Activating…" : `Activate ${config.assetCode} (1 signature)`}
          </button>
          <p style={{ fontSize: 13, color: "#778" }}>
            One signature creates your trustline (CAP-73) and authorizes it — no separate
            "create a trustline" step.
          </p>
        </>
      )}

      {state === "error" && <p style={{ color: "#c0392b" }}>{error}</p>}
    </section>
  );
}

const card: React.CSSProperties = {
  border: "1px solid #e3e6ee",
  borderRadius: 14,
  padding: 24,
};
