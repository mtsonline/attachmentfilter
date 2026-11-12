// Entry point of the admin UI. Content is only ever inserted via textContent
// or DOM APIs, never as HTML strings, so the strict CSP needs no exceptions.

interface Health {
  status: string;
  version: string;
}

async function fetchHealth(): Promise<Health> {
  const res = await fetch("/healthz", { headers: { Accept: "application/json" } });
  if (!res.ok) {
    throw new Error(`backend answered ${res.status}`);
  }
  return (await res.json()) as Health;
}

async function render(root: HTMLElement): Promise<void> {
  const heading = document.createElement("h1");
  heading.textContent = "AttachmentFilter";
  const status = document.createElement("p");
  root.replaceChildren(heading, status);

  try {
    const health = await fetchHealth();
    status.textContent = `Backend ${health.status} (${health.version})`;
  } catch (err) {
    status.textContent = `Backend unreachable: ${err instanceof Error ? err.message : "unknown error"}`;
  }
}

const root = document.getElementById("app");
if (root) {
  void render(root);
}
