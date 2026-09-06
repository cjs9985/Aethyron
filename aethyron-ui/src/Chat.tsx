import { useEffect, useRef, useState } from "react";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type MessageRole = "user" | "aethyron" | "progress" | "error";

interface Message {
  id: number;
  role: MessageRole;
  content: string;
  /** ISO timestamp */
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

  const label = isUser
    ? "YOU"
    : isProgress
    ? "···"
    : isError
    ? "ERROR"
    : "AETHYRON";

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
        <span
          style={{
            fontSize: "10px",
            fontWeight: 700,
            letterSpacing: "0.1em",
            color: labelColor,
          }}
        >
          {label}
        </span>
        <span style={{ fontSize: "10px", color: "rgba(156,163,175,0.5)" }}>
          {msg.ts}
        </span>
      </div>

      {/* Bubble */}
      <div
        style={{
          maxWidth: "85%",
          padding: "10px 14px",
          background: bubbleColor,
          border: `1px solid ${borderColor}`,
          borderRadius: isUser
            ? "12px 12px 2px 12px"
            : "12px 12px 12px 2px",
          color: isProgress ? "rgba(156,163,175,0.7)" : "rgba(229,231,235,0.92)",
          fontSize: isProgress ? "12px" : "13px",
          lineHeight: 1.55,
          fontStyle: isProgress ? "italic" : "normal",
          whiteSpace: "pre-wrap",
          wordBreak: "break-word",
          backdropFilter: "blur(6px)",
        }}
      >
        {msg.content}
      </div>
    </div>
  );
}

// ---------------------------------------------------------------------------
// Main Chat component
// ---------------------------------------------------------------------------

interface ChatProps {
  /** Called when the user clicks the close / collapse button */
  onClose: () => void;
}

