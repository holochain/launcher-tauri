import {
  DisabledAppReason,
  DnaGossipInfo,
  InstalledAppInfo,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

export function isAppRunning(app: InstalledAppInfo): boolean {
  return (app.status as any) === "running";
}
export function isAppDisabled(app: InstalledAppInfo): boolean {
  return Object.keys(app.status).includes("disabled");
}
export function isAppPaused(app: InstalledAppInfo): boolean {
  return Object.keys(app.status).includes("paused");
}
export function getReason(app: InstalledAppInfo): string | undefined {
  if (isAppRunning(app)) return undefined;
  if (isAppDisabled(app)) {
    const reason = (
      app.status as unknown as {
        disabled: {
          reason: DisabledAppReason;
        };
      }
    ).disabled.reason;

    if ((reason as any) === "never_started") {
      return "App was never started";
    } else if ((reason as any) === "user") {
      return "App was disabled by the user";
    } else {
      return `There was an error with this app: ${
        (
          reason as {
            error: string;
          }
        ).error
      }`;
    }
  } else {
    return (
      app.status as unknown as {
        paused: { reason: { error: string } };
      }
    ).paused.reason.error;
  }
}

export function gossipProgressIncoming(info: DnaGossipInfo) {
  const incoming_bytes_expected =
    info.total_historical_gossip_throughput.expected_op_bytes.incoming;
  const incoming_bytes_actual =
    info.total_historical_gossip_throughput.op_bytes.incoming;

  const ratio = 100 * (incoming_bytes_actual / incoming_bytes_expected);
  return ratio > 100 ? 100 : ratio;
}

export function gossipProgressOutgoing(info: DnaGossipInfo) {
  const outgoing_bytes_expected =
    info.total_historical_gossip_throughput.expected_op_bytes.outgoing;
  const outgoing_bytes_actual =
    info.total_historical_gossip_throughput.op_bytes.outgoing;

  const ratio = 100 * (outgoing_bytes_actual / outgoing_bytes_expected);
  return ratio > 100 ? 100 : ratio;
}

export function gossipProgressIncomingString(info: DnaGossipInfo) {
  const incoming_bytes_expected =
    info.total_historical_gossip_throughput.expected_op_bytes.incoming;
  const incoming_bytes_actual =
    info.total_historical_gossip_throughput.op_bytes.incoming;
  return `${prettyBytes(incoming_bytes_actual)} / ${prettyBytes(
    incoming_bytes_expected
  )}`;
}

export function gossipProgressOutgoingString(info: DnaGossipInfo) {
  const outgoing_bytes_expected =
    info.total_historical_gossip_throughput.expected_op_bytes.outgoing;
  const outgoing_bytes_actual =
    info.total_historical_gossip_throughput.op_bytes.outgoing;
  return `${prettyBytes(outgoing_bytes_actual)} / ${prettyBytes(
    outgoing_bytes_expected
  )}`;
}
