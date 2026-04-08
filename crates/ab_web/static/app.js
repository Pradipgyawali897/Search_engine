const API = {
  list: (refresh) => `/api/content?limit=20&refresh=${refresh ? "true" : "false"}`,
  detail: (id) => `/api/content/${encodeURIComponent(id)}`,
};

const state = {
  requestToken: 0,
  docs: [],
  filteredDocs: [],
  selectedId: null,
  selectedDoc: null,
  listLoading: true,
  detailLoading: false,
  error: null,
  detailError: null,
  query: "",
  lastSyncedAt: null,
};

const el = {
  refreshBtn: document.getElementById("refreshBtn"),
  statusChip: document.getElementById("statusChip"),
  metricDocs: document.getElementById("metricDocs"),
  metricDocsNote: document.getElementById("metricDocsNote"),
  metricSelected: document.getElementById("metricSelected"),
  metricSelectedNote: document.getElementById("metricSelectedNote"),
  metricSynced: document.getElementById("metricSynced"),
  metricSyncedNote: document.getElementById("metricSyncedNote"),
  listCount: document.getElementById("listCount"),
  searchInput: document.getElementById("searchInput"),
  listState: document.getElementById("listState"),
  listError: document.getElementById("listError"),
  listErrorMessage: document.getElementById("listErrorMessage"),
  emptyListState: document.getElementById("emptyListState"),
  docList: document.getElementById("docList"),
  detailState: document.getElementById("detailState"),
  detailError: document.getElementById("detailError"),
  detailErrorMessage: document.getElementById("detailErrorMessage"),
  detailSubtitle: document.getElementById("detailSubtitle"),
  detailStamp: document.getElementById("detailStamp"),
  detailView: document.getElementById("detailView"),
  articleHost: document.getElementById("articleHost"),
  articleTitle: document.getElementById("articleTitle"),
  articleSummary: document.getElementById("articleSummary"),
  articleUrl: document.getElementById("articleUrl"),
  articleMeta: document.getElementById("articleMeta"),
  articleText: document.getElementById("articleText"),
  rawHtmlSection: document.getElementById("rawHtmlSection"),
  articleHtml: document.getElementById("articleHtml"),
  docItemTemplate: document.getElementById("docItemTemplate"),
  metaChipTemplate: document.getElementById("metaChipTemplate"),
};

bootstrap();

el.refreshBtn.addEventListener("click", () => refreshAll());
el.searchInput.addEventListener("input", (event) => {
  state.query = event.target.value.trim().toLowerCase();
  applyFilter();
  renderList();
});

el.docList.addEventListener("click", async (event) => {
  const item = event.target.closest("[data-doc-id]");
  if (!item) return;
  const id = item.getAttribute("data-doc-id");
  if (!id || state.selectedId === id) return;
  await selectDocument(id);
});

async function bootstrap() {
  updateStatus("Loading");
  syncUrlSelection();
  await loadDocuments({ preserveSelection: false });
}

async function refreshAll() {
  await loadDocuments({ preserveSelection: true, forceReload: true });
}

function syncUrlSelection() {
  const params = new URLSearchParams(window.location.search);
  const selected = params.get("id");
  if (selected) {
    state.selectedId = selected;
  }
}

async function loadDocuments({ preserveSelection, forceReload }) {
  const token = ++state.requestToken;
  state.listLoading = true;
  state.error = null;
  state.lastSyncedAt = null;
  setListError(null);
  setDetailError(null);
  renderLoadingState();

  try {
    const response = await fetch(API.list(forceReload), {
      headers: { Accept: "application/json" },
      cache: forceReload ? "no-store" : "default",
    });

    if (!response.ok) {
      throw new Error(`List request failed with ${response.status}`);
    }

    const payload = await response.json();
    if (token !== state.requestToken) return;

    state.docs = normalizeCollection(payload);
    state.filteredDocs = applyQuery(state.docs, state.query);
    state.lastSyncedAt = extractMetaTimestamp(payload) || Date.now();
    state.listLoading = false;

    renderList();
    renderSummary();

    const preferredId =
      state.selectedId && state.docs.some((doc) => sameId(doc.id, state.selectedId))
        ? state.selectedId
        : state.selectedId || state.docs[0]?.id || null;

    if (preferredId) {
      await selectDocument(preferredId, { fromLoad: true });
    } else {
      state.selectedId = null;
      state.selectedDoc = null;
      renderDetail();
    }

    updateStatus("Loaded");
  } catch (error) {
    if (token !== state.requestToken) return;
    state.listLoading = false;
    state.error = error instanceof Error ? error.message : "Unknown error";
    state.docs = [];
    state.filteredDocs = [];
    state.selectedId = null;
    state.selectedDoc = null;
    renderList();
    renderSummary();
    renderDetail();
    setListError(state.error);
    updateStatus("Error");
  }
}

