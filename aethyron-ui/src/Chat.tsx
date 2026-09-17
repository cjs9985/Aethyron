import { useCallback, useEffect, useRef, useState } from "react";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type MessageRole = "user" | "aethyron" | "progress" | "error";

interface Message {
  id: number;
  role: MessageRole;
  content: string;
  /** data-URL preview shown inside the bubble when user sent an image */
  imagePreview?: string;
  ts: string;
}

interface ChatEvent {
  kind: "progress" | "result" | "error";
  message: string;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

let _id = 0;
const nextId = () => ++_id;
const now = () => new Date().toLocaleTimeString("en-US", { hour12: false });

/** Read a File as a data-URL string. */
function readFileAsDataURL(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(file);
  });
}

/** Strip the data-URL prefix and return raw base64. */
function dataURLToBase64(dataURL: string): string {
  return dataURL.split(",")[1] ?? "";
}

const ACCEPTED_IMAGE_TYPES = ["image/png", "image/jpeg", "image/webp", "image/gif"];

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function MessageBubble({ msg }: { msg: Message }) {
  const isUser = msg.role === "user";
  const isProgress = msg.role === "progress";
  const isError = msg.role === "error";

  const bubbleColor = isUser
    ? "rgba(53, 124, 255, 0.18)"
    : isProgress
    ? "rgba(80, 80, 80, 0.25)"
    : isError
    ? "rgba(248, 113, 113, 0.15)"
    : "rgba(15, 25, 50, 0.75)";

  const borderColor = isUser
    ? "rgba(80, 140, 255, 0.55)"
    : isProgress
    ? "rgba(100, 100, 120, 0.3)"
    : isError
    ? "rgba(248, 113, 113, 0.5)"
    : "rgba(53, 124, 255, 0.35)";

  const labelColor = isUser
    ? "#7db2ff"
    : isProgress
    ? "#6b7280"
    : isError
    ? "#f87171"
    : "#4ade80";

  const label = isUser ? "YOU" : isProgress ? "···" : isError ? "ERROR" : "AETHYRON";

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: isUser ? "flex-end" : "flex-start",
        marginBottom: "12px",
      }}
    >
      {/* Label + timestamp */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "6px",
          marginBottom: "4px",
          flexDirection: isUser ? "row-reverse" : "row",
        }}
      >
        <span style={{ fontSize: "10px", fontWeight: 700, letterSpacing: "0.1em", color: labelColor }}>
          {label}
        </span>
        <span style={{ fontSize: "10px", color: "rgba(156,163,175,0.5)" }}>{msg.ts}</span>
      </div>

      {/* Bubble */}
      <div
        style={{
          maxWidth: "85%",
          padding: "10px 14px",
          background: bubbleColor,
          border: `1px solid ${borderColor}`,
          borderRadius: isUser ? "12px 12px 2px 12px" : "12px 12px 12px 2px",
          color: isProgress ? "rgba(156,163,175,0.7)" : "rgba(229,231,235,0.92)",
          fontSize: isProgress ? "12px" : "13px",
          lineHeight: 1.55,
          fontStyle: isProgress ? "italic" : "normal",
          whiteSpace: "pre-wrap",
          wordBreak: "break-word",
          backdropFilter: "blur(6px)",
        }}
      >
        {/* Attached image preview inside the bubble */}
        {msg.imagePreview && (
          <img
            src={msg.imagePreview}
            alt="attached"
            style={{
              display: "block",
              maxWidth: "100%",
              maxHeight: "200px",
              borderRadius: "6px",
              marginBottom: msg.content ? "8px" : 0,
              border: "1px solid rgba(80,140,255,0.3)",
            }}
          />
        )}
        {msg.content}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Main Chat component
// ---------------------------------------------------------------------------

interface ChatProps {
  onClose: () => void;
}

