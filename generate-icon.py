"""Generate a Windows-compatible multi-size Aethyron .ico (BMP 16-64, PNG 256)."""

from __future__ import annotations

import math
import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OUT = ROOT / "aethyron.ico"

NAVY = (3, 5, 10)
BLUE = (53, 124, 255)
GLOW = (125, 178, 255)
RING = (57, 125, 255)


def clamp(v: float) -> int:
    return max(0, min(255, int(round(v))))


def mix(a: tuple[int, int, int], b: tuple[int, int, int], t: float) -> tuple[int, int, int]:
    t = max(0.0, min(1.0, t))
    return (
        clamp(a[0] + (b[0] - a[0]) * t),
        clamp(a[1] + (b[1] - a[1]) * t),
        clamp(a[2] + (b[2] - a[2]) * t),
    )


def hex_sdf(px: float, py: float, radius: float) -> float:
    q = (abs(px), abs(py))
    k = math.sqrt(3.0)
    return max(q[0] * 0.5 + q[1] * (k / 2.0), q[0]) - radius


def render(size: int) -> list[tuple[int, int, int, int]]:
    pixels: list[tuple[int, int, int, int]] = []
    cx = (size - 1) / 2.0
    cy = (size - 1) / 2.0
    scale = size / 256.0
    corner = 28.0 * scale
    box_r = size / 2.0 - 2.0 * scale
    hex_r = 78.0 * scale
    inner_r = 34.0 * scale
    ring_r = 96.0 * scale
    stroke = max(1.6, 7.0 * scale)

    for y in range(size):
        for x in range(size):
            dx = x - cx
            dy = y - cy
            ax, ay = abs(dx), abs(dy)

            # rounded square alpha
            ox = max(0.0, ax - (box_r - corner))
            oy = max(0.0, ay - (box_r - corner))
            sq = math.hypot(ox, oy) - corner
            alpha = max(0.0, min(1.0, 1.0 - sq))

            dist = math.hypot(dx, dy)
            glow = max(0.0, 1.0 - dist / (110.0 * scale))
            color = mix(NAVY, BLUE, 0.18 + 0.35 * glow * glow)

            ring = abs(dist - ring_r)
            if ring < stroke * 1.4:
                t = 1.0 - ring / (stroke * 1.4)
                color = mix(color, RING, t * 0.85)

            outer = abs(hex_sdf(dx, dy, hex_r))
            if outer < stroke:
                t = 1.0 - outer / stroke
                color = mix(color, BLUE, 0.55 + 0.45 * t)

            inner = abs(hex_sdf(dx, dy, inner_r))
            if inner < stroke * 0.85:
                t = 1.0 - inner / (stroke * 0.85)
                color = mix(color, GLOW, 0.65 + 0.35 * t)

            core = hex_sdf(dx, dy, 12.0 * scale)
            if core < 0:
                color = mix(color, GLOW, 0.9)

            # Simple "A" chevron
            axn, ayn = dx / scale, dy / scale
            in_a = False
            if -52 <= ayn <= 48:
                half = (48 - ayn) * 0.62
                if abs(axn) < half and abs(axn) > half - 14:
                    in_a = True
                if 8 <= ayn <= 18 and abs(axn) < half * 0.55:
                    in_a = True
            if in_a:
                color = mix(color, (230, 240, 255), 0.92)

            pixels.append((color[0], color[1], color[2], clamp(alpha * 255)))
    return pixels


def png_bytes(size: int, pixels: list[tuple[int, int, int, int]]) -> bytes:
    raw = bytearray()
    for y in range(size):
        raw.append(0)
        for x in range(size):
            r, g, b, a = pixels[y * size + x]
            raw.extend((r, g, b, a))

    def chunk(tag: bytes, data: bytes) -> bytes:
        crc = zlib.crc32(tag + data) & 0xFFFFFFFF
        return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", crc)

    ihdr = struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(bytes(raw), 9))
        + chunk(b"IEND", b"")
    )


def bmp_dib(size: int, pixels: list[tuple[int, int, int, int]]) -> bytes:
    header = struct.pack(
        "<IIIHHIIIIII",
        40,
        size,
        size * 2,
        1,
        32,
        0,
        size * size * 4,
        0,
        0,
        0,
        0,
    )
    xor = bytearray()
    for y in range(size - 1, -1, -1):
        for x in range(size):
            r, g, b, a = pixels[y * size + x]
            xor.extend((b, g, r, a))
    row_bytes = ((size + 31) // 32) * 4
    mask = bytearray()
    for y in range(size - 1, -1, -1):
        row = bytearray(row_bytes)
        for x in range(size):
            if pixels[y * size + x][3] < 128:
                row[x // 8] |= 0x80 >> (x % 8)
        mask.extend(row)
    return header + xor + mask


def write_ico(path: Path) -> None:
    entries: list[tuple[int, bytes]] = []
    for size in (16, 24, 32, 48, 64):
        entries.append((size, bmp_dib(size, render(size))))
    entries.append((256, png_bytes(256, render(256))))

    count = len(entries)
    offset = 6 + 16 * count
    out = bytearray(struct.pack("<HHH", 0, 1, count))
    blobs = bytearray()
    for size, data in entries:
        w = 0 if size >= 256 else size
        h = 0 if size >= 256 else size
        out.extend(
            struct.pack(
                "<BBBBHHII",
                w,
                h,
                0,
                0,
                1,
                32,
                len(data),
                offset,
            )
        )
        blobs.extend(data)
        offset += len(data)
    path.write_bytes(bytes(out) + bytes(blobs))
    print(f"Wrote {path} ({path.stat().st_size} bytes, {count} images)")


if __name__ == "__main__":
    write_ico(OUT)
