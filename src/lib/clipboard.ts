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