export function Chat({ onClose }: ChatProps) {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: nextId(),
      role: "aethyron",
      content:
        "Aethyron online. Describe your engineering goal and I will plan, implement, and validate it.",
      ts: now(),
    },
  ]);
  const [input, setInput] = useState("");
  const [busy, setBusy] = useState(false);

  const bottomRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const sseRef = useRef<EventSource | null>(null);

  // Auto-scroll to newest message
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Clean up SSE on unmount
  useEffect(() => {
    return () => {
      sseRef.current?.close();
    };
  }, []);

  const addMessage = (role: MessageRole, content: string) => {
    setMessages((prev) => [
      ...prev,
      { id: nextId(), role, content, ts: now() },
    ]);
  };

  /** Replace the last progress message, or add a new one. */
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

  /** Remove the last progress bubble (after final result arrives). */
  const clearProgress = () => {
    setMessages((prev) => {
      const lastIdx = [...prev].reverse().findIndex((m) => m.role === "progress");
      if (lastIdx === -1) return prev;
      const realIdx = prev.length - 1 - lastIdx;
      return prev.filter((_, i) => i !== realIdx);
    });
  };

  const sendMessage = async () => {
    const text = input.trim();
    if (!text || busy) return;

    setInput("");
    setBusy(true);
    addMessage("user", text);

    // ── Open SSE stream first so we don't miss early events ──
    sseRef.current?.close();
    const sse = new EventSource("/chat/stream");
    sseRef.current = sse;

    sse.onmessage = (e) => {
      try {
        const event: ChatEvent = JSON.parse(e.data as string);
        if (event.kind === "progress") {
          upsertProgress(event.message);
        } else if (event.kind === "result") {
          clearProgress();
          addMessage("aethyron", event.message);
          sse.close();
          sseRef.current = null;
          setBusy(false);
        } else if (event.kind === "error") {
          clearProgress();
          addMessage("error", event.message);
          sse.close();
          sseRef.current = null;
          setBusy(false);
        }
      } catch {
        // malformed event — ignore
      }
    };

    sse.onerror = () => {
      // SSE connection dropped — the POST response will still carry the result
      sse.close();
      sseRef.current = null;
    };

    // ── POST the message ──
    try {
      const res = await fetch("/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: text }),
      });

      const data = await res.json() as {
        reply?: string;
        tasks_completed?: number;
        files_changed?: number;
        repairs?: number;
        success?: boolean;
        error?: string;
      };

      // If SSE already delivered the result we don't duplicate
      if (data.error) {
        clearProgress();
        addMessage("error", data.error);
        sse.close();
        sseRef.current = null;
        setBusy(false);
      }
      // If SSE hasn't fired yet (e.g. no subscriber connection), show result
      else if (sseRef.current) {
        clearProgress();
        addMessage("aethyron", data.reply ?? "Mission complete.");
        sse.close();
        sseRef.current = null;
        setBusy(false);
      }
    } catch {
      clearProgress();
      addMessage("error", "Could not reach Aethyron API.");
      sse.close();
      sseRef.current = null;
      setBusy(false);
    }

    inputRef.current?.focus();
  };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "min(560px, 94vw)",
        height: "min(620px, 80vh)",
        background: "rgba(3, 5, 10, 0.88)",
        border: "1px solid rgba(80, 140, 255, 0.35)",
        borderRadius: "16px",
        overflow: "hidden",
        backdropFilter: "blur(16px)",
        boxShadow: "0 8px 40px rgba(0,0,0,0.7), 0 0 0 1px rgba(53,124,255,0.1)",
        fontFamily: "'Segoe UI', system-ui, sans-serif",
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
          {/* Status dot */}
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
          <span
            style={{
              fontSize: "13px",
              fontWeight: 700,
              letterSpacing: "0.06em",
              color: "rgba(219, 234, 254, 0.9)",
            }}
          >
            AETHYRON CHAT
          </span>
          {busy && (
            <span
              style={{
                fontSize: "11px",
                color: "#facc15",
                opacity: 0.8,
              }}
            >
              running mission…
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
          onMouseEnter={(e) =>
            ((e.target as HTMLButtonElement).style.color = "rgba(229,231,235,0.9)")
          }
          onMouseLeave={(e) =>
            ((e.target as HTMLButtonElement).style.color = "rgba(156,163,175,0.6)")
          }
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

      {/* ── Input row ── */}
      <div
        style={{
          display: "flex",
          gap: "8px",
          padding: "12px 14px",
          borderTop: "1px solid rgba(80, 140, 255, 0.2)",
          background: "rgba(5, 8, 16, 0.95)",
          flexShrink: 0,
        }}
      >
        <input
          ref={inputRef}
          type="text"
          placeholder={busy ? "Mission in progress…" : "Describe your engineering goal…"}
          value={input}
          disabled={busy}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && sendMessage()}
          style={{
            flex: 1,
            padding: "10px 14px",
            background: busy
              ? "rgba(5, 7, 13, 0.5)"
              : "rgba(5, 7, 13, 0.88)",
            border: "1px solid rgba(80, 140, 255, 0.35)",
            borderRadius: "8px",
            color: busy ? "rgba(156,163,175,0.5)" : "white",
            fontSize: "13px",
            outline: "none",
            transition: "border-color 0.2s, background 0.2s",
            cursor: busy ? "not-allowed" : "text",
          }}
          onFocus={(e) =>
            !busy && ((e.target as HTMLInputElement).style.borderColor = "rgba(80,140,255,0.7)")
          }
          onBlur={(e) =>
            ((e.target as HTMLInputElement).style.borderColor = "rgba(80,140,255,0.35)")
          }
        />
        <button
          onClick={sendMessage}
          disabled={busy || !input.trim()}
          style={{
            padding: "10px 18px",
            background:
              busy || !input.trim()
                ? "rgba(53, 124, 255, 0.3)"
                : "rgba(53, 124, 255, 0.85)",
            border: "none",
            borderRadius: "8px",
            color: busy || !input.trim() ? "rgba(255,255,255,0.4)" : "white",
            fontSize: "13px",
            fontWeight: 600,
            cursor: busy || !input.trim() ? "not-allowed" : "pointer",
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
