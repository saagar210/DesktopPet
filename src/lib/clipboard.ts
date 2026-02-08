interface CopyTextOptions {
  includePayloadOnFallback?: boolean;
  successMessage?: string;
}

export async function copyTextWithFallback(
  payload: string,
  label: string,
  options: CopyTextOptions = {}
) {
  const includePayloadOnFallback = options.includePayloadOnFallback ?? true;
  const successMessage = options.successMessage ?? `${label} copied to clipboard`;

  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(payload);
      return successMessage;
    } catch {
      if (includePayloadOnFallback) {
        return `Clipboard write blocked. ${label}: ${payload}`;
      }
      return `Clipboard write blocked. ${label}`;
    }
  }

  if (includePayloadOnFallback) {
    return `Clipboard unavailable. ${label}: ${payload}`;
  }
  return `Clipboard unavailable. ${label}`;
}