async function selectDocument(id, options = {}) {
  const doc = findDocumentById(id);
  state.selectedId = String(id);
  state.detailLoading = true;
  state.detailError = null;
  state.selectedDoc = doc ?? null;
  setDetailError(null);
  renderDetail();
  updateUrl(id);
  updateStatus("Loading detail");

  if (!options.fromLoad) {
    renderList();
  }

  try {
    const response = await fetch(API.detail(id), {
      headers: { Accept: "application/json" },
      cache: "no-store",
    });

    if (!response.ok) {
      throw new Error(`Detail request failed with ${response.status}`);
    }

    const payload = await response.json();
    const detailed = normalizeDocument(payload, doc);
    state.selectedDoc = detailed;
    state.detailLoading = false;
    renderSummary();
    renderList();
    renderDetail();
    updateStatus("Loaded");
  } catch (error) {
    const fallback = doc ?? state.selectedDoc;
    state.detailLoading = false;
    if (fallback) {
      state.selectedDoc = fallback;
      state.detailError = error instanceof Error ? error.message : "Unknown error";
      renderDetail();
      updateStatus("Partial");
      return;
    }

    state.detailError = error instanceof Error ? error.message : "Unknown error";
    setDetailError(state.detailError);
    renderDetail();
    updateStatus("Error");
  }
}

function renderLoadingState() {
  el.listState.classList.remove("hidden");
  el.emptyListState.classList.add("hidden");
  el.docList.classList.add("hidden");
  el.listError.classList.add("hidden");
  el.detailState.classList.remove("hidden");
  el.detailView.classList.add("hidden");
  el.detailError.classList.add("hidden");
  el.listState.querySelector("strong").textContent = "Loading documents";
  el.listState.querySelector("p").textContent = "Reading the latest rows from PostgreSQL.";
  el.detailState.querySelector("strong").textContent = "Waiting for a document";
  el.detailState.querySelector("p").textContent =
    "Choose a row from the list to show the article text and metadata.";
}

function renderList() {
  const filtered = state.filteredDocs;
  el.docList.innerHTML = "";
  el.listCount.textContent = `${filtered.length} ${filtered.length === 1 ? "item" : "items"}`;
  el.metricDocs.textContent = String(state.docs.length);
  el.metricDocsNote.textContent =
    state.docs.length > 0 ? "Scraped rows currently available in the database" : "Waiting for scraped documents";
  el.metricSynced.textContent = state.lastSyncedAt ? formatTimestamp(state.lastSyncedAt) : "--";
  el.metricSyncedNote.textContent =
    state.lastSyncedAt && state.docs.length
      ? "The list was fetched from PostgreSQL just now"
      : "Refresh to query the database again";

  if (state.listLoading) {
    el.listState.classList.remove("hidden");
    el.docList.classList.add("hidden");
    el.emptyListState.classList.add("hidden");
    return;
  }

  el.listState.classList.add("hidden");

  if (state.error) {
    el.listError.classList.remove("hidden");
    el.docList.classList.add("hidden");
    el.emptyListState.classList.add("hidden");
    return;
  }

  if (!filtered.length) {
    el.emptyListState.classList.remove("hidden");
    el.docList.classList.add("hidden");
    return;
  }

  el.listError.classList.add("hidden");
  el.emptyListState.classList.add("hidden");
  el.docList.classList.remove("hidden");

  for (const doc of filtered) {
    el.docList.appendChild(createDocItem(doc));
  }
}

function renderSummary() {
  const doc = state.selectedDoc;
  if (!doc) {
    el.metricSelected.textContent = "None";
    el.metricSelectedNote.textContent = "Pick a document from the list";
    return;
  }

  el.metricSelected.textContent = doc.title || doc.canonical_url || `#${doc.id}`;
  el.metricSelectedNote.textContent = doc.content_type || doc.host || "Selected document";
}