export function Chat({ onClose }: ChatProps) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: nextId(),
      role: "aethyron",
      content: "Aethyron online. Describe your engineering goal and I will plan, implement, and validate it. You can also attach an image — drag it here, paste it, or use the 📎 button.",
      ts: now(),
    },
  ]);

  const [input, setInput] = useState("");
  const [busy, setBusy] = useState(false);
  /** data-URL of the pending attached image */
  const [pendingImage, setPendingImage] = useState<string | null>(null);
  /** Whether the drop zone is active */
  const [dragOver, setDragOver] = useState(false);

  const bottomRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const sseRef = useRef<EventSource | null>(null);
  const sseFinishedRef = useRef(false);

  // Auto-scroll to newest message
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Clean up SSE on unmount
  useEffect(() => {
    return () => {
      sseRef.current?.close();
      sseRef.current = null;
    };
  }, []);

  // ── Paste handler — intercept image pastes anywhere in the panel ──
  useEffect(() => {
    const handlePaste = (e: ClipboardEvent) => {
      const items = e.clipboardData?.items;
      if (!items) return;
      for (const item of Array.from(items)) {
        if (ACCEPTED_IMAGE_TYPES.includes(item.type)) {
          const file = item.getAsFile();
          if (file) {
            e.preventDefault();
            readFileAsDataURL(file).then(setPendingImage).catch(console.error);
            return;
          }
        }
      }
    };
    window.addEventListener("paste", handlePaste);
    return () => window.removeEventListener("paste", handlePaste);
  }, []);

  // ── Drag-and-drop handlers ──
  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(true);
  }, []);

  const handleDragLeave = useCallback(() => setDragOver(false), []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    const file = Array.from(e.dataTransfer.files).find((f) =>
      ACCEPTED_IMAGE_TYPES.includes(f.type)
    );
    if (file) {
      readFileAsDataURL(file).then(setPendingImage).catch(console.error);
    }
  }, []);

  // ── File picker ──
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      readFileAsDataURL(file).then(setPendingImage).catch(console.error);
    }
    // Reset so the same file can be reselected
    e.target.value = "";
  };

  // ── Message helpers ──
  const addMessage = (role: MessageRole, content: string, imagePreview?: string) => {
    setMessages((prev) => [...prev, { id: nextId(), role, content, imagePreview, ts: now() }]);
  };

  const upsertProgress = (content: string) => {
    setMessages((prev) => {
      const lastIdx = [...prev].reverse().findIndex((m) => m.role === "progress");
      if (lastIdx !== -1) {
        const realIdx = prev.length - 1 - lastIdx;
        const updated = [...prev];
        updated[realIdx] = { ...updated[realIdx], content, ts: now() };
        return updated;
      }
      return [...prev, { id: nextId(), role: "progress", content, ts: now() }];
    });
  };

  const clearProgress = () => {
    setMessages((prev) => {
      const lastIdx = [...prev].reverse().findIndex((m) => m.role === "progress");
      if (lastIdx === -1) return prev;
      const realIdx = prev.length - 1 - lastIdx;
      return prev.filter((_, i) => i !== realIdx);
    });
  };

  // ── Send ──
  const sendMessage = async () => {
    const text = input.trim();
    if ((!text && !pendingImage) || busy) return;

    const imageDataURL = pendingImage;
    setInput("");
    setPendingImage(null);
    setBusy(true);
    sseFinishedRef.current = false;

    // Show user bubble with optional image preview
    addMessage("user", text, imageDataURL ?? undefined);

    // Open SSE before POST
    sseRef.current?.close();
    const sse = new EventSource("/chat/stream");
    sseRef.current = sse;

    sse.onmessage = (e) => {
      try {
        const event: ChatEvent = JSON.parse(e.data as string);
        if (event.kind === "progress") {
          upsertProgress(event.message);
          return;
        }
        if (event.kind === "result") {
          sseFinishedRef.current = true;
          clearProgress();
          addMessage("aethyron", event.message);
          sse.close();
          if (sseRef.current === sse) sseRef.current = null;
          setBusy(false);
          inputRef.current?.focus();
          return;
        }
        if (event.kind === "error") {
          sseFinishedRef.current = true;
          clearProgress();
          addMessage("error", event.message);
          sse.close();
          if (sseRef.current === sse) sseRef.current = null;
          setBusy(false);
          inputRef.current?.focus();
        }
      } catch { /* malformed — ignore */ }
    };

    sse.onerror = () => {
      sse.close();
      if (sseRef.current === sse) sseRef.current = null;
    };

    // Build POST body — include base64 image if present
    const body: { message: string; image?: string } = {
      message: text || "What do you see in this image?",
    };
    if (imageDataURL) {
      body.image = dataURLToBase64(imageDataURL);
    }

    try {
      const res = await fetch("/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });

      const data = await res.json() as {
        reply?: string;
        tasks_completed?: number;
        files_changed?: number;
        repairs?: number;
        success?: boolean;
        error?: string;
      };

      if (sseFinishedRef.current) return;

      if (data.error) {
        sseFinishedRef.current = true;
        clearProgress();
        addMessage("error", data.error);
        sse.close();
        if (sseRef.current === sse) sseRef.current = null;
        setBusy(false);
        inputRef.current?.focus();
        return;
      }

      sseFinishedRef.current = true;
      clearProgress();
      addMessage("aethyron", data.reply ?? "Done.");
      sse.close();
      if (sseRef.current === sse) sseRef.current = null;
      setBusy(false);
      inputRef.current?.focus();
    } catch {
      if (!sseFinishedRef.current) {
        sseFinishedRef.current = true;
        clearProgress();
        addMessage("error", "Could not reach Aethyron API.");
        sse.close();
        if (sseRef.current === sse) sseRef.current = null;
        setBusy(false);
        inputRef.current?.focus();
      }
    }
  };

  const canSend = !busy && (input.trim().length > 0 || pendingImage !== null);

  return (
    <div
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      style={{
        display: "flex",
        flexDirection: "column",
        width: "min(560px, 94vw)",
        height: "min(620px, 80vh)",
        background: dragOver
          ? "rgba(53, 124, 255, 0.08)"
          : "rgba(3, 5, 10, 0.88)",
        border: dragOver
          ? "1px solid rgba(80, 140, 255, 0.8)"
          : "1px solid rgba(80, 140, 255, 0.35)",
        borderRadius: "16px",
        overflow: "hidden",
        backdropFilter: "blur(16px)",
        boxShadow: "0 8px 40px rgba(0,0,0,0.7), 0 0 0 1px rgba(53,124,255,0.1)",
        fontFamily: "'Segoe UI', system-ui, sans-serif",
        transition: "background 0.2s, border-color 0.2s",
      }}
    >
      {/* ── Header ── */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "12px 16px",
          borderBottom: "1px solid rgba(80, 140, 255, 0.2)",
          background: "rgba(8, 12, 24, 0.9)",
          flexShrink: 0,
        }}
      >
        <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
          <div
            style={{
              width: 8,
              height: 8,
              borderRadius: "50%",
              background: busy ? "#facc15" : "#4ade80",
              boxShadow: `0 0 6px ${busy ? "#facc15" : "#4ade80"}`,
              transition: "background 0.3s, box-shadow 0.3s",
            }}
          />
          <span style={{ fontSize: "13px", fontWeight: 700, letterSpacing: "0.06em", color: "rgba(219,234,254,0.9)" }}>
            AETHYRON CHAT
          </span>
          {busy && (
            <span style={{ fontSize: "11px", color: "#facc15", opacity: 0.8 }}>
              {pendingImage ? "analysing image…" : "running mission…"}
            </span>
          )}
          {dragOver && !busy && (
            <span style={{ fontSize: "11px", color: "#7db2ff", opacity: 0.9 }}>
              drop image here
            </span>
          )}
        </div>
        <button
          onClick={onClose}
          title="Close chat"
          style={{
            background: "transparent",
            border: "none",
            color: "rgba(156,163,175,0.6)",
            cursor: "pointer",
            fontSize: "18px",
            lineHeight: 1,
            padding: "2px 6px",
            borderRadius: "4px",
            transition: "color 0.2s",
          }}
          onMouseEnter={(e) => ((e.target as HTMLButtonElement).style.color = "rgba(229,231,235,0.9)")}
          onMouseLeave={(e) => ((e.target as HTMLButtonElement).style.color = "rgba(156,163,175,0.6)")}
        >
          ✕
        </button>
      </div>

      {/* ── Message thread ── */}
      <div
        style={{
          flex: 1,
          overflowY: "auto",
          padding: "16px 14px",
          display: "flex",
          flexDirection: "column",
          scrollbarWidth: "thin",
          scrollbarColor: "rgba(53,124,255,0.3) transparent",
        }}
      >
        {messages.map((msg) => (
          <MessageBubble key={msg.id} msg={msg} />
        ))}
        <div ref={bottomRef} />
      </div>

      {/* ── Pending image preview strip ── */}
      {pendingImage && (
        <div
          style={{
            padding: "8px 14px 0",
            background: "rgba(5, 8, 16, 0.95)",
            borderTop: "1px solid rgba(80, 140, 255, 0.15)",
            display: "flex",
            alignItems: "center",
            gap: "10px",
            flexShrink: 0,
          }}
        >
          <div style={{ position: "relative", display: "inline-block" }}>
            <img
              src={pendingImage}
              alt="pending attachment"
              style={{
                height: "56px",
                maxWidth: "120px",
                objectFit: "cover",
                borderRadius: "6px",
                border: "1px solid rgba(80,140,255,0.5)",
              }}
            />
            <button
              onClick={() => setPendingImage(null)}
              title="Remove image"
              style={{
                position: "absolute",
                top: -6,
                right: -6,
                width: 18,
                height: 18,
                borderRadius: "50%",
                background: "rgba(248,113,113,0.9)",
                border: "none",
                color: "white",
                fontSize: "10px",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                lineHeight: 1,
              }}
            >
              ✕
            </button>
          </div>
          <span style={{ fontSize: "11px", color: "rgba(156,163,175,0.7)" }}>
            Image attached — add a message or send as-is
          </span>
        </div>
      )}

      {/* ── Input row ── */}
      <div
        style={{
          display: "flex",
          gap: "8px",
          padding: "12px 14px",
          borderTop: "1px solid rgba(80, 140, 255, 0.2)",
          background: "rgba(5, 8, 16, 0.95)",
          flexShrink: 0,
          alignItems: "center",
        }}
      >
        {/* Hidden file input */}
        <input
          ref={fileInputRef}
          type="file"
          accept="image/png,image/jpeg,image/webp,image/gif"
          style={{ display: "none" }}
          onChange={handleFileChange}
        />

        {/* Attach image button */}
        <button
          onClick={() => fileInputRef.current?.click()}
          disabled={busy}
          title="Attach image (or paste / drag-drop)"
          style={{
            flexShrink: 0,
            width: 36,
            height: 36,
            borderRadius: "8px",
            background: pendingImage
              ? "rgba(53, 124, 255, 0.4)"
              : "rgba(53, 124, 255, 0.15)",
            border: "1px solid rgba(80, 140, 255, 0.4)",
            color: busy ? "rgba(255,255,255,0.3)" : "rgba(180,210,255,0.9)",
            cursor: busy ? "not-allowed" : "pointer",
            fontSize: "16px",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            transition: "background 0.2s",
          }}
        >
          📎
        </button>

        <input
          ref={inputRef}
          type="text"
          placeholder={busy ? "Working…" : pendingImage ? "Add a message (optional)…" : "Describe your goal or paste an image…"}
          value={input}
          disabled={busy}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && sendMessage()}
          style={{
            flex: 1,
            padding: "10px 14px",
            background: busy ? "rgba(5,7,13,0.5)" : "rgba(5,7,13,0.88)",
            border: "1px solid rgba(80,140,255,0.35)",
            borderRadius: "8px",
            color: busy ? "rgba(156,163,175,0.5)" : "white",
            fontSize: "13px",
            outline: "none",
            transition: "border-color 0.2s, background 0.2s",
            cursor: busy ? "not-allowed" : "text",
          }}
          onFocus={(e) => !busy && ((e.target as HTMLInputElement).style.borderColor = "rgba(80,140,255,0.7)")}
          onBlur={(e) => ((e.target as HTMLInputElement).style.borderColor = "rgba(80,140,255,0.35)")}
        />

        <button
          onClick={sendMessage}
          disabled={!canSend}
          style={{
            padding: "10px 18px",
            background: canSend ? "rgba(53,124,255,0.85)" : "rgba(53,124,255,0.3)",
            border: "none",
            borderRadius: "8px",
            color: canSend ? "white" : "rgba(255,255,255,0.4)",
            fontSize: "13px",
            fontWeight: 600,
            cursor: canSend ? "pointer" : "not-allowed",
            whiteSpace: "nowrap",
            transition: "background 0.2s, color 0.2s",
            letterSpacing: "0.04em",
          }}
        >
          {busy ? "···" : "Send"}
        </button>
      </div>
    </div>
  );
}
