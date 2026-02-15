interface CopyTextOptions {
  includePayloadOnFallback?: boolean;
  maxFallbackPayloadLength?: number;
  successMessage?: string;
}

export async function copyTextWithFallback(
  payload: string,
  label: string,
  options: CopyTextOptions = {}
) {
  const includePayloadOnFallback = options.includePayloadOnFallback ?? true;
  const maxFallbackPayloadLength = options.maxFallbackPayloadLength ?? 1200;
  const successMessage = options.successMessage ?? `${label} copied to clipboard`;
  const fallbackPayload =
    payload.length > maxFallbackPayloadLength
      ? `${payload.slice(0, maxFallbackPayloadLength)}… (truncated ${payload.length - maxFallbackPayloadLength} chars)`
      : payload;

  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(payload);
      return successMessage;
    } catch {
      if (includePayloadOnFallback) {
        return `Clipboard write blocked. ${label}: ${fallbackPayload}`;
      }
      return `Clipboard write blocked. ${label}`;
    }
  }

  if (includePayloadOnFallback) {
    return `Clipboard unavailable. ${label}: ${fallbackPayload}`;
  }
  return `Clipboard unavailable. ${label}`;
}

// Validation report utilities
import type { PackValidationResult } from "../pets/packValidation";

/**
 * Format validation report as readable text
 */
export function formatValidationReport(result: PackValidationResult): string {
  const { speciesId, pass, checks } = result;
  const status = pass ? "✓ PASSED" : "✗ FAILED";
  const failedChecks = checks.filter((c) => !c.pass);
  const passedChecks = checks.filter((c) => c.pass);

  let report = `Species Pack Validation Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Species ID: ${speciesId}
Status: ${status}
Passed: ${passedChecks.length}/${checks.length} checks

`;

  if (failedChecks.length > 0) {
    report += `Failed Checks
────────────────────────────────────────

`;
    failedChecks.forEach((check, index) => {
      report += `${index + 1}. ${check.label}
   Issue: ${check.detail}
   Fix: ${check.remediation}

`;
    });
  }

  if (passedChecks.length > 0) {
    report += `Passed Checks
────────────────────────────────────────

`;
    passedChecks.forEach((check) => {
      report += `✓ ${check.label}\n`;
    });
  }

  report += `
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Generated: ${new Date().toISOString()}
`;

  return report;
}

/**
 * Copy validation report to clipboard
 */
export async function copyValidationReport(result: PackValidationResult): Promise<string> {
  const report = formatValidationReport(result);
  return copyTextWithFallback(report, "Validation report", {
    successMessage: "Validation report copied to clipboard",
  });
}