function renderDetail() {
  const doc = state.selectedDoc;
  if (!doc) {
    el.detailState.classList.remove("hidden");
    el.detailView.classList.add("hidden");
    el.detailError.classList.add("hidden");
    el.detailSubtitle.textContent = "Select a document to inspect the scraped content.";
    el.detailStamp.textContent = "--";
    return;
  }

  el.detailState.classList.add("hidden");
  el.detailView.classList.remove("hidden");
  el.detailError.classList.toggle("hidden", !state.detailError);
  if (state.detailError) {
    el.detailErrorMessage.textContent = state.detailError;
  }

  el.articleHost.textContent = doc.host || hostFromUrl(doc.canonical_url) || "Unknown host";
  el.articleTitle.textContent = doc.title || doc.canonical_url || `Document #${doc.id}`;
  el.articleSummary.textContent =
    doc.summary || summarizeText(doc.plain_text || doc.excerpt || "", 360) || "No summary stored.";
  el.articleUrl.textContent = doc.canonical_url || "Open source";
  el.articleUrl.href = doc.canonical_url || "#";
  el.articleUrl.setAttribute("aria-disabled", doc.canonical_url ? "false" : "true");
  if (!doc.canonical_url) {
    el.articleUrl.removeAttribute("href");
  }

  el.detailSubtitle.textContent = describeDocument(doc);
  el.detailStamp.textContent = formatTimestamp(doc.fetched_at || doc.indexed_at || doc.created_at);

  el.articleMeta.innerHTML = "";
  const meta = [
    ["ID", doc.id ?? "n/a"],
    ["Host", doc.host || "n/a"],
    ["Content type", doc.content_type || "unknown"],
    ["Fetched", formatTimestamp(doc.fetched_at || doc.indexed_at || doc.created_at)],
    ["Indexed", formatTimestamp(doc.indexed_at || doc.fetched_at || doc.created_at)],
    ["Path", doc.path || "/"],
  ];
  for (const [label, value] of meta) {
    el.articleMeta.appendChild(createMetaChip(label, value));
  }

  const plainText = doc.plain_text || doc.excerpt || doc.summary || "";
  el.articleText.textContent = plainText || "No extracted text stored for this document.";

  if (doc.raw_html) {
    el.rawHtmlSection.classList.remove("hidden");
    el.articleHtml.textContent = doc.raw_html;
  } else {
    el.rawHtmlSection.classList.add("hidden");
    el.articleHtml.textContent = "";
  }
}

function createDocItem(doc) {
  const node = el.docItemTemplate.content.cloneNode(true);
  const button = node.querySelector(".doc-item");
  const title = node.querySelector(".doc-item-title");
  const badge = node.querySelector(".doc-item-badge");
  const url = node.querySelector(".doc-item-url");
  const summary = node.querySelector(".doc-item-summary");
  const time = node.querySelector(".doc-item-time");
  const host = node.querySelector(".doc-item-host");

  button.setAttribute("data-doc-id", String(doc.id));
  button.classList.toggle("is-active", sameId(doc.id, state.selectedId));
  title.textContent = doc.title || doc.canonical_url || `Document #${doc.id}`;
  badge.textContent = doc.content_type || "document";
  url.textContent = doc.canonical_url || doc.url || "No URL stored";
  summary.textContent = doc.summary || summarizeText(doc.plain_text || "", 180) || "No summary available.";
  time.textContent = formatTimestamp(doc.fetched_at || doc.indexed_at || doc.created_at);
  host.textContent = doc.host || hostFromUrl(doc.canonical_url) || "unknown";

  return node;
}

function createMetaChip(label, value) {
  const node = el.metaChipTemplate.content.cloneNode(true);
  const chip = node.querySelector(".meta-chip");
  chip.textContent = `${label}: ${value}`;
  return chip;
}

function setListError(message) {
  if (!message) {
    el.listError.classList.add("hidden");
    el.listErrorMessage.textContent = "";
    return;
  }

  el.listError.classList.remove("hidden");
  el.listErrorMessage.textContent = message;
}

function setDetailError(message) {
  if (!message) {
    el.detailError.classList.add("hidden");
    el.detailErrorMessage.textContent = "";
    return;
  }

  el.detailError.classList.remove("hidden");
  el.detailErrorMessage.textContent = message;
}

function updateStatus(label) {
  el.statusChip.textContent = label;
}

function updateUrl(id) {
  const url = new URL(window.location.href);
  if (id == null) {
    url.searchParams.delete("id");
  } else {
    url.searchParams.set("id", String(id));
  }
  window.history.replaceState({}, "", url);
}

function applyFilter() {
  state.filteredDocs = applyQuery(state.docs, state.query);
}

function applyQuery(items, query) {
  if (!query) return [...items];
  return items.filter((item) => {
    const haystack = [
      item.title,
      item.canonical_url,
      item.host,
      item.path,
      item.content_type,
      item.summary,
      item.plain_text,
      item.excerpt,
      item.raw_html,
    ]
      .filter(Boolean)
      .join(" ")
      .toLowerCase();
    return haystack.includes(query);
  });
}

function normalizeCollection(payload) {
  const raw = extractCollection(payload);
  return raw.map((item) => normalizeDocument(item));
}

function extractCollection(payload) {
  if (Array.isArray(payload)) return payload;
  if (!payload || typeof payload !== "object") return [];

  const candidates = [
    payload.items,
    payload.documents,
    payload.rows,
    payload.data,
    payload.results,
    payload.content,
  ];

  for (const candidate of candidates) {
    if (Array.isArray(candidate)) return candidate;
  }

  if (payload.document) return [payload.document];
  if (payload.content) return [payload.content];
  return [];
}

function normalizeDocument(payload, fallback = null) {
  const source =
    payload && typeof payload === "object"
      ? payload.content && typeof payload.content === "object"
        ? payload.content
        : payload.document && typeof payload.document === "object"
          ? payload.document
          : payload.data && typeof payload.data === "object"
            ? payload.data
            : payload
      : {};
  const base = fallback && typeof fallback === "object" ? fallback : {};

  const doc = {
    id: source.id ?? base.id ?? null,
    title: firstText(source, base, ["title", "name", "headline"]),
    canonical_url: firstText(source, base, ["canonical_url", "url", "source_url"]),
    host: firstText(source, base, ["host"]),
    path: firstText(source, base, ["path"]),
    summary: firstText(source, base, ["summary", "excerpt", "description"]),
    plain_text: firstText(source, base, ["plain_text", "text", "content", "body"]),
    raw_html: firstText(source, base, ["raw_html", "html"]),
    content_type: firstText(source, base, ["content_type", "type", "mime_type"]),
    fetched_at: source.fetched_at ?? base.fetched_at ?? null,
    indexed_at: source.indexed_at ?? base.indexed_at ?? null,
    created_at: source.created_at ?? base.created_at ?? null,
  };

  if (!doc.summary && doc.plain_text) {
    doc.summary = summarizeText(doc.plain_text, 240);
  }

  if (!doc.plain_text && doc.raw_html) {
    doc.plain_text = stripHtml(doc.raw_html);
  }

  if (!doc.host && doc.canonical_url) {
    doc.host = hostFromUrl(doc.canonical_url);
  }

  if (!doc.path) {
    doc.path = "/";
  }

  return doc;
}

function extractMetaTimestamp(payload) {
  if (!payload || typeof payload !== "object") return null;
  const meta = payload.meta && typeof payload.meta === "object" ? payload.meta : null;
  if (!meta) return null;
  return (
    meta.fetched_at ||
    meta.updated_at ||
    meta.synced_at ||
    meta.queried_at ||
    meta.timestamp ||
    null
  );
}

function findDocumentById(id) {
  return state.docs.find((doc) => sameId(doc.id, id)) || null;
}

function sameId(left, right) {
  return String(left) === String(right);
}

function firstText(source, fallback, keys) {
  for (const key of keys) {
    const value = textValue(source?.[key]);
    if (value) return value;
  }

  for (const key of keys) {
    const value = textValue(fallback?.[key]);
    if (value) return value;
  }

  return "";
}

function textValue(value) {
  if (typeof value !== "string") return "";
  const trimmed = value.trim();
  return trimmed || "";
}

function hostFromUrl(url) {
  try {
    return new URL(url).host;
  } catch {
    return "";
  }
}

function summarizeText(text, limit) {
  const compact = String(text || "").replace(/\s+/g, " ").trim();
  if (!compact) return "";
  if (compact.length <= limit) return compact;
  return `${compact.slice(0, limit).trimEnd()}...`;
}

function formatTimestamp(value) {
  if (!value) return "--";
  const date = normalizeDate(value);
  if (!date) return String(value);
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}

function normalizeDate(value) {
  if (typeof value === "number") {
    return new Date(value > 10_000_000_000 ? value : value * 1000);
  }
  if (typeof value === "string") {
    const trimmed = value.trim();
    if (!trimmed) return null;
    const numeric = Number(trimmed);
    if (!Number.isNaN(numeric) && trimmed === String(numeric)) {
      return new Date(numeric > 10_000_000_000 ? numeric : numeric * 1000);
    }
    const date = new Date(trimmed);
    return Number.isNaN(date.getTime()) ? null : date;
  }
  return null;
}

function describeDocument(doc) {
  const parts = [];
  if (doc.host) parts.push(doc.host);
  if (doc.path && doc.path !== "/") parts.push(doc.path);
  if (doc.content_type) parts.push(doc.content_type);
  return parts.length ? parts.join(" • ") : "No metadata available";
}

function stripHtml(html) {
  const container = document.createElement("div");
  container.innerHTML = html;
  return (container.textContent || container.innerText || "").trim();
}
